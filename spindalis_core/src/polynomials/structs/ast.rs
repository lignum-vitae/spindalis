use crate::polynomials::ast::{Expr, Token};
use std::iter::Peekable;
use std::vec::IntoIter;

pub type TokenStream = Peekable<IntoIter<Token>>;

#[derive(Debug, PartialEq)]
pub(crate) struct PolynomialAst {
    expr: Expr,
}
impl PolynomialAst {
    pub(crate) fn new(expr: Expr) -> Self {
        return Self { expr };
    }
}
