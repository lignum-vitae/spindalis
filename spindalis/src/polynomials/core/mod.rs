pub mod extended;
pub mod simple;

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
