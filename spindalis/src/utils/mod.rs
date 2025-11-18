#[allow(non_snake_case)]
pub mod arr2D;
pub mod substitution;
pub mod variation;

pub use arr2D::Arr2D;
pub use arr2D::Rounding;
pub use substitution::back_substitution;
pub use substitution::forward_substitution;
pub use variation::arith_mean;
pub use variation::geom_mean;
pub use variation::std_dev;

#[derive(Debug)]
pub enum Arr2DError {
    InconsistentRowLengths,
    NonSquareMatrix,
    SingularMatrix,
    InvalidReshape {
        size: usize,
        new_height: usize,
    },
    InvalidShape {
        input_size: usize,
        output_size: usize,
    },
    InvalidDotShape {
        lhs: usize,
        rhs: usize,
    },
    ConversionFailed {
        from: &'static str,
        to: &'static str,
    },
}

#[derive(Copy, Clone)]
pub enum StdDevType {
    Poulation,
    Sample,
}
