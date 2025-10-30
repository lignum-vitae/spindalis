pub mod core;
pub mod derivatives;

pub use core::extended::parse_polynomial_extended;

pub fn ascii_letters() -> String {
    ('a'..='z').chain('A'..='Z').collect()
}

#[derive(Debug, Clone)]
pub struct Term {
    pub coefficient: f64,
    pub variables: Vec<(String, f64)>,
}
