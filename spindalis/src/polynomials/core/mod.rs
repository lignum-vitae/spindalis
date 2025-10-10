pub mod multivar;
pub mod polynomial;

#[derive(Debug)]
pub enum PolynomialError {
    InvalidExponent,
    InvalidConstant,
    InvalidCoefficient,
    MissingVariable,
    PolynomialSyntaxError,
}
