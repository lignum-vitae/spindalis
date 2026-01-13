// "Needless borrows" are needed for tests
#![allow(clippy::needless_borrows_for_generic_args)]

pub mod reduction;
pub mod regressors;
pub mod solvers;
pub mod utils;

pub mod polynomials {
    pub use spindalis_core::polynomials as core;
    pub use spindalis_macros as macros;

    // Component Structs
    pub use core::Term;
    pub use core::structs::PolynomialTraits;

    // Polynomial Structs
    pub use core::structs::IntermediatePolynomial;
    pub use core::structs::SimplePolynomial;

    // Error Enums
    pub use core::PolynomialError;

    // Parsers and evaluators
    pub use core::intermediate::eval_intermediate_polynomial;
    pub use core::intermediate::parse_intermediate_polynomial;
    pub use core::simple::eval_simple_polynomial;
    pub use core::simple::parse_simple_polynomial;
    pub use macros::{parse_intermediate_polynomial, parse_simple_polynomial};
}

pub mod derivatives {
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

pub mod decomposition {
    pub use crate::solvers::decomposition::lu::lu_decomposition;
    pub use crate::solvers::decomposition::plu::lu_pivot_decomposition;
}

pub mod eigen {
    pub use crate::solvers::eigen::power_method::power_method;
}
