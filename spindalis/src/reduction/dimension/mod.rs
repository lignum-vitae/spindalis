pub mod linear;
pub mod non_linear;

use jedvek::Matrix2DError;
pub use linear::pca::pca;

#[derive(Debug)]
pub enum ReductionError {
    ShapeError(DimensionError),
    InvalidFlatVector(Matrix2DError),
    ZeroMean,
}

impl From<Matrix2DError> for ReductionError {
    fn from(err: Matrix2DError) -> Self {
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
