// "Needless borrows" are needed for tests
#![allow(clippy::needless_borrows_for_generic_args)]

pub mod reduction;
pub mod regressors;
pub mod solvers;
pub mod utils;

pub use spindalis_core::utils as core_util;
// Utils
pub use core_util::factorial::factorial_f64;
pub use core_util::factorial::gamma_f64;
pub use core_util::rounding::round_f64;

pub mod prelude {
    pub use crate::polynomials::PolynomialTraits;
    pub use crate::regressors::LinearRegressor;
    pub use crate::solvers::{Bounds, SolveMode};
}

pub mod polynomials {
    pub use spindalis_core::polynomials as poly;
    pub use spindalis_macros as macros;

    // Component Structs
    pub use poly::Term;
    pub use poly::structs::PolynomialTraits;

    // Polynomial Structs
    pub use poly::structs::IntermediatePolynomial;
    pub use poly::structs::Polynomial;
    pub use poly::structs::SimplePolynomial;

    // Error Enums
    pub use poly::PolynomialError;

    // Parsers and evaluators as functions
    pub use macros::{parse_intermediate_polynomial, parse_simple_polynomial};
    pub use poly::advanced::eval_advanced_polynomial;
    pub use poly::advanced::lexer;
    pub use poly::advanced::parse_advanced_polynomial;
    pub use poly::intermediate::eval_intermediate_polynomial;
    pub use poly::intermediate::parse_intermediate_polynomial;
    pub use poly::simple::eval_simple_polynomial;
    pub use poly::simple::parse_simple_polynomial;
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
