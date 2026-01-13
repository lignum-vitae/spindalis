use crate::polynomials::PolynomialError;
use crate::polynomials::structs::base::{Polynomial, TokenStream};
use std::collections::HashMap;
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
*  type Err = ();
*  fn from_str(s:&str)->Result<Self,Self::Err>{
*      match s.to_lowercase().as_str(){
*          var1str => Ok(SomeEnum::Var1),
*          var2str => Ok(SomeEnum::Var2),
*          ...
*          _ => Err(()),
*      }
*  }
* }
*
*/

macro_rules! token_from_str {
    (
        //INPUT
        $(#[$meta_exp:meta])*
        $visb:vis $enum_name:ident {
            $($var_name:ident => $var_str:literal),* $(,)*
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
            fn from_str(s:&str)->::std::result::Result<Self,Self::Err>{
                match s.to_lowercase().as_str(){
                    $($var_str => Ok($enum_name::$var_name),)*
                    _ => Err(()),
                }
            }
        }
    };
}

// `token_from_char` is similar to `token_from_str`
macro_rules! token_from_char {
    // INPUT
    (
        // attribute(s) (optional) e.g. `#[Derive(Foo,Bar)]`
        $(#[$attr:meta])*
        // visibility and enum name e.g. `pub SomeEnum {...}`
        $visb:vis $enum_name:ident {
            // variants & their `char` values e.g. `Add => +`
            $($var_name:ident => $var_char:literal),* $(,)*
        }
    ) =>
    // OUTPUT
    {
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
                    $($var_char => Ok($enum_name::$var_name),)*
                    _ => Err(()),
                }
            }
        }
    };
}

// declaring `Operators` with `token_from_char`
token_from_char! {
    #[derive(Debug, PartialEq, Hash, Eq,Copy,Clone)]
    pub Operators {
        Add   => '+',
        Sub   => '-',
        Div   => '/',
        Mul   => '*',
        CDot   => '·',
        Rem   => '%',
        Caret => '^',
        Fac => '!',
    }
}

impl std::fmt::Display for Operators {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Div => "/",
            Self::Mul => "*",
            Self::CDot => "·",
            Self::Rem => "%",
            Self::Caret => "^",
            Self::Fac => "!",
        };
        write!(f, "{s}")
    }
}

// declaring `Functions` with `token_from_str`
token_from_str! {
    #[derive(Debug, PartialEq, Eq,Clone)]
    pub Functions {
        Sin => "sin",
        Cos => "cos",
        Tan => "tan",
        Cot => "cot",
        Log => "log",
        Ln => "ln",
    }
}

impl std::fmt::Display for Functions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Sin => "sin",
            Self::Cos => "cos",
            Self::Tan => "tan",
            Self::Cot => "cot",
            Self::Log => "log",
            Self::Ln => "ln",
        };
        write!(f, "{s}")
    }
}

// declaring `Constants` with `token_from_str`
token_from_str! {
    #[derive(Debug, PartialEq, Eq,Clone)]
    pub Constants {
        Pi => "pi",
        E => "e",
        Tau => "tau",
        Phi => "phi",
    }
}

