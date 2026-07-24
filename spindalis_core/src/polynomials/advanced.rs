use crate::polynomials::PolynomialError;
use crate::polynomials::structs::advanced::{Expr, Polynomial, TokenStream, Visitor};
use std::collections::{BTreeSet, HashMap};
use std::f64;
use std::str::FromStr;
use std::sync::LazyLock;

/*
* ##### TOKEN_FROM_STR MACRO: USAGE EXAMPLE ####
*
* ### INPUT:
*
* token_from_str! {
* #[derive(Foo,Bar)] //
* #[derive(This,Too)] //
* pub SomeEnum { // visibility specification is OPTIONAL. `SomeEnum{...}` would also work
*  Var1 => "var1str", // NOTE1: strings must be in lowercase
*  Var2 => "var2str", // NOTE2: strings must not be duplicated
*  Var3 => "var3str" -> "e", // NOTE3: optional display char after "->"
*  ..., // <- may or may not end with a trailing comma
* }
*
* ### OUTPUT GENERATED:
*
* // 1. `enum` declaration (with optionally specified visibility)
*
* #[derive(Foo,Bar)]
* #[derive(This,Too)]
* pub enum SomeEnum {
*  Var1,
*  Var2,
*  ...
* }
*
* // 2. `FromStr` and string matching logic
* impl FromStr for SomeEnum{
*    type Err = ();
*    fn from_str(s:&str)->Result<Self,Self::Err>{
*        match s.to_lowercase().as_str(){
*            var1str => Ok(SomeEnum::Var1),
*            var2str => Ok(SomeEnum::Var2),
*            ...
*            _ => Err(()),
*        }
*    }
* }
*
* // 3. Display implementation
* impl Display for SomeEnum{
*    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
*        let s: &str = match self {
*            Self::... => ...,
*            ...
*        };
*        write!(f, "{s}")
*    }
* }
*/

macro_rules! __display_str {
    ($parse:literal => $display:literal) => {
        $display
    };
    ($parse:literal) => {
        $parse
    };
}

