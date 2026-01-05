pub mod ast;
pub mod extended;
pub mod simple;

pub mod structs;

pub use extended::Term;

// Error Enum

#[derive(Debug)]
pub enum PolynomialError {
    InvalidCoefficient { coeff: String },
    InvalidConstant,
    InvalidExponent { pow: String },
    InvalidFractionalExponent { pow: String },
    InvalidFraction { frac: String },
    InvalidNumber { num: String },
    PolynomialSyntaxError,
    MissingVariable,
    TooManyVariables { variables: Vec<String> },
    TooFewVariables { variables: Vec<String> },
    UnexpectedChar { char: char },
    VariableNotFound { variable: String },
}
