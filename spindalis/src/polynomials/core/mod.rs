pub mod simple;
pub mod complex;

#[derive(Debug)]
pub enum PolynomialError {
    InvalidExponent,
    InvalidConstant,
    InvalidCoefficient,
    MissingVariable,
    PolynomialSyntaxError,
}
