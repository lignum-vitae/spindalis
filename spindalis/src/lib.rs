pub mod reduction;
pub mod regressors;
pub mod solvers;
pub mod utils;

pub mod polynomials {
    pub use spindalis_core::polynomials as core;
    pub use spindalis_macros as macros;

    // Structs
    pub use core::Term;

    // Error Enums
    pub use core::AstPolyErr;
    pub use core::ComplexPolyErr;
    pub use core::PolynomialError;

    // Parsers and evaluators
    pub use core::extended::eval_polynomial_extended;
    pub use core::extended::parse_polynomial_extended;
    pub use core::simple::eval_simple_polynomial;
    pub use core::simple::parse_simple_polynomial;
    pub use macros::{parse_polynomial_extended, parse_simple_polynomial};
}

pub mod derivatives {
    pub use spindalis_core::derivatives::extended::partial_derivative;
    pub use spindalis_core::derivatives::simple::simple_derivative;
}

pub mod integrals {
    // Error Enums
    pub use spindalis_core::integrals::IntegralError;

    // Functions
    pub use spindalis_core::integrals::simple_definite::definite_integral;
    pub use spindalis_core::integrals::simple_definite::romberg_definite;
    pub use spindalis_core::integrals::simple_definite::analytical_integral;
    pub use spindalis_core::integrals::simple_indefinite::indefinite_integral;
}

/*
pub use regressors::linear::linear_regression;
*/
