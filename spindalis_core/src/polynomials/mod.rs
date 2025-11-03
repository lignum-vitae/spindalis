pub mod ast;
pub mod extended;
pub mod simple;

#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    pub coefficient: f64,
    pub variables: Vec<(String, f64)>,
}

#[derive(Debug)]
pub enum PolynomialError {
    InvalidExponent,
    InvalidConstant,
    InvalidCoefficient,
    MissingVariable,
    PolynomialSyntaxError,
}

#[derive(Debug)]
pub enum ComplexPolyErr {
    InvalidCoefficient { coeff: String },
    InvalidFractionalExponent { pow: String },
    InvalidExponent { pow: String },
}

#[derive(Debug)]
pub enum AstPolyErr {
    InvalidNumber { num: String },
    UnexpectedChar { char: char },
}
