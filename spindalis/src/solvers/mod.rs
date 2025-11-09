pub mod bisection;
pub mod gaussian_elim;
pub mod nrm;

pub use bisection::bisection;
pub use gaussian_elim::gaussian_elimination;
pub use nrm::newton_raphson_method;

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
    InvalidVector(Arr2DError),
    NumArgumentsMismatch { num_rows: usize, rhs_len: usize },
}

impl From<Arr2DError> for SolverError {
    fn from(err: Arr2DError) -> Self {
        SolverError::InvalidVector(err)
    }
}

// Bounds for bisection method
pub struct Bounds {
    pub lower: f64,
    pub init: f64,
    pub upper: f64,
}
