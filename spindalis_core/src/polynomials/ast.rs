use crate::polynomials::PolynomialError;
use crate::polynomials::structs::ast::{PolynomialAst, TokenStream};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str::FromStr;

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
        Rem   => '%',
        Caret => '^',
    }
}

// declaring `Functions` with `token_from_str`
token_from_str! {
    #[derive(Debug, PartialEq,Clone)]
    pub Functions {
        Sin => "sin",
        Cos => "cos",
        Tan => "tan",
        Cot => "cot",
        Log => "log",
        Ln => "ln",
    }
}

// declaring `Constants` with `token_from_str`
token_from_str! {
    #[derive(Debug, PartialEq,Clone)]
    pub Constants {
        Pi => "pi",
        E => "e",
        Tau => "tau",
        Phi => "phi",
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
        inner: Box<Expr>,
    },
    UnaryOp {
        op: Operators,
        node: Box<Expr>,
    },
    BinaryOp {
        op: Operators,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[allow(dead_code)]
fn lexer<S>(input: S) -> Result<Vec<Token>, PolynomialError>
where
    S: AsRef<str>,
{
    let input = input.as_ref();
    let mut tokens: Vec<Token> = Vec::new();
    let mut temp = String::new();
    let chars = input.replace(" ", "");

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
                            tokens.push(Token::Constant(x))
                        } else {
                            tokens.push(Token::Variable(temp.clone()))
                        }
                    }
                    _ => {
                        if let Ok(x) = Functions::from_str(&temp) {
                            tokens.push(Token::Function(x))
                        } else if let Ok(x) = Constants::from_str(&temp) {
                            tokens.push(Token::Constant(x))
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

static BINDING_POW: Lazy<HashMap<Operators, f64>> = Lazy::new(|| {
    HashMap::from([
        (Operators::Sub, 1.0),
        (Operators::Add, 1.0),
        (Operators::Mul, 2.0),
        (Operators::Div, 2.0),
        (Operators::Rem, 3.0),
        (Operators::Caret, 4.0),
    ])
});

fn expect(token_stream: &mut TokenStream, expected_token: Token) -> Result<(), PolynomialError> {
    if let Some(token) = token_stream.peek() {
        if expected_token == *token {
            return Ok(());
        } else {
            return Err(PolynomialError::UnexpectedToken {
                token: token.clone(),
            });
        }
    } else {
        return Err(PolynomialError::UnexpectedEndOfTokens);
    }
}

fn implied_multiplication_pass(token_stream: &mut Vec<Token>) {
    // Pass to identify implied multiplication to insert Operators::Mul
    // e.g. 4x = 4*x
    // 4x^2 == 4*(x^2)
    let mut idx = 0;
    loop {
        if idx >= token_stream.len() {
            break;
        }
        match token_stream.get(idx) {
            Some(Token::Number(_)) => match token_stream.get(idx + 1) {
                Some(Token::Variable(_)) => {
                    token_stream.insert(idx + 1, Token::Operator(Operators::Mul));
                }
                Some(Token::Function(_)) => {
                    token_stream.insert(idx + 1, Token::Operator(Operators::Mul));
                }
                Some(Token::Constant(_)) => {
                    token_stream.insert(idx + 1, Token::Operator(Operators::Mul));
                }
                Some(Token::LParen) => {
                    token_stream.insert(idx + 1, Token::Operator(Operators::Mul));
                }
                _ => {}
            },
            Some(Token::Variable(_)) => match token_stream.get(idx + 1) {
                Some(Token::Number(_)) => {
                    token_stream.insert(idx + 1, Token::Operator(Operators::Mul));
                }
                Some(Token::Variable(_)) => {
                    token_stream.insert(idx + 1, Token::Operator(Operators::Mul));
                }
                Some(Token::Function(_)) => {
                    token_stream.insert(idx + 1, Token::Operator(Operators::Mul));
                }
                Some(Token::Constant(_)) => {
                    token_stream.insert(idx + 1, Token::Operator(Operators::Mul));
                }
                Some(Token::LParen) => {
                    token_stream.insert(idx + 1, Token::Operator(Operators::Mul));
                }
                _ => {}
            },
            Some(Token::Constant(_)) => match token_stream.get(idx + 1) {
                Some(Token::Number(_)) => {
                    token_stream.insert(idx + 1, Token::Operator(Operators::Mul));
                }
                Some(Token::Variable(_)) => {
                    token_stream.insert(idx + 1, Token::Operator(Operators::Mul));
                }
                Some(Token::Function(_)) => {
                    token_stream.insert(idx + 1, Token::Operator(Operators::Mul));
                }
                Some(Token::Constant(_)) => {
                    token_stream.insert(idx + 1, Token::Operator(Operators::Mul));
                }
                Some(Token::LParen) => {
                    token_stream.insert(idx + 1, Token::Operator(Operators::Mul));
                }
                _ => {}
            },
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
            let expr = parse_expr(token_stream, 0.0);
            expect(token_stream, Token::RParen)?;
            expr
        }
        Some(Token::Function(f)) => {
            let func = f;
            let inner = parse_expr(token_stream, 5.0)?;
            return Ok(Expr::Function {
                func,
                inner: Box::new(inner),
            });
        }
        _ => return Err(PolynomialError::PolynomialSyntaxError),
    }?;
    // iteratively looks for operators with lower binding than minimum binding power
    loop {
        if let Some(Token::Operator(op)) = token_stream.peek() {
            let cbind_pow = *BINDING_POW.get(op).unwrap_or(&0.0);
            if cbind_pow < min_bind_pow {
                break;
            } else {
                let op = *op;
                token_stream.next();
                // right associativity of operators
                let right = parse_expr(token_stream, cbind_pow + 1.0)?;
                left = Expr::BinaryOp {
                    op,
                    lhs: Box::new(left),
                    rhs: Box::new(right),
                };
                continue;
            }
        } else {
            break;
        }
    }
    return Ok(left);
}

#[allow(dead_code)]
fn parser(token_stream: Vec<Token>) -> Result<PolynomialAst, PolynomialError> {
    let mut tokens = token_stream;
    implied_multiplication_pass(&mut tokens);
    let mut token_stream = tokens.into_iter().peekable();
    Ok(PolynomialAst::new(parse_expr(&mut token_stream, 0.0)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_parse() {
        let expr = "4";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str).unwrap();
        let expect = PolynomialAst::new(Expr::Number(4.0));
        assert_eq!(result, expect);
    }

    #[test]
    fn test_variable_parse() {
        let expr = "x";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str).unwrap();
        let expect = PolynomialAst::new(Expr::Variable("x".into()));
        assert_eq!(result, expect);
    }

    #[test]
    fn test_exponents_parse() {
        let expr = "x^2";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str).unwrap();
        let expect = PolynomialAst::new(Expr::BinaryOp {
            op: Operators::Caret,
            lhs: Box::new(Expr::Variable("x".into())),
            rhs: Box::new(Expr::Number(2.0)),
        });
        assert_eq!(result, expect);
    }

    #[test]
    fn test_integer_coefficient_parse() {
        let expr = "4x";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str).unwrap();
        let expect = PolynomialAst::new(Expr::BinaryOp {
            op: Operators::Mul,
            lhs: Box::new(Expr::Number(4.0)),
            rhs: Box::new(Expr::Variable("x".into())),
        });
        assert_eq!(result, expect);
    }

    #[test]
    fn test_float_coefficient_parse() {
        let expr = "4.2x";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str).unwrap();
        let expect = PolynomialAst::new(Expr::BinaryOp {
            op: Operators::Mul,
            lhs: Box::new(Expr::Number(4.2)),
            rhs: Box::new(Expr::Variable("x".into())),
        });
        assert_eq!(result, expect);
    }

    #[test]
    fn test_basic_expression_parse() {
        let expr = "4x+2";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str).unwrap();
        let expect = PolynomialAst::new(Expr::BinaryOp {
            op: Operators::Add,
            lhs: Box::new(Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(4.0)),
                rhs: Box::new(Expr::Variable("x".into())),
            }),
            rhs: Box::new(Expr::Number(2.0)),
        });
        assert_eq!(result, expect);
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
            }),
        };

        let r_child = Expr::BinaryOp {
            op: Operators::Mul,
            lhs: Box::new(Expr::Number(2.3)),
            rhs: Box::new(Expr::BinaryOp {
                op: Operators::Caret,
                lhs: Box::new(Expr::Variable("x".into())),
                rhs: Box::new(Expr::Number(3.0)),
            }),
        };

        let expect = PolynomialAst::new(Expr::BinaryOp {
            op: Operators::Add,
            lhs: Box::new(l_child),
            rhs: Box::new(r_child),
        });
        assert_eq!(result, expect);
    }

    #[test]
    fn test_complex_expression_parse() {
        let expr = "4x + 2 - 5x^2 * 4x^4 / 6x^6";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str).unwrap();

        // x^2
        let term_x2 = Expr::BinaryOp {
            op: Operators::Caret,
            lhs: Box::new(Expr::Variable("x".into())),
            rhs: Box::new(Expr::Number(2.0)),
        };

        // 5 * x^2
        let term_5x2 = Expr::BinaryOp {
            op: Operators::Mul,
            lhs: Box::new(Expr::Number(5.0)),
            rhs: Box::new(term_x2),
        };

        // (5 * x^2) * 4
        let term_5x24 = Expr::BinaryOp {
            op: Operators::Mul,
            lhs: Box::new(term_5x2),
            rhs: Box::new(Expr::Number(4.0)),
        };

        // x^4
        let term_x4 = Expr::BinaryOp {
            op: Operators::Caret,
            lhs: Box::new(Expr::Variable("x".into())),
            rhs: Box::new(Expr::Number(4.0)),
        };

        // ((5 * x^2) * 4) * x^4
        let term_5x24x4 = Expr::BinaryOp {
            op: Operators::Mul,
            lhs: Box::new(term_5x24),
            rhs: Box::new(term_x4),
        };

        // (((5 * x^2) * 4) * x^4) / 6
        let mul_left = Expr::BinaryOp {
            op: Operators::Div,
            lhs: Box::new(term_5x24x4),
            rhs: Box::new(Expr::Number(6.0)),
        };

        // x^6
        let term_x6 = Expr::BinaryOp {
            op: Operators::Caret,
            lhs: Box::new(Expr::Variable("x".into())),
            rhs: Box::new(Expr::Number(6.0)),
        };

        // ((((5 * x^2) * 4) * x^4) / 6) * x^6
        let fmul_right = Expr::BinaryOp {
            op: Operators::Mul,
            lhs: Box::new(mul_left),
            rhs: Box::new(term_x6),
        };

        // 4x + 2
        let fmul_left = Expr::BinaryOp {
            op: Operators::Add,
            lhs: Box::new(Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(4.0)),
                rhs: Box::new(Expr::Variable("x".into())),
            }),
            rhs: Box::new(Expr::Number(2.0)),
        };

        // (4x + 2) - (...)
        let expected = PolynomialAst::new(Expr::BinaryOp {
            op: Operators::Sub,
            lhs: Box::new(fmul_left),
            rhs: Box::new(fmul_right),
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_zero_x_parse() {
        let expr = "0x";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str).unwrap();
        let expect = PolynomialAst::new(Expr::BinaryOp {
            op: Operators::Mul,
            lhs: Box::new(Expr::Number(0.0)),
            rhs: Box::new(Expr::Variable("x".into())),
        });
        assert_eq!(result, expect);
    }

    #[test]
    fn test_zero_parse() {
        let expr = "0";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str).unwrap();
        let expect = PolynomialAst::new(Expr::Number(0.0));
        assert_eq!(result, expect);
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
            }),
            rhs: Box::new(Expr::Variable("y".into())),
        };

        // 4x^2 = 4 * (x^2)
        let term_4x2 = Expr::BinaryOp {
            op: Operators::Mul,
            lhs: Box::new(Expr::Number(4.0)),
            rhs: Box::new(Expr::BinaryOp {
                op: Operators::Caret,
                lhs: Box::new(Expr::Variable("x".into())),
                rhs: Box::new(Expr::Number(2.0)),
            }),
        };

        // 2y
        let term_2y = Expr::BinaryOp {
            op: Operators::Mul,
            lhs: Box::new(Expr::Number(2.0)),
            rhs: Box::new(Expr::Variable("y".into())),
        };

        // 4xy + 4x^2
        let term_lleft = Expr::BinaryOp {
            op: Operators::Add,
            lhs: Box::new(term_4xy),
            rhs: Box::new(term_4x2),
        };

        // (4xy + 4x^2) - 2y
        let term_left = Expr::BinaryOp {
            op: Operators::Sub,
            lhs: Box::new(term_lleft),
            rhs: Box::new(term_2y),
        };

        // ((4xy + 4x^2) - 2y) + 4
        let expect = PolynomialAst::new(Expr::BinaryOp {
            op: Operators::Add,
            lhs: Box::new(term_left),
            rhs: Box::new(Expr::Number(4.0)),
        });
        assert_eq!(result, expect);
    }

    // Error handling tests
    #[test]
    fn test_invalid_expression() {
        let expr = "4 +++ 3x";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str);
        println!("{:?}", result);
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn test_missing_right_hand() {
        let expr = "4x +";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str);
        println!("{:?}", result);
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn test_missing_left_hand() {
        let expr = "+ 3x";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str);
        println!("{:?}", result);
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn test_only_operator() {
        let expr = "+";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str);
        println!("{:?}", result);
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn test_invalid_multiple_exponents() {
        let expr = "4x^^^2";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str);
        println!("{:?}", result);
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn test_valid_multiple_exponents() {
        let expr = "4x^2^3";
        let tok_str = lexer(expr).unwrap();
        let result = parser(tok_str);
        println!("{:?}", result);
        assert!(matches!(result, Ok(_)));
    }

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
    fn test_parens() {
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

        // edge case: empty enum
        // compiles but returns `Err(())` on comparison
        token_from_str! {
            #[derive(Debug,PartialEq)]
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
            #[derive(Debug, PartialEq)]
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
            #[derive(Debug, PartialEq)]
            pub EmptyEnum {}
        }

        #[test]
        fn empty_enum_returns_err() {
            assert_eq!(EmptyEnum::from_char('x'), Err(()));
        }
    }
}