impl std::fmt::Display for Constants {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Pi => "π",
            Self::E => "e",
            Self::Tau => "τ",
            Self::Phi => "ϕ",
        };
        write!(f, "{s}")
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

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Constant(Constants),
    Function {
        func: Functions,
        inner: Box<Self>,
    },
    UnaryOpPrefix {
        op: Operators,
        value: Box<Self>,
    },
    UnaryOpPostfix {
        op: Operators,
        value: Box<Self>,
    },
    BinaryOp {
        op: Operators,
        lhs: Box<Self>,
        rhs: Box<Self>,
        paren: bool,
    },
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Variable(v) => write!(f, "{v}"),
            Self::Constant(c) => write!(f, "{c}"),
            Self::Function { func, inner } => write!(f, "{func}({inner})"),
            Self::UnaryOpPrefix { op, value } => write!(f, "{op}{value}"),
            Self::UnaryOpPostfix { op, value } => write!(f, "{value}{op}"),
            Self::BinaryOp {
                op,
                lhs,
                rhs,
                paren,
            } => {
                // 4 * x   -> 4x
                // 5 * x^2 -> 5x^2
                // 2 * π   -> 2π
                let mut implied: Option<String> = None;

                if *op == Operators::Mul {
                    implied = match (&**lhs, &**rhs) {
                        (Self::Number(n), Self::Variable(v)) => Some(format!("{n}{v}")),
                        (Self::Number(n), Self::Constant(c)) => Some(format!("{n}{c}")),
                        (
                            Self::Number(n),
                            Self::BinaryOp {
                                op: Operators::Caret,
                                ..
                            },
                        ) => Some(format!("{n}{rhs}")),
                        (Self::Variable(v), Self::Number(n)) => Some(format!("{v}{n}")),
                        (Self::Constant(c), Self::Number(n)) => Some(format!("{c}{n}")),
                        _ => None,
                    };
                } else if *op == Operators::Caret {
                    implied = match (&**lhs, &**rhs) {
                        (Self::Variable(v), Self::Number(n)) => Some(format!("{v}^{n}")),
                        (Self::Constant(c), Self::Number(n)) => Some(format!("{c}^{n}")),
                        _ => None,
                    };
                }

                if let Some(s) = implied {
                    if *paren {
                        return write!(f, "({s})");
                    }
                    return write!(f, "{s}");
                }
                if *paren {
                    write!(f, "({lhs} {op} {rhs})")
                } else {
                    write!(f, "{lhs} {op} {rhs}")
                }
            }
        }
    }
}

