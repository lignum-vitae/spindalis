pub mod core;
pub mod derivatives;

pub use core::multivar::parse_multivar;

pub fn ascii_letters() -> String {
    ('a'..='z').chain('A'..='Z').collect()
}

#[derive(Debug, Clone)]
pub struct Term {
    pub coefficient: f64,
    pub variables: Vec<(String, usize)>,
}
