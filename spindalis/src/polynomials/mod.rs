pub mod derivatives;

pub use spindalis_core::polynomials as core;

pub use spindalis_core::polynomials::Term;

pub use core::extended::eval_polynomial_extended;
pub use core::extended::parse_polynomial_extended;
pub use core::simple::eval_simple_polynomial;
pub use core::simple::parse_simple_polynomial;
