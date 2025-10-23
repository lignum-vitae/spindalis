pub mod core;
pub mod derivatives;

pub use core::complex::parse_complex_poly;

pub fn ascii_letters() -> String {
    ('a'..='z').chain('A'..='Z').collect()
}

#[derive(Debug, Clone)]
pub struct Term {
    pub coefficient: f64,
    pub variables: Vec<(String, f64)>,
}
