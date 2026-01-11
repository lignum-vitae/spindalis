use crate::polynomials::extended::{Expr, Token};
use std::iter::Peekable;
use std::vec::IntoIter;

pub type TokenStream = Peekable<IntoIter<Token>>;

#[derive(Debug, PartialEq)]
pub struct ExtendedPolynomial {
    expr: Expr,
}
impl ExtendedPolynomial {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }
}
