pub mod bisection;
pub mod nrm;

use crate::polynomials::core::PolynomialError;
pub use bisection::bisection;
pub use nrm::newton_raphson_method;

pub enum SolveMode {
    Root,
    Extrema,
}

#[derive(Debug)]
pub enum SolverError {
    InvalidPolynomial(PolynomialError),
    MaxIterationsReached,
}

impl From<PolynomialError> for SolverError {
    fn from(err: PolynomialError) -> Self {
        SolverError::InvalidPolynomial(err)
    }
}
