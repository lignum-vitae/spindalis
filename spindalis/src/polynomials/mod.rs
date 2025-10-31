pub mod core;
pub mod derivatives;

pub use core::extended::eval_polynomial_extended;
pub use core::extended::parse_polynomial_extended;
pub use core::simple::eval_simple_polynomial;
pub use core::simple::parse_simple_polynomial;

pub fn ascii_letters() -> String {
    ('a'..='z').chain('A'..='Z').collect()
}

#[derive(Debug, Clone)]
pub struct Term {
    pub coefficient: f64,
    pub variables: Vec<(String, f64)>,
}
