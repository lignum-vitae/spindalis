pub mod linear;
pub mod non_linear;

use crate::utils::Arr2DError;
pub use linear::pca::pca;

#[derive(Debug)]
pub enum ReductionError {
    ShapeError(DimensionError),
    InvalidFlatVector(Arr2DError),
    ZeroMean,
}

impl From<Arr2DError> for ReductionError {
    fn from(err: Arr2DError) -> Self {
        ReductionError::InvalidFlatVector(err)
    }
}

#[derive(Debug)]
pub enum DimensionError {
    NotSquare { height: usize, width: usize },
    EmptyVector,
    DimensionMismatch { len_x: usize, len_y: usize },
    Incompatible,
}

impl From<DimensionError> for ReductionError {
    fn from(err: DimensionError) -> Self {
        ReductionError::ShapeError(err)
    }
}
