use crate::polynomials::advanced::{Expr, Token};
use std::iter::Peekable;
use std::vec::IntoIter;

pub type TokenStream = Peekable<IntoIter<Token>>;

#[derive(Debug, PartialEq)]
pub struct Polynomial {
    pub expr: Expr,
}
impl Polynomial {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }
}
impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expr)
    }
}