#[allow(dead_code)]
fn lexer<S>(input: S) -> Result<Vec<Token>, PolynomialError>
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
                                tokens.push(Token::Variable(char.to_string()));
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
                if let Ok(op) = Operators::from_char(ch) {
                    tokens.push(Token::Operator(op));
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
    // This is later replaced with the Mul operator in the parser function.
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
            let inner = parse_expr(token_stream, 5.0)?;
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

    while matches!(token_stream.peek(), Some(Token::Operator(op)) if *op == Operators::Fac) {
        token_stream.next();
        left = Expr::UnaryOpPostfix {
            op: Operators::Fac,
            value: Box::new(left),
        };
    }

    // iteratively looks for operators with lower binding than minimum binding power
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

#[allow(dead_code)]
fn parser(token_stream: Vec<Token>) -> Result<Polynomial, PolynomialError> {
    let mut tokens = token_stream;
    implied_multiplication_pass(&mut tokens);
    let mut token_stream = tokens.into_iter().peekable();

    // Parse valid polynomial until end or invalid token
    let ast_node = parse_expr(&mut token_stream, 0.0)?;

    // Returns error if there are remaining tokens after parsing
    if let Some(token) = token_stream.next() {
        return Err(PolynomialError::UnexpectedToken { token });
    }

    Ok(Polynomial::new(ast_node))
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
                println!("{:?} {:?}", result[i], expected[i]);
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
                println!("{:?} {:?}", result[i], expected[i]);
                assert_eq!(result[i], expected[i]);
            }
        }
    }

    // ---------------------------
    // Parsing tests
    // ---------------------------
    mod parser_tests {
        use super::*;

        #[test]
        fn test_number_parse() {
            let expr = "4";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Number(4.0));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_variable_parse() {
            let expr = "x";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Variable("x".into()));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_exponents_parse() {
            let expr = "x^2";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str).unwrap();
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
            let result = parser(tok_str).unwrap();
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
            let result = parser(tok_str).unwrap();
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
            let result = parser(tok_str).unwrap();
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
            let result = parser(tok_str).unwrap();
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
            let result = parser(tok_str).unwrap();

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
            let expr = "4x + 2 - 5x^2 * 4x^4 / 6x^6";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str).unwrap();

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

            // (4 * x^4)
            let term_4x4 = Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(4.0)),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs: Box::new(Expr::Variable("x".into())),
                    rhs: Box::new(Expr::Number(4.0)),
                    paren: false,
                }),
                paren: false,
            };

            // (5 * x^2) * (4 * x^4)
            let numerator = Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(term_5x2),
                rhs: Box::new(term_4x4),
                paren: false,
            };

            // 6 * x^6
            let denominator = Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(6.0)),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs: Box::new(Expr::Variable("x".into())),
                    rhs: Box::new(Expr::Number(6.0)),
                    paren: false,
                }),
                paren: false,
            };

            // ((5 * x^2) * (4 * x^4)) / (6 * x^6)
            let fmul_right = Expr::BinaryOp {
                op: Operators::Div,
                lhs: Box::new(numerator),
                rhs: Box::new(denominator),
                paren: false,
            };

            // ((4 * x) + 2) - ((5 * x^2) * (4 * x^4)) / (6 * x^6)
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
            let result = parser(tok_str).unwrap();
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(0.0)),
                rhs: Box::new(Expr::Variable("x".into())),
                paren: false,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_zero_parse() {
            let expr = "0";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str).unwrap();
            let expected = Polynomial::new(Expr::Number(0.0));
            assert_eq!(result, expected);
        }

        #[test]
        fn test_multivariate_expression_parse() {
            let expr = "4xy + 4x^2 - 2y + 4";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str).unwrap();

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
            let result = parser(tok_str).unwrap();
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
            let result = parser(tkn_str).unwrap();
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Div,
                lhs: Box::new(Expr::BinaryOp {
                    op: Operators::Add,
                    lhs: Box::new(Expr::Number(3.0)),
                    rhs: Box::new(Expr::Number(2.0)),
                    paren: true,
                }),
                rhs: Box::new(Expr::Number(4.0)),
                paren: false,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_parsing_no_parents() {
            let expr = "3 + 2 / 4";
            let tkn_str = lexer(expr).unwrap();
            let result = parser(tkn_str).unwrap();
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Add,
                lhs: Box::new(Expr::Number(3.0)),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Div,
                    lhs: Box::new(Expr::Number(2.0)),
                    rhs: Box::new(Expr::Number(4.0)),
                    paren: false,
                }),
                paren: false,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_nested_parens() {
            let expr = "(3 + (4+2)) * 5x";
            let tkn_str = lexer(expr).unwrap();
            let result = parser(tkn_str).unwrap();
            let expected = Polynomial::new(Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::BinaryOp {
                    op: Operators::Add,
                    lhs: Box::new(Expr::Number(3.0)),
                    rhs: Box::new(Expr::BinaryOp {
                        op: Operators::Add,
                        lhs: Box::new(Expr::Number(4.0)),
                        rhs: Box::new(Expr::Number(2.0)),
                        paren: true,
                    }),
                    paren: true,
                }),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Mul,
                    lhs: Box::new(Expr::Number(5.0)),
                    rhs: Box::new(Expr::Variable("x".into())),
                    paren: false,
                }),
                paren: false,
            });
            assert_eq!(result, expected);
        }

        #[test]
        fn test_unary_prefix() {
            let expr = "-3x + 4";
            let tkn_str = lexer(expr).unwrap();
            let result = parser(tkn_str).unwrap();

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
            let result = parser(tok_str).unwrap();
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
            let result = parser(tok_str).unwrap();
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
            let result = parser(tok_str).unwrap();
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
            let result = parser(tok_str).unwrap();
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
            let result = parser(tok_str).unwrap();
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
            let result = parser(tok_str);
            println!("{result:?}");
            assert!(result.is_err());
        }

        #[test]
        fn test_missing_right_hand() {
            let expr = "4x +";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            println!("{result:?}");
            assert!(result.is_err());
        }

        #[test]
        fn test_missing_left_hand() {
            let expr = "+ 3x";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            println!("{result:?}");
            assert!(result.is_err());
        }

        #[test]
        fn test_only_operator() {
            let expr = "+";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            println!("{result:?}");
            assert!(result.is_err());
        }

        #[test]
        fn test_invalid_multiple_exponents() {
            let expr = "4x^^^2";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            println!("{result:?}");
            assert!(result.is_err());
        }

        #[test]
        fn test_missing_closing_paren() {
            let expr = "(4x + 2 / 4";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            println!("{result:?}");
            assert!(result.is_err());
        }

        #[test]
        fn test_missing_opening_paren() {
            let expr = "4x + 2) / 4";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            println!("{result:?}");
            assert!(result.is_err());
        }

        #[test]
        fn test_missing_opening_paren_end_with_closing_paren() {
            let expr = "4x + 2)";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            println!("{result:?}");
            assert!(result.is_err());
        }

        #[test]
        fn test_missing_func_paren() {
            let expr = "sin 4";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_missing_func_closing_paren() {
            let expr = "sin(4";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_only_minus_is_allowed() {
            let expr = "+3";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_invalid_operator_mul() {
            let expr = "*3";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_invalid_operator_div() {
            let expr = "/3";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_invalid_operator_caret() {
            let expr = "^3";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_invalid_operator_rem() {
            let expr = "%3";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_missing_rhs() {
            let expr = "-";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_cannot_be_followed_by_rparen() {
            let expr = "-)";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_cannot_be_followed_by_binary_operator() {
            let expr = "-+3";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_unary_cannot_start_expression() {
            let expr = "!";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_unary_cannot_follow_binary_operator() {
            let expr = "3+!2";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_unary_cannot_follow_lparen() {
            let expr = "(!2)";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_unary_cannot_follow_prefix_minus() {
            let expr = "-!3";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_unary_cannot_apply_to_empty_parens() {
            let expr = "()!";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_unary_extra_token_after_factorial() {
            let expr = "3!x";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_postfix_unary_invalid_following_token_lparen() {
            let expr = "3!(";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_inside_parens_missing_rhs() {
            let expr = "(-)";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }

        #[test]
        fn test_unary_prefix_after_binary_operator_missing_rhs() {
            let expr = "3*-";
            let tok_str = lexer(expr).unwrap();
            let result = parser(tok_str);
            assert!(result.is_err());
        }
    }

    // ---------------------------
    // Test Display
    // ---------------------------
    mod display_tests {
        use super::*;

        #[test]
        fn test_display_format() {
            let expr = "4x + 2 - 5x^2 * 4x^4 / 6x^6";
            let tok_str = lexer(expr).unwrap();
            let parsed = parser(tok_str).unwrap();
            let display_str = format!("{parsed}");
            let expected_str = "4x + 2 - 5x^2 * 4x^4 / 6x^6";
            assert_eq!(display_str, expected_str);
        }

        #[test]
        fn test_display_with_parentheses() {
            let expr = "4x + (2 - 5x^2) * 4x^4 / 6x^6";
            let tok_str = lexer(expr).unwrap();
            let parsed = parser(tok_str).unwrap();
            let display_str = format!("{parsed}");
            let expected_str = "4x + (2 - 5x^2) * 4x^4 / 6x^6";
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
        fn test_display_constant() {
            let e = Expr::Constant(Constants::Pi);
            assert_eq!(format!("{e}"), "π");
        }

        #[test]
        fn test_display_function() {
            let e = Expr::Function {
                func: Functions::Sin,
                inner: Box::new(Expr::Variable("x".into())),
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
                rhs: Box::new(Expr::Variable("x".into())),
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
                    rhs: Box::new(Expr::Variable("x".into())),
                    paren: false,
                }),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs: Box::new(Expr::Variable("x".into())),
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
                    inner: Box::new(Expr::Variable("x".into())),
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
                    rhs: Box::new(Expr::Variable("x".into())),
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
            #[derive(Debug, PartialEq, Eq)]
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

        // edge case: empty enum
        // compiles but returns `Err(())` on comparison
        token_from_str! {
            #[derive(Debug,PartialEq, Eq)]
            pub EmptyEnum{}
        }

        #[test]
        fn empty_enum_returns_err() {
            assert_eq!(EmptyEnum::from_str("abc"), Err(()));
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

        // edge case: empty enum
        // compiles but always returns `Err(())`
        token_from_char! {
            #[derive(Debug, PartialEq, Eq)]
            pub EmptyEnum {}
        }

        #[test]
        fn empty_enum_returns_err() {
            assert_eq!(EmptyEnum::from_char('x'), Err(()));
        }
    }
}
