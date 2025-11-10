use crate::polynomials::AstPolyErr;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Operators {
    Add,
    Sub,
    Div,
    Mul,
    Rem,
    Caret,
}

impl Operators {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Operators::Add),
            '-' => Some(Operators::Sub),
            '*' => Some(Operators::Mul),
            '/' => Some(Operators::Div),
            '%' => Some(Operators::Rem),
            '^' => Some(Operators::Caret),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Functions {
    Sin,
    Cos,
    Tan,
    Cot,
    Log,
    Ln,
}

impl FromStr for Functions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sin" => Ok(Functions::Sin),
            "cos" => Ok(Functions::Cos),
            "tan" => Ok(Functions::Tan),
            "cot" => Ok(Functions::Cot),
            "log" => Ok(Functions::Log),
            "ln" => Ok(Functions::Ln),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Constants {
    Pi,
    E,
    Tau,
    Phi,
}

impl FromStr for Constants {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pi" => Ok(Constants::Pi),
            "e" => Ok(Constants::E),
            "tau" => Ok(Constants::Tau),
            "phi" => Ok(Constants::Phi),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Variable(String),
    Operator(Operators),
    Function(Functions),
    Constant(Constants),
    LParen,
    RParen,
}

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
fn lexer<S>(input: S) -> Result<Vec<Token>, AstPolyErr>
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
                    Err(_) => return Err(AstPolyErr::InvalidNumber { num: temp }),
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
                if let Some(op) = Operators::from_char(ch) {
                    tokens.push(Token::Operator(op));
                    chars.next();
                } else {
                    return Err(AstPolyErr::UnexpectedChar { char: ch });
                }
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
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
}
