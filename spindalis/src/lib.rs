// "Needless borrows" are needed for tests
#![allow(clippy::needless_borrows_for_generic_args)]

pub mod reduction;
pub mod regressors;
pub mod solvers;
pub mod utils;

pub mod prelude {
    pub use crate::polynomials::PolynomialTraits;
    pub use crate::regressors::LinearRegressor;
    pub use crate::solvers::{Bounds, SolveMode};
}

pub mod polynomials {
    pub use spindalis_core::polynomials as core;
    pub use spindalis_macros as macros;

    // Component Structs
    pub use core::Term;
    pub use core::structs::PolynomialTraits;

    // Polynomial Structs
    pub use core::structs::IntermediatePolynomial;
    pub use core::structs::Polynomial;
    pub use core::structs::SimplePolynomial;

    // Error Enums
    pub use core::PolynomialError;

    // Parsers and evaluators as functions
    pub use core::advanced::eval_advanced_polynomial;
    pub use core::advanced::lexer;
    pub use core::advanced::parse_advanced_polynomial;
    pub use core::intermediate::eval_intermediate_polynomial;
    pub use core::intermediate::parse_intermediate_polynomial;
    pub use core::simple::eval_simple_polynomial;
    pub use core::simple::parse_simple_polynomial;
    pub use macros::{parse_intermediate_polynomial, parse_simple_polynomial};
}

pub mod derivatives {
    pub use spindalis_core::derivatives::advanced::advanced_derivative;
    pub use spindalis_core::derivatives::intermediate::partial_derivative;
    pub use spindalis_core::derivatives::simple::simple_derivative;
}

pub mod integrals {
    // Error Enums
    pub use spindalis_core::integrals::IntegralError;

    // Functions
    pub use spindalis_core::integrals::intermediate_indefinite::indefinite_integral_intermediate;
    pub use spindalis_core::integrals::simple_indefinite::indefinite_integral_simple;
    pub use spindalis_core::integrals::univariate_definite::analytical_integral;
    pub use spindalis_core::integrals::univariate_definite::definite_integral;
    pub use spindalis_core::integrals::univariate_definite::romberg_definite;
}

pub mod eigen {
    pub use crate::solvers::eigen::power_method::power_method;
}
