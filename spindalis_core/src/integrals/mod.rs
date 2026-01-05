pub mod extended_indefinite;
pub mod simple_indefinite;
pub mod univariate_definite;

pub use simple_indefinite::indefinite_integral_simple;
pub use univariate_definite::analytical_integral;
pub use univariate_definite::definite_integral;
pub use univariate_definite::romberg_definite;

pub use crate::polynomials::PolynomialError;

#[derive(Debug)]
pub enum IntegralError {
    MaxIterationsReached,
    FunctionError(PolynomialError),
}

impl From<PolynomialError> for IntegralError {
    fn from(err: PolynomialError) -> Self {
        IntegralError::FunctionError(err)
    }
}
