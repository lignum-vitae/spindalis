pub mod bisection;
pub mod decomposition;
pub mod eigen;
pub mod gaussian_elim;
pub mod nrm;

pub use bisection::bisection;
pub use gaussian_elim::gaussian_elimination;
pub use nrm::newton_raphson_method;

use crate::polynomials::PolynomialError;
use crate::utils::Arr2DError;

#[derive(PartialEq)]
pub enum SolveMode {
    Root,
    Extrema,
}

#[derive(Debug)]
pub enum SolverError {
    MaxIterationsReached,
    NoConvergence,
    XInitOutOfBounds,
    NonSquareMatrix,
    SingularMatrix,
    InvalidVector(Arr2DError),
    FunctionError(PolynomialError),
    NumArgumentsMismatch { num_rows: usize, rhs_len: usize },
}

impl From<Arr2DError> for SolverError {
    fn from(err: Arr2DError) -> Self {
        SolverError::InvalidVector(err)
    }
}

impl From<PolynomialError> for SolverError {
    fn from(err: PolynomialError) -> Self {
        SolverError::FunctionError(err)
    }
}

// Bounds for bisection method
pub struct Bounds {
    pub lower: f64,
    pub init: f64,
    pub upper: f64,
}