macro_rules! token_from_str {
    (
        //INPUT
        $(#[$meta_exp:meta])*
        $visb:vis $enum_name:ident {
            $(
                // variants & their `str` values e.g. `Tau => "tau" -> "τ"`
                $var_name:ident => $var_str:literal
                // optional `-> "τ"`
                $(-> $display_str:literal)?
            ),+ $(,)?
        }
    ) => {
        // OUTPUT
        // 1. `enum` declaration
        $(#[$meta_exp])*
        $visb enum $enum_name {
            $($var_name),*
        }

        // 2. `FromStr` implementation
        impl ::std::str::FromStr for $enum_name {
            type Err = ();

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s.to_lowercase().as_str() {
                    $(
                        $var_str => Ok(Self::$var_name),
                        $(
                            $display_str => Ok(Self::$var_name),
                        )?
                    )*
                    _ => Err(()),
                }
            }
        }

        // 3. Display implementation
        impl ::std::fmt::Display for $enum_name {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result {
                let s = match self {
                    $(
                        Self::$var_name => __display_str!($var_str $(=> $display_str)?),
                    )*

                };
                write!(f, "{s}")
            }
        }
    };
}

// `token_from_char` is similar to `token_from_str`
macro_rules! token_from_char {
    (
    // INPUT
    // attribute(s) (optional) e.g. `#[Derive(Foo,Bar)]`
        $(#[$attr:meta])*
        // visibility and enum name e.g. `pub SomeEnum {...}`
        $visb:vis $enum_name:ident {
            $(
                // variants & their `char` values e.g. `Add => +`
                $var_name:ident => $var_char:literal
                // optional display char
                $(-> $display_str:literal)?
            ),* $(,)?
        }
    ) => {
        // OUTPUT
        // 1. `enum` declaration
        $(#[$attr])*
        $visb enum $enum_name {
            $($var_name),*
        }
        // 2. `fn from_char` with matching rules
        impl $enum_name {
            #[allow(clippy::result_unit_err)]
            pub fn from_char(c: char) -> ::std::result::Result<Self, ()> {
                match c {
                    $(
                        $var_char => Ok(Self::$var_name),
                        $(
                            $display_str => Ok(Self::$var_name),
                        )?
                    )*
                    _ => Err(()),
                }
            }
        }

        // 3. Display implementation
        impl ::std::fmt::Display for $enum_name {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result {
                let s = match self {
                    $(
                        Self::$var_name => __display_str!($var_char $(=> $display_str)?),
                    )*
                };
                write!(f, "{s}")
            }
        }
    };
}

// declaring `Operators` with `token_from_char`
token_from_char! {
    #[derive(Debug, PartialEq,Eq,Copy,Clone,Hash)]
    pub Operators {
        Add     => '+',
        Sub     => '-',
        Div     => '/',
        Mul     => '*',
        CDot    => '·',
        Rem     => '%',
        Caret   => '^',
        Fac     => '!',
        TempAdd => '#',
        TempSub => '@',
        TempMul => '&',
        TempDiv => '$',
    }
}

// declaring `Functions` with `token_from_str`
token_from_str! {
    #[derive(Debug, PartialEq,Clone,Copy)]
    pub Functions {
        Sin => "sin",
        Cos => "cos",
        Tan => "tan",
        Cot => "cot",
        Sec => "sec",
        Csc => "csc",
        Log => "log",
        Ln => "ln",
        Sqrt => "sqrt"
    }
}

// declaring `Constants` with `token_from_str`
token_from_str! {
    #[derive(Debug, PartialEq,Clone,Copy,)]
    pub Constants {
        Pi => "pi" -> "π",
        E => "e",
        Tau => "tau" -> "τ",
        Phi => "phi"-> "ϕ",
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Variable(String),
    Operator(Operators),
    Function(Functions),
    Constant(Constants),
    LParen,
    RParen,
}

pub fn lexer<S>(input: S) -> Result<Vec<Token>, PolynomialError>
where
    S: AsRef<str>,
{
    let input = input.as_ref();
    let mut tokens: Vec<Token> = Vec::new();
    let mut temp = String::new();
    let chars = input.replace(' ', "");

    let mut chars = chars.chars().peekable();
    while let Some(&ch) = chars.peek() {
        temp.clear();
        match ch {
            '0'..='9' | '.' => {
                while let Some(&next_char) = chars.peek() {
                    if next_char.is_ascii_digit() || next_char == '.' {
                        temp.push(next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match temp.parse::<f64>() {
                    Ok(x) => tokens.push(Token::Number(x)),
                    Err(_) => return Err(PolynomialError::InvalidNumber { num: temp }),
                }
            }
            'a'..='z' | 'A'..='Z' => {
                while let Some(&next_char) = chars.peek() {
                    if next_char.is_ascii_alphabetic() {
                        temp.push(next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match temp.len() {
                    1 => {
                        if let Ok(x) = Constants::from_str(&temp) {
                            tokens.push(Token::Constant(x));
                        } else {
                            tokens.push(Token::Variable(temp.clone()));
                        }
                    }
                    _ => {
                        if let Ok(x) = Functions::from_str(&temp) {
                            tokens.push(Token::Function(x));
                        } else if let Ok(x) = Constants::from_str(&temp) {
                            tokens.push(Token::Constant(x));
                        } else {
                            for char in temp.chars() {
                                let char = char.to_string();
                                if let Ok(x) = Constants::from_str(&char) {
                                    tokens.push(Token::Constant(x));
                                } else {
                                    tokens.push(Token::Variable(char));
                                }
                            }
                        }
                    }
                }
            }

            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }

            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }

            _ => {
                let mut buf = [0; 4];
                let ch_str = ch.encode_utf8(&mut buf);
                if let Ok(op) = Operators::from_char(ch) {
                    tokens.push(Token::Operator(op));
                    chars.next();
                } else if let Ok(c) = Constants::from_str(ch_str) {
                    tokens.push(Token::Constant(c));
                    chars.next();
                } else {
                    return Err(PolynomialError::UnexpectedChar { char: ch });
                }
            }
        }
    }

    Ok(tokens)
}

static BINDING_POW: LazyLock<HashMap<Operators, f64>> = LazyLock::new(|| {
    HashMap::from([
        (Operators::Sub, 1.0),
        (Operators::Add, 1.0),
        (Operators::Mul, 2.0),
        (Operators::Div, 2.0),
        (Operators::Rem, 3.0),
        (Operators::CDot, 4.0),
        (Operators::Caret, 5.0),
    ])
});

fn ensure(token_stream: &mut TokenStream, expected_token: &Token) -> Result<(), PolynomialError> {
    match token_stream.next() {
        Some(token) if token == *expected_token => Ok(()),
        Some(token) => Err(PolynomialError::UnexpectedToken { token }),
        None => Err(PolynomialError::UnexpectedEndOfTokens),
    }
}

fn implied_multiplication_pass(token_stream: &mut Vec<Token>) {
    // Pass to identify implied multiplication to insert Operators::Mul
    // e.g. 4x = (4·x)
    // 4x^2 == (4·x^2)
    // The CDot Operator is a special temporary token with a unique binding power
    // to ensure that 4x^2 is treated as a single unit.
    // This is later replaced with the Mul operator in the parse_advanced_polynomial function.
    let mut idx = 0;
    loop {
        if idx >= token_stream.len() {
            break;
        }
        match token_stream.get(idx) {
            Some(Token::Number(_)) => {
                if let Some(
                    Token::Variable(_) | Token::Function(_) | Token::Constant(_) | Token::LParen,
                ) = token_stream.get(idx + 1)
                {
                    token_stream.insert(idx + 1, Token::Operator(Operators::CDot));
                }
            }
            Some(Token::Variable(_) | Token::Constant(_)) => {
                if let Some(
                    Token::Number(_)
                    | Token::Variable(_)
                    | Token::Function(_)
                    | Token::Constant(_)
                    | Token::LParen,
                ) = token_stream.get(idx + 1)
                {
                    token_stream.insert(idx + 1, Token::Operator(Operators::CDot));
                }
            }
            _ => {}
        }
        idx += 1;
    }
}

fn parse_expr(token_stream: &mut TokenStream, min_bind_pow: f64) -> Result<Expr, PolynomialError> {
    let mut left = match token_stream.next() {
        Some(Token::Number(n)) => Ok(Expr::Number(n)),
        Some(Token::Variable(n)) => Ok(Expr::Variable(n)),
        Some(Token::Constant(n)) => Ok(Expr::Constant(n)),
        Some(Token::LParen) => {
            let mut expr = parse_expr(token_stream, 0.0)?;
            ensure(token_stream, &Token::RParen)?;
            if let Expr::BinaryOp { ref mut paren, .. } = expr {
                *paren = true;
            }
            Ok(expr)
        }
        Some(Token::RParen) => {
            return Err(PolynomialError::UnexpectedToken {
                token: Token::RParen,
            });
        }
        Some(Token::Function(func)) => {
            // Functions should always be followed by parentheses
            if token_stream.peek() != Some(&Token::LParen) {
                match token_stream.next() {
                    Some(t) => {
                        return Err(PolynomialError::UnexpectedToken { token: t });
                    }
                    None => return Err(PolynomialError::UnexpectedEndOfTokens),
                }
            }
            token_stream.next();
            let inner = parse_expr(token_stream, 0.0)?;
            ensure(token_stream, &Token::RParen)?;

            Ok(Expr::Function {
                func,
                inner: Box::new(inner),
            })
        }
        Some(Token::Operator(operator)) => {
            if operator != Operators::Sub {
                return Err(PolynomialError::UnexpectedToken {
                    token: Token::Operator(operator),
                });
            }
            let next_expr = parse_expr(token_stream, 2.0)?;
            Ok(Expr::UnaryOpPrefix {
                op: operator,
                value: Box::new(next_expr),
            })
        }
        _ => return Err(PolynomialError::PolynomialSyntaxError),
    }?;

    // Handles factorial (as well as stacked factorials eg. 3!!!)
    while matches!(token_stream.peek(), Some(Token::Operator(op)) if *op == Operators::Fac) {
        token_stream.next();
        left = Expr::UnaryOpPostfix {
            op: Operators::Fac,
            value: Box::new(left),
        };
    }
    // Handles parentheses right after factorial using implicit multiplication
    if matches!(token_stream.peek(), Some(Token::LParen)) {
        left = Expr::BinaryOp {
            op: Operators::Mul,
            lhs: Box::new(left),
            rhs: Box::new(parse_expr(token_stream, 0.0)?),
            paren: true,
        }
    }

    // Iteratively looks for operators with lower binding than minimum binding power
    while let Some(Token::Operator(op)) = token_stream.peek() {
        let cbind_pow = *BINDING_POW.get(op).unwrap_or(&0.0);
        if cbind_pow < min_bind_pow {
            break;
        }

        let mut op = *op;
        if op == Operators::CDot {
            op = Operators::Mul;
        };
        token_stream.next();
        // right associativity of operators
        let right = parse_expr(token_stream, cbind_pow + 1.0)?;
        left = Expr::BinaryOp {
            op,
            lhs: Box::new(left),
            rhs: Box::new(right),
            paren: false,
        };
    }

    Ok(left)
}

pub fn parse_advanced_polynomial(token_stream: Vec<Token>) -> Result<Polynomial, PolynomialError> {
    let mut tokens = token_stream;
    implied_multiplication_pass(&mut tokens);
    let mut token_stream = tokens.into_iter().peekable();

    // Parse valid polynomial until end or invalid token
    let ast_node = parse_expr(&mut token_stream, 0.0)?;

    // Returns error if there are remaining tokens after parsing
    if let Some(token) = token_stream.next() {
        return Err(PolynomialError::UnexpectedToken { token });
    }

    Ok(Polynomial::new(fold_operations(ast_node)))
}

pub(crate) fn fold_operations(expr: Expr) -> Expr {
    match expr {
        Expr::Number(_) | Expr::Variable(_) | Expr::Constant(_) => expr,
        Expr::BinaryOp {
            op,
            lhs,
            rhs,
            paren,
        } => {
            let lhs = fold_operations(*lhs);
            let rhs = fold_operations(*rhs);
            match (op, lhs, rhs) {
                // 0*_ = 0
                (Operators::Mul, Expr::Number(0.), _) => Expr::Number(0.),
                (Operators::Mul, _, Expr::Number(0.)) => Expr::Number(0.),

                // 1*x = x
                (Operators::Mul, Expr::Number(1.), r) => fold_operations(r),
                (Operators::Mul, l, Expr::Number(1.)) => fold_operations(l),

                // _^0 = 1 & 0^_ = 0
                // Note: Above condition includes 0^0
                (Operators::Caret, _, Expr::Number(0.)) => Expr::Number(1.),
                (Operators::Caret, Expr::Number(0.), _) => Expr::Number(0.),

                // x^1 = x
                (Operators::Caret, l, Expr::Number(1.)) => fold_operations(l),

                // x+0 = x
                (Operators::Add, Expr::Number(0.), r) => fold_operations(r),
                (Operators::Add, l, Expr::Number(0.)) => fold_operations(l),

                // 0-x = -x & x-0 = x
                (Operators::Sub, l, Expr::Number(0.)) => fold_operations(l),
                (Operators::Sub, Expr::Number(0.), r) => Expr::UnaryOpPrefix {
                    op: Operators::Sub,
                    value: Box::new(fold_operations(r)),
                },

                // x/1 = x
                (Operators::Div, l, Expr::Number(1.)) => fold_operations(l),

                (
                    Operators::TempAdd
                    | Operators::TempSub
                    | Operators::TempMul
                    | Operators::TempDiv,
                    _,
                    _,
                ) => Expr::Number(0.),

                // 1 / x / y -> y / x
                (
                    Operators::Div,
                    Expr::BinaryOp {
                        op: Operators::Div,
                        lhs: dlhs,
                        rhs: drhs,
                        ..
                    },
                    r,
                ) => {
                    let numerator = Expr::BinaryOp {
                        op: Operators::Mul,
                        lhs: Box::new(*dlhs),
                        rhs: Box::new(r),
                        paren: false,
                    };
                    let folded_numerator = fold_operations(numerator);
                    Expr::BinaryOp {
                        op: Operators::Div,
                        lhs: Box::new(folded_numerator),
                        rhs: Box::new(*drhs),
                        paren: false,
                    }
                }

                // (-3)/2 -> -(3/2), (-3x)*2 -> -(3x*2)
                (
                    Operators::Div | Operators::Mul,
                    Expr::UnaryOpPrefix {
                        op: Operators::Sub,
                        value: val,
                    },
                    r,
                ) => Expr::UnaryOpPrefix {
                    op: Operators::Sub,
                    value: Box::new(fold_operations(Expr::BinaryOp {
                        op,
                        lhs: Box::new(*val),
                        rhs: Box::new(r),
                        paren,
                    })),
                },

                // 3 * 2 => 6, 3 + 4 => 7
                (Operators::Add, Expr::Number(a), Expr::Number(b)) => Expr::Number(a + b),
                (Operators::Sub, Expr::Number(a), Expr::Number(b)) => {
                    let res = a - b;
                    if res < 0_f64 {
                        Expr::UnaryOpPrefix {
                            op: Operators::Sub,
                            value: Box::new(Expr::Number(-res)),
                        }
                    } else {
                        Expr::Number(res)
                    }
                }
                (Operators::Mul, Expr::Number(a), Expr::Number(b)) => {
                    let res = a * b;
                    if res < 0_f64 {
                        Expr::UnaryOpPrefix {
                            op: Operators::Sub,
                            value: Box::new(Expr::Number(-res)),
                        }
                    } else {
                        Expr::Number(res)
                    }
                }
                (Operators::Div, Expr::Number(a), Expr::Number(b)) if b != 0.0 => {
                    let res = a / b;
                    if res < 0_f64 {
                        Expr::UnaryOpPrefix {
                            op: Operators::Sub,
                            value: Box::new(Expr::Number(-res)),
                        }
                    } else {
                        Expr::Number(res)
                    }
                }
                (Operators::Caret, Expr::Number(a), Expr::Number(b)) => Expr::Number(a.powf(b)),

                // c1 * (c2 * x) => (c1 * c2) * x
                (
                    Operators::Mul,
                    Expr::Number(c1),
                    Expr::BinaryOp {
                        op: Operators::Mul,
                        lhs: inner_lhs,
                        rhs: inner_rhs,
                        ..
                    },
                ) => {
                    if let Expr::Number(c2) = *inner_lhs {
                        fold_operations(Expr::BinaryOp {
                            op: Operators::Mul,
                            lhs: Box::new(Expr::Number(c1 * c2)),
                            rhs: inner_rhs,
                            paren,
                        })
                    } else {
                        Expr::BinaryOp {
                            op,
                            lhs: Box::new(Expr::Number(c1)),
                            rhs: Box::new(Expr::BinaryOp {
                                op: Operators::Mul,
                                lhs: inner_lhs,
                                rhs: inner_rhs,
                                paren,
                            }),
                            paren,
                        }
                    }
                }

                // (x * y)^n -> x^n * y^n
                (
                    Operators::Caret,
                    Expr::BinaryOp {
                        op: Operators::Mul,
                        lhs: inner_lhs,
                        rhs: inner_rhs,
                        ..
                    },
                    exp,
                ) => fold_operations(Expr::BinaryOp {
                    op: Operators::Mul,
                    lhs: Box::new(Expr::BinaryOp {
                        op: Operators::Caret,
                        lhs: inner_lhs,
                        rhs: Box::new(exp.clone()),
                        paren: false,
                    }),
                    rhs: Box::new(Expr::BinaryOp {
                        op: Operators::Caret,
                        lhs: inner_rhs,
                        rhs: Box::new(exp),
                        paren: false,
                    }),
                    paren: false,
                }),

                // Variable exponent division: y^a / y^b -> 1, y^k, or 1/y^k
                (Operators::Div, l, r) => {
                    let mut num_factors = Vec::new();
                    let mut den_factors = Vec::new();

                    collect_factors(&l, &mut num_factors);
                    collect_factors(&r, &mut den_factors);

                    // Look for matching base sub-trees
                    let matching_factor = num_factors.iter().find_map(|(base1, p1)| {
                        den_factors
                            .iter()
                            .find(|(base2, _)| base1 == base2) // Deep AST equality
                            .map(|(_, p2)| (base1.clone(), *p1, *p2))
                    });

                    if let Some((base_expr, p1, p2)) = matching_factor {
                        let diff = p1 - p2;

                        if diff == 0.0 {
                            let new_lhs = fold_operations(remove_factor(l, &base_expr));
                            let new_rhs = fold_operations(remove_factor(r, &base_expr));
                            fold_operations(Expr::BinaryOp {
                                op: Operators::Div,
                                lhs: Box::new(new_lhs),
                                rhs: Box::new(new_rhs),
                                paren,
                            })
                        } else if diff > 0.0 {
                            let new_lhs = fold_operations(replace_factor(l, &base_expr, diff));
                            let new_rhs = fold_operations(remove_factor(r, &base_expr));
                            fold_operations(Expr::BinaryOp {
                                op: Operators::Div,
                                lhs: Box::new(new_lhs),
                                rhs: Box::new(new_rhs),
                                paren,
                            })
                        } else {
                            let new_lhs = fold_operations(remove_factor(l, &base_expr));
                            let new_rhs =
                                fold_operations(replace_factor(r, &base_expr, diff.abs()));
                            fold_operations(Expr::BinaryOp {
                                op: Operators::Div,
                                lhs: Box::new(new_lhs),
                                rhs: Box::new(new_rhs),
                                paren,
                            })
                        }
                    } else {
                        Expr::BinaryOp {
                            op: Operators::Div,
                            lhs: Box::new(l),
                            rhs: Box::new(r),
                            paren,
                        }
                    }
                }

                // Non foldable conditions
                (_, l, r) => Expr::BinaryOp {
                    op,
                    lhs: Box::new(l),
                    rhs: Box::new(r),
                    paren,
                },
            }
        }
        expr => expr,
    }
}

fn collect_factors(expr: &Expr, factors: &mut Vec<(Expr, f64)>) {
    match expr {
        // Multiplication: recurse into children
        Expr::BinaryOp {
            op: Operators::Mul,
            lhs,
            rhs,
            ..
        } => {
            collect_factors(lhs, factors);
            collect_factors(rhs, factors);
        }
        // Powers: A^n -> base A with power n
        Expr::BinaryOp {
            op: Operators::Caret,
            lhs,
            rhs,
            ..
        } => {
            if let Expr::Number(n) = **rhs {
                factors.push((*lhs.clone(), n));
            } else {
                factors.push((expr.clone(), 1.0));
            }
        }
        // Base case: Any other expression (Variable, Constant, Add, Sub, etc.) has power 1.0
        other => factors.push((other.clone(), 1.0)),
    }
}

// Removes a factor (e.g. `34 + y` or `(34 + y)^2`), replacing it with 1.0 (which folds away)
fn remove_factor(expr: Expr, target: &Expr) -> Expr {
    if &expr == target {
        return Expr::Number(1.0);
    }

    match expr {
        Expr::BinaryOp {
            op: Operators::Caret,
            lhs,
            ..
        } if &*lhs == target => Expr::Number(1.0),

        Expr::BinaryOp {
            op: Operators::Mul,
            lhs,
            rhs,
            paren,
        } => Expr::BinaryOp {
            op: Operators::Mul,
            lhs: Box::new(remove_factor(*lhs, target)),
            rhs: Box::new(remove_factor(*rhs, target)),
            paren,
        },

        other => other,
    }
}

// Replaces an expression factor's power (e.g. `(34 + y)^2` -> `(34 + y)^1` which is just `(34 + y)`)
fn replace_factor(expr: Expr, target: &Expr, new_pow: f64) -> Expr {
    // Direct match on the target expression itself (eg target is `34 + y`)
    if &expr == target {
        return if new_pow == 1.0 {
            expr
        } else {
            Expr::BinaryOp {
                op: Operators::Caret,
                lhs: Box::new(expr),
                rhs: Box::new(Expr::Number(new_pow)),
                paren: false,
            }
        };
    }

    match expr {
        // Match on a Caret node whose base matches target (eg `(34 + y)^2`)
        Expr::BinaryOp {
            op: Operators::Caret,
            lhs,
            ..
        } if &*lhs == target => {
            if new_pow == 1.0 {
                *lhs
            } else {
                Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs,
                    rhs: Box::new(Expr::Number(new_pow)),
                    paren: false,
                }
            }
        }

        // Recurse down multiplication chains
        Expr::BinaryOp {
            op: Operators::Mul,
            lhs,
            rhs,
            paren,
        } => Expr::BinaryOp {
            op: Operators::Mul,
            lhs: Box::new(replace_factor(*lhs, target, new_pow)),
            rhs: Box::new(replace_factor(*rhs, target, new_pow)),
            paren,
        },

        // Base case for non-matching nodes
        other => other,
    }
}

impl From<f64> for Expr {
    fn from(v: f64) -> Self {
        Expr::Number(v)
    }
}
impl From<&'static str> for Expr {
    fn from(v: &'static str) -> Self {
        Expr::Variable(v.into())
    }
}

pub fn eval_advanced_polynomial<V, S, F>(
    poly: &Polynomial,
    variables: &V,
) -> Result<f64, PolynomialError>
where
    V: IntoIterator<Item = (S, F)> + std::fmt::Debug + Clone,
    S: AsRef<str>,
    F: Into<f64>,
{
    let vars_map: HashMap<String, f64> = variables
        .clone()
        .into_iter()
        .map(|(k, v)| (k.as_ref().to_string(), v.into()))
        .collect();
    let literal_expr = replace_variable_occurence(&poly.expr, &vars_map)?;

    let eval = Evaluator;
    let Some(result) = literal_expr.accept(&eval) else {
        return Err(PolynomialError::MissingVariable);
    };
    Ok(result)
    // TODO: Work on error messages
}

fn replace_variable_occurence(
    expr: &Expr,
    vars: &HashMap<String, f64>,
) -> Result<Expr, PolynomialError> {
    expr.map(&mut |e| match e {
        Expr::Variable(v) => {
            vars.get(v)
                .copied()
                .map(Expr::Number)
                .ok_or(PolynomialError::VariableNotFound {
                    variable: v.to_string(),
                })
        }
        x => Ok(x.clone()),
    })
}

pub fn extract_univariate_variable(expr: &Expr) -> Result<String, PolynomialError> {
    let mut variables: BTreeSet<String> = Default::default();
    expr.visit(&mut |e| {
        if let Expr::Variable(v) = e {
            variables.insert(v.to_string());
        }
    });
    if variables.len() > 1 {
        Err(PolynomialError::TooManyVariables {
            variables: variables.into_iter().collect::<Vec<_>>(),
        })
    } else {
        variables
            .into_iter()
            .next()
            .map_or_else(|| Err(PolynomialError::MissingVariable), Ok)
    }
}

fn factorial_f64(n: f64) -> f64 {
    if n < 0.0 {
        return f64::NAN;
    }
    let n_int = n.floor() as u64;
    (1..=n_int).fold(1.0, |acc, x| acc * x as f64)
}
pub(crate) struct Evaluator;

impl Visitor for Evaluator {
    type Output = Option<f64>;

    fn visit_number(&self, value: f64) -> Option<f64> {
        Some(value)
    }

    fn visit_variable(&self, _: &str) -> Option<f64> {
        unreachable!()
    }

    fn visit_binary_op(&self, op: &Operators, lhs: &Expr, rhs: &Expr, _: bool) -> Option<f64> {
        let lhs = lhs.accept(self)?;
        let rhs = rhs.accept(self)?;

        let ans = match op {
            Operators::Div => Some(lhs / rhs),
            Operators::Mul | Operators::CDot => Some(lhs * rhs),
            Operators::Add => Some(lhs + rhs),
            Operators::Sub => Some(lhs - rhs),
            Operators::Rem => Some(lhs % rhs),
            Operators::Caret => Some(lhs.powf(rhs)),
            _ => None,
        };
        if let Some(res) = ans {
            return Some(res);
        };
        None
    }

    fn visit_unary_postfix(&self, op: &Operators, value: &Expr) -> Option<f64> {
        match op {
            Operators::Fac => Some(factorial_f64(value.accept(self)?)),
            _ => None,
        }
    }

    fn visit_unary_prefix(&self, op: &Operators, value: &Expr) -> Option<f64> {
        if let Some(value) = value.accept(self) {
            let new_value = match op {
                Operators::Add => Some(value),
                Operators::Sub => Some(-value),
                _ => None,
            };
            if let Some(res) = new_value {
                return Some(res);
            }
        }
        None
    }
    fn visit_function(&self, func: &Functions, value: &Expr) -> Option<f64> {
        if let Some(value) = value.accept(self) {
            let res = match func {
                Functions::Sin => value.sin(),
                Functions::Cos => value.cos(),
                Functions::Tan => value.tan(),
                Functions::Cot => 1.0 / value.tan(),
                Functions::Sec => 1.0 / value.cos(),
                Functions::Csc => 1.0 / value.sin(),
                Functions::Ln | Functions::Log => value.ln(),
                Functions::Sqrt => value.sqrt(),
            };
            return Some(res);
        }
        None
    }

    fn visit_constant(&self, cnst: &Constants) -> Option<f64> {
        let constant = match cnst {
            Constants::Pi => f64::consts::PI,
            Constants::E => f64::consts::E,
            Constants::Tau => f64::consts::TAU,
            Constants::Phi => 1.618_033_988_749_895_f64, // f64::consts::PHI
        };
        Some(constant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------
    // Tokenizing tests
    // ---------------------------
    mod lexer_tests {
        use super::*;

        #[test]
        fn test_number_token() {
            let expr = "32";
            let result = lexer(expr).unwrap();
            let expected = Token::Number(32.0);

            assert_eq!(result[0], expected);
        }

        #[test]
        fn test_decimal_number_token() {
            let expr = "32.0";
            let result = lexer(expr).unwrap();
            let expected = Token::Number(32.0);

            assert_eq!(result[0], expected);
        }

        #[test]
        fn test_float_token() {
            let expr = "0.32";
            let result = lexer(expr).unwrap();
            let expected = Token::Number(0.32);

            assert_eq!(result[0], expected);
        }

        #[test]
        fn test_decimal_only_float() {
            let expr = ".32";
            let result = lexer(expr).unwrap();
            let expected = Token::Number(0.32);

            assert_eq!(result[0], expected);
        }

        #[test]
        fn test_variable_only() {
            let expr = "x";
            let result = lexer(expr).unwrap();
            let expected = Token::Variable("x".to_string());

            assert_eq!(result[0], expected);
        }

        #[test]
        fn test_num_and_variable() {
            let expr = "32x";
            let result = lexer(expr).unwrap();
            let expected = [Token::Number(32.0), Token::Variable("x".to_string())];

            for i in 0..expected.len() {
                assert_eq!(result[i], expected[i]);
            }
        }

        #[test]
        fn test_expression() {
            let expr = "3x^2 + 1.43";
            let result = lexer(expr).unwrap();
            let expected = [
                Token::Number(3.0),
                Token::Variable("x".to_string()),
                Token::Operator(Operators::Caret),
                Token::Number(2.0),
                Token::Operator(Operators::Add),
                Token::Number(1.43),
            ];

            for i in 0..expected.len() {
                assert_eq!(result[i], expected[i]);
            }
        }

        #[test]
        fn test_expression_with_constant() {
            let expr = "3x^2 + 1.43xy - pi^3";
            let result = lexer(expr).unwrap();
            let expected = [
                Token::Number(3.0),
                Token::Variable("x".to_string()),
                Token::Operator(Operators::Caret),
                Token::Number(2.0),
                Token::Operator(Operators::Add),
                Token::Number(1.43),
                Token::Variable("x".to_string()),
                Token::Variable("y".to_string()),
                Token::Operator(Operators::Sub),
                Token::Constant(Constants::Pi),
                Token::Operator(Operators::Caret),
                Token::Number(3.0),
            ];

            for i in 0..expected.len() {
                assert_eq!(result[i], expected[i]);
            }
        }

        #[test]
        fn test_tokenizing_parens() {
            let expr = "(3*2) / 4";
            let result = lexer(expr).unwrap();
            let expected = [
                Token::LParen,
                Token::Number(3.0),
                Token::Operator(Operators::Mul),
                Token::Number(2.0),
                Token::RParen,
                Token::Operator(Operators::Div),
                Token::Number(4.0),
            ];

            for i in 0..expected.len() {
                assert_eq!(result[i], expected[i]);
            }
        }
    }

    // ---------------------------
    // Parsing tests
    // ---------------------------
    mod parse_advanced_polynomial_tests {
        use super::*;

        #[test]
        fn test_number_parse() {
            let expr = "4";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Number(4.0));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_variable_parse() {
            let expr = "x";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Variable("x".into()));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_phi_parse() {
            let expr = "phi";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Constant(Constants::Phi));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_phi_parse_2() {
            let expr = "ϕ";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Constant(Constants::Phi));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_pi_parse() {
            let expr = "pi";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Constant(Constants::Pi));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_pi_parse_2() {
            let expr = "π";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Constant(Constants::Pi));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_exponents_parse() {
            let expr = "x^2";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Caret,
                lhs: Box::new(Expr::Variable("x".into())),
                rhs: Box::new(Expr::Number(2.0)),
                paren: false,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_integer_coefficient_parse() {
            let expr = "4x";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(4.0)),
                rhs: Box::new(Expr::Variable("x".into())),
                paren: false,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_float_coefficient_parse() {
            let expr = "4.2x";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(4.2)),
                rhs: Box::new(Expr::Variable("x".into())),
                paren: false,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_basic_expression_parse() {
            let expr = "4x+2";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Add,
                lhs: Box::new(Expr::BinaryOp {
                    op: Operators::Mul,
                    lhs: Box::new(Expr::Number(4.0)),
                    rhs: Box::new(Expr::Variable("x".into())),
                    paren: false,
                }),
                rhs: Box::new(Expr::Number(2.0)),
                paren: false,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_function_parsing() {
            let expr = "sin(4)";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Function {
                func: Functions::Sin,
                inner: Box::new(Expr::Number(4.0)),
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_int_float_expression_parse() {
            let expr = "4x^2 + 2.3x^3";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();

            let l_child = Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(4.0)),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs: Box::new(Expr::Variable("x".into())),
                    rhs: Box::new(Expr::Number(2.0)),
                    paren: false,
                }),
                paren: false,
            };

            let r_child = Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(2.3)),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs: Box::new(Expr::Variable("x".into())),
                    rhs: Box::new(Expr::Number(3.0)),
                    paren: false,
                }),
                paren: false,
            };

            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Add,
                lhs: Box::new(l_child),
                rhs: Box::new(r_child),
                paren: false,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_complex_expression_parse() {
            let expr = "4x + 2 - 5x^2 * 4y^4 / 6z^6";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();

            /* --- Left side of subtraction: (4 * x) + 2 --- */
            let fmul_left = Expr::BinaryOp {
                op: Operators::Add,
                lhs: Box::new(Expr::BinaryOp {
                    op: Operators::Mul,
                    lhs: Box::new(Expr::Number(4.0)),
                    rhs: Box::new(Expr::Variable("x".into())),
                    paren: false,
                }),
                rhs: Box::new(Expr::Number(2.0)),
                paren: false,
            };

            /* --- Right side of subtraction: ((5 * x^2) * (4 * x^4)) / (6 * x^6) --- */
            // 5 * x^2
            let term_5x2 = Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(5.0)),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs: Box::new(Expr::Variable("x".into())),
                    rhs: Box::new(Expr::Number(2.0)),
                    paren: false,
                }),
                paren: false,
            };

            // (4 * y^4)
            let term_4y4 = Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(4.0)),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs: Box::new(Expr::Variable("y".into())),
                    rhs: Box::new(Expr::Number(4.0)),
                    paren: false,
                }),
                paren: false,
            };

            // (5 * x^2) * (4 * x^4)
            let numerator = Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(term_5x2),
                rhs: Box::new(term_4y4),
                paren: false,
            };

            // 6 * z^6
            let denominator = Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(6.0)),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs: Box::new(Expr::Variable("z".into())),
                    rhs: Box::new(Expr::Number(6.0)),
                    paren: false,
                }),
                paren: false,
            };

            // ((5 * x^2) * (4 * y^4)) / (6 * z^6)
            let fmul_right = Expr::BinaryOp {
                op: Operators::Div,
                lhs: Box::new(numerator),
                rhs: Box::new(denominator),
                paren: false,
            };

            // ((4 * x) + 2) - ((5 * x^2) * (4 * y^4)) / (6 * z^6)
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Sub,
                lhs: Box::new(fmul_left),
                rhs: Box::new(fmul_right),
                paren: false,
            });

            assert_eq!(result, expected);
        }

        #[test]
        fn test_zero_x_parse() {
            let expr = "0x";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Number(0.));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_zero_parse() {
            let expr = "0";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Number(0.0));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_multivariate_expression_parse() {
            let expr = "4xy + 4x^2 - 2y + 4";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();

            // 4xy = (4 * x) * y
            let term_4xy = Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::BinaryOp {
                    op: Operators::Mul,
                    lhs: Box::new(Expr::Number(4.0)),
                    rhs: Box::new(Expr::Variable("x".into())),
                    paren: false,
                }),
                rhs: Box::new(Expr::Variable("y".into())),
                paren: false,
            };

            // 4x^2 = 4 * (x^2)
            let term_4x2 = Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(4.0)),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs: Box::new(Expr::Variable("x".into())),
                    rhs: Box::new(Expr::Number(2.0)),
                    paren: false,
                }),
                paren: false,
            };

            // 2y
            let term_2y = Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(2.0)),
                rhs: Box::new(Expr::Variable("y".into())),
                paren: false,
            };

            // 4xy + 4x^2
            let term_lleft = Expr::BinaryOp {
                op: Operators::Add,
                lhs: Box::new(term_4xy),
                rhs: Box::new(term_4x2),
                paren: false,
            };

            // (4xy + 4x^2) - 2y
            let term_left = Expr::BinaryOp {
                op: Operators::Sub,
                lhs: Box::new(term_lleft),
                rhs: Box::new(term_2y),
                paren: false,
            };

            // ((4xy + 4x^2) - 2y) + 4
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Add,
                lhs: Box::new(term_left),
                rhs: Box::new(Expr::Number(4.0)),
                paren: false,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_valid_multiple_exponents() {
            let expr = "4x^2^3";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(4.0)),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs: Box::new(Expr::BinaryOp {
                        op: Operators::Caret,
                        lhs: Box::new(Expr::Variable("x".into())),
                        rhs: Box::new(Expr::Number(2.0)),
                        paren: false,
                    }),
                    rhs: Box::new(Expr::Number(3.0)),
                    paren: false,
                }),
                paren: false,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_parsing_parentheses() {
            let expr = "(3+2) / 4";
            let tkn_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tkn_str).unwrap();
            let expected = Polynomial::new(Expr::Number(1.25));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_parsing_no_parents() {
            let expr = "3 + 2 / 4";
            let tkn_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tkn_str).unwrap();
            let expected = Polynomial::new(Expr::Number(3.5));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_nested_parens() {
            let expr = "(3 - (4-2)) * 5x";
            let tkn_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tkn_str).unwrap();
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(5.)),
                rhs: Box::new(Expr::Variable("x".into())),
                paren: false,
            });
            // with paren -> 5x, without parent -> 15x
            assert_eq!(result, expected);
        }

        #[test]
        fn test_unary_prefix() {
            let expr = "-3x + 4";
            let tkn_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tkn_str).unwrap();

            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Add,
                lhs: Box::new(Expr::UnaryOpPrefix {
                    op: Operators::Sub,
                    value: Box::new(Expr::BinaryOp {
                        op: Operators::Mul,
                        lhs: Box::new(Expr::Number(3.0)),
                        rhs: Box::new(Expr::Variable("x".into())),
                        paren: false,
                    }),
                }),
                rhs: Box::new(Expr::Number(4.0)),
                paren: false,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_postfix_unary_simple() {
            let expr = "4!";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::UnaryOpPostfix {
                op: Operators::Fac,
                value: Box::new(Expr::Number(4.0)),
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_postfix_unary_variable() {
            let expr = "x!";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::UnaryOpPostfix {
                op: Operators::Fac,
                value: Box::new(Expr::Variable("x".into())),
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_postfix_unary_chained() {
            let expr = "4!!";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::UnaryOpPostfix {
                op: Operators::Fac,
                value: Box::new(Expr::UnaryOpPostfix {
                    op: Operators::Fac,
                    value: Box::new(Expr::Number(4.0)),
                }),
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_postfix_unary_with_binary() {
            let expr = "3! + 2";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Add,
                lhs: Box::new(Expr::UnaryOpPostfix {
                    op: Operators::Fac,
                    value: Box::new(Expr::Number(3.0)),
                }),
                rhs: Box::new(Expr::Number(2.0)),
                paren: false,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_postfix_unary_with_parens() {
            let expr = "(2+1)!";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::UnaryOpPostfix {
                op: Operators::Fac,
                value: Box::new(Expr::BinaryOp {
                    op: Operators::Add,
                    lhs: Box::new(Expr::Number(2.0)),
                    rhs: Box::new(Expr::Number(1.0)),
                    paren: true,
                }),
            });
            assert_eq!(result, expected);
        }

        // Error handling tests
        #[test]
        fn test_invalid_expression() {
            let expr = "4 +++ 3x";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_missing_right_hand() {
            let expr = "4x +";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_missing_left_hand() {
            let expr = "+ 3x";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_only_operator() {
            let expr = "+";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_invalid_multiple_exponents() {
            let expr = "4x^^^2";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_missing_closing_paren() {
            let expr = "(4x + 2 / 4";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_missing_opening_paren() {
            let expr = "4x + 2) / 4";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_missing_opening_paren_end_with_closing_paren() {
            let expr = "4x + 2)";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_missing_func_paren() {
            let expr = "sin 4";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_missing_func_closing_paren() {
            let expr = "sin(4";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_only_minus_is_allowed() {
            let expr = "+3";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_invalid_operator_mul() {
            let expr = "*3";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_invalid_operator_div() {
            let expr = "/3";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_invalid_operator_caret() {
            let expr = "^3";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_invalid_operator_rem() {
            let expr = "%3";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_missing_rhs() {
            let expr = "-";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_cannot_be_followed_by_rparen() {
            let expr = "-)";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_cannot_be_followed_by_binary_operator() {
            let expr = "-+3";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_unary_cannot_start_expression() {
            let expr = "!";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_unary_cannot_follow_binary_operator() {
            let expr = "3+!2";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_unary_cannot_follow_lparen() {
            let expr = "(!2)";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_unary_cannot_follow_prefix_minus() {
            let expr = "-!3";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_unary_cannot_apply_to_empty_parens() {
            let expr = "()!";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_unary_extra_token_after_factorial() {
            let expr = "3!x";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_unary_invalid_following_unclosed_lparen() {
            let expr = "3!(";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_followed_by_paren() {
            let expr = "3!(2)";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::UnaryOpPostfix {
                    op: Operators::Fac,
                    value: Box::new(Expr::Number(3.0)),
                }),
                rhs: Box::new(Expr::Number(2.0)),
                paren: true,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_unary_prefix_inside_parens_missing_rhs() {
            let expr = "(-)";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_after_binary_operator_missing_rhs() {
            let expr = "3*-";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str);
            assert!(result.is_err());
        }
        #[test]
        fn test_folding_operations() {
            let expr = "4x+2^0-0x^3";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Add,
                lhs: Box::new(Expr::BinaryOp {
                    op: Operators::Mul,
                    lhs: Box::new(Expr::Number(4.)),
                    rhs: Box::new("x".into()),
                    paren: false,
                }),
                rhs: Box::new(Expr::Number(1.)),
                paren: false,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_folding_operations_2() {
            let expr = "0^0 + 5 / 1"; // 1 + 5 = 6
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Number(6.));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_folding_operations_3() {
            let expr = "0^5 + 5 / 1";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Number(5.));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_folding_operations_4() {
            let expr = "(0x^0 + 5) + 5 / 1";
            // (0(1) + 5) + 5 -> 5 + 5 = 10
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Number(10.));

            assert_eq!(result, expected);
        }

        #[test]
        fn test_folding_operations_5() {
            let expr = "(0x^0 * 32) * 5 / 1";
            let tok_str = lexer(expr).unwrap();
            let result = parse_advanced_polynomial(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Number(0.));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_univariance_of_an_expression() {
            let expr = "x^3-3x+5!";
            let tok_str = lexer(expr).unwrap();
            let parsed_result = parse_advanced_polynomial(tok_str).unwrap();
            let evaluated_result = extract_univariate_variable(&parsed_result.expr).unwrap();
            assert_eq!(evaluated_result, String::from("x"));
        }

        #[test]
        fn test_too_many_variables_expression() {
            let expr = "x^3-3y+5!";
            let tok_str = lexer(expr).unwrap();
            let parsed_result = parse_advanced_polynomial(tok_str).unwrap();
            let evaluated_result = extract_univariate_variable(&parsed_result.expr);
            assert!(evaluated_result.is_err());
        }
    }
    // ---------------------------
    // Test Evaluation
    // ---------------------------
    mod test_eval {
        use super::*;

        #[test]
        fn test_postfix_followed_by_paren_eval() {
            let expr = "3!(2x)";
            let poly = Polynomial::parse(expr).unwrap();
            let evaluated_result = poly.eval_univariate(2).unwrap();
            assert_eq!(evaluated_result, 24.);
        }

        #[test]
        fn test_failing_substitution_expression() {
            let expr = "x^3-3xy+5";
            let tok_str = lexer(expr).unwrap();
            let parsed_result = parse_advanced_polynomial(tok_str).unwrap();
            let evaluated_result = eval_advanced_polynomial(&parsed_result, &[("x", 5)]);
            assert!(evaluated_result.is_err());
        }

        #[test]
        fn test_eval_expression() {
            let expr = "x^3-3xy+5!";
            let tok_str = lexer(expr).unwrap();
            let parsed_result = parse_advanced_polynomial(tok_str).unwrap();
            let evaluated_result =
                eval_advanced_polynomial(&parsed_result, &[("x", 5), ("y", 5)]).unwrap();
            assert_eq!(evaluated_result, 170.0);
        }

        #[test]
        fn test_univariant_evaluation() {
            let expr = "z^3-3z+5!";
            let poly = Polynomial::parse(expr).unwrap();
            let evaluated_result = poly.eval_univariate(10).unwrap();
            assert_eq!(evaluated_result, 1090.);
        }

        #[test]
        fn test_univariant_failure_evaluation() {
            let expr = "-3x+5y!";
            let poly = Polynomial::parse(expr).unwrap();
            let evaluated_result = poly.eval_univariate(1);
            assert!(evaluated_result.is_err());
        }

        #[test]
        fn test_univariant_no_variables_evaluation() {
            let expr = "-3+5!";
            let poly = Polynomial::parse(expr).unwrap();
            let evaluated_result = poly.eval_univariate(1).unwrap();
            assert_eq!(evaluated_result, 117.0);
        }

        #[test]
        fn test_multivariant_missing_mapping_expression() {
            let expr = "2x+3y!";
            let poly = Polynomial::parse(expr).unwrap();
            let evaluated_result = poly.eval_multivariate(&[("x", 2)]);
            assert!(evaluated_result.is_err());
        }

        #[test]
        fn test_multivariant_expression() {
            let expr = "2x+3y!";
            let poly = Polynomial::parse(expr).unwrap();
            let evaluated_result = poly.eval_multivariate(&[("x", 2), ("y", 1)]).unwrap();
            assert_eq!(evaluated_result, 7.0);
        }

        #[test]
        fn test_function_eval() {
            let expr = "sin(3x)";
            let poly = Polynomial::parse(expr).unwrap();
            let evaluated_result = poly.eval_multivariate(&[("x", 30)]).unwrap();

            let decimals = 6;
            let rounded =
                (evaluated_result * 10_f64.powi(decimals)).round() / 10_f64.powi(decimals);
            assert_eq!(rounded, 0.893997);
        }
    }
    // ---------------------------
    // Test Display
    // ---------------------------
    mod display_tests {
        use super::*;

        #[test]
        fn test_display_format() {
            let expr = "4x + 2 - 5x^2 * 4y^4 / 6z^6";
            let tok_str = lexer(expr).unwrap();
            let parsed = parse_advanced_polynomial(tok_str).unwrap();
            let display_str = format!("{parsed}");
            let expected_str = "4x + 2 - 5x^2 * 4y^4 / (6z^6)";
            assert_eq!(display_str, expected_str);
        }

        #[test]
        fn test_display_with_parentheses() {
            let expr = "4x + (2 - 5x^2) * 4x^4 / 6y^6";
            let tok_str = lexer(expr).unwrap();
            let parsed = parse_advanced_polynomial(tok_str).unwrap();
            let display_str = format!("{parsed}");
            let expected_str = "4x + (2 - 5x^2) * 4x^4 / (6y^6)";
            assert_eq!(display_str, expected_str);
        }

        #[allow(clippy::approx_constant)]
        #[test]
        fn test_display_number() {
            let e = Expr::Number(3.14);
            assert_eq!(format!("{e}"), "3.14");
        }

        #[test]
        fn test_display_variable() {
            let e = Expr::Variable("x".into());
            assert_eq!(format!("{e}"), "x");
        }

        #[test]
        fn test_display_constant_1() {
            let e = Expr::Constant(Constants::Pi);
            assert_eq!(format!("{e}"), "π");
        }

        #[test]
        fn test_display_constant_2() {
            let e = Expr::Constant(Constants::Tau);
            assert_eq!(format!("{e}"), "τ");
        }

        #[test]
        fn test_display_constant_3() {
            let e = Expr::Constant(Constants::Phi);
            assert_eq!(format!("{e}"), "ϕ");
        }

        #[test]
        fn test_display_function() {
            let e = Expr::Function {
                func: Functions::Sin,
                inner: Box::new("x".into()),
            };
            assert_eq!(format!("{e}"), "sin(x)");
        }

        #[test]
        fn test_display_unary_op() {
            let e = Expr::UnaryOpPrefix {
                op: Operators::Sub,
                value: Box::new(Expr::Number(3.0)),
            };
            assert_eq!(format!("{e}"), "-3");
        }

        #[test]
        fn test_display_binary_op() {
            let e = Expr::BinaryOp {
                op: Operators::Add,
                lhs: Box::new(Expr::Number(1.0)),
                rhs: Box::new("x".into()),
                paren: false,
            };
            assert_eq!(format!("{e}"), "1 + x");
        }

        #[test]
        fn test_display_nested_binary() {
            let e = Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::BinaryOp {
                    op: Operators::Mul,
                    lhs: Box::new(Expr::Number(4.0)),
                    rhs: Box::new("x".into()),
                    paren: false,
                }),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs: Box::new("x".into()),
                    rhs: Box::new(Expr::Number(2.0)),
                    paren: false,
                }),
                paren: false,
            };
            assert_eq!(format!("{e}"), "4x * x^2");
        }

        #[test]
        fn test_display_function_nested() {
            let e = Expr::Function {
                func: Functions::Sin,
                inner: Box::new(Expr::Function {
                    func: Functions::Cos,
                    inner: Box::new("x".into()),
                }),
            };
            assert_eq!(format!("{e}"), "sin(cos(x))");
        }

        #[test]
        fn test_display_function_with_expr() {
            let e = Expr::Function {
                func: Functions::Log,
                inner: Box::new(Expr::BinaryOp {
                    op: Operators::Mul,
                    lhs: Box::new(Expr::Number(4.0)),
                    rhs: Box::new("x".into()),
                    paren: false,
                }),
            };
            assert_eq!(format!("{e}"), "log(4x)");
        }

        #[test]
        fn test_display_function_constant_unary_prefix() {
            let e = Expr::Function {
                func: Functions::Sin,
                inner: Box::new(Expr::UnaryOpPrefix {
                    op: Operators::Sub,
                    value: Box::new(Expr::Constant(Constants::Pi)),
                }),
            };
            assert_eq!(format!("{e}"), "sin(-π)");
        }

        #[test]
        fn test_display_function_number_unary_postfix() {
            let e = Expr::Function {
                func: Functions::Sin,
                inner: Box::new(Expr::UnaryOpPostfix {
                    op: Operators::Fac,
                    value: Box::new(Expr::Number(4.0)),
                }),
            };
            assert_eq!(format!("{e}"), "sin(4!)");
        }
    }
    // ---------------------------
    // token_from_str! tests
    // ---------------------------
    mod str_macro_tests {
        use super::*;

        token_from_str! {
            #[derive(Debug, PartialEq)]
            pub TestEnum {
                Alpha => "alpha",
                Beta  => "beta",
                Gamma => "gamma",
            }
        }

        #[test]
        fn success() {
            assert_eq!(TestEnum::from_str("alpha").unwrap(), TestEnum::Alpha);
            assert_eq!(TestEnum::from_str("beta").unwrap(), TestEnum::Beta);
            assert_eq!(TestEnum::from_str("gamma").unwrap(), TestEnum::Gamma);
        }

        #[test]
        fn case_insensitive() {
            assert_eq!(TestEnum::from_str("AlPhA").unwrap(), TestEnum::Alpha);
            assert_eq!(TestEnum::from_str("BETA").unwrap(), TestEnum::Beta);
            assert_eq!(TestEnum::from_str("GaMmA").unwrap(), TestEnum::Gamma);
        }

        #[test]
        fn invalid_str() {
            assert!(TestEnum::from_str("not_an_option").is_err());
            assert!(TestEnum::from_str("").is_err());
        }
    }

    // ---------------------------
    // token_from_char! tests
    // ---------------------------
    mod char_macro_tests {

        token_from_char! {
            #[derive(Debug, PartialEq, Eq)]
            pub TestOps {
                Plus  => '+',
                Minus => '-',
            }
        }

        #[test]
        fn success() {
            assert_eq!(TestOps::from_char('+'), Ok(TestOps::Plus));
            assert_eq!(TestOps::from_char('-'), Ok(TestOps::Minus));
        }

        #[test]
        fn invalid_char() {
            assert_eq!(TestOps::from_char('*'), Err(()));
            assert_eq!(TestOps::from_char(' '), Err(()));
            assert_eq!(TestOps::from_char('\n'), Err(()));
        }
    }
}
