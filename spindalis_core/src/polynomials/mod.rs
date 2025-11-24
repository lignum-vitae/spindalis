pub mod ast;
pub mod extended;
pub mod simple;

pub mod structs;

#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    pub coefficient: f64,
    pub variables: Vec<(String, f64)>,
}

#[derive(Debug)]
pub enum PolynomialError {
    InvalidCoefficient { coeff: String },
    InvalidConstant,
    InvalidExponent { pow: String },
    InvalidFractionalExponent { pow: String },
    PolynomialSyntaxError,
    MissingVariable,
    TooManyVariables { variables: Vec<String> },
    TooFewVariables { variables: Vec<String> },
}

#[derive(Debug)]
pub enum AstPolyErr {
    InvalidNumber { num: String },
    UnexpectedChar { char: char },
}
