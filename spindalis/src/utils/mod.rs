#[allow(non_snake_case)]
pub mod arr2D;

pub mod variation;

#[derive(Debug)]
pub enum Arr2DError {
    InconsistentRowLengths,
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
