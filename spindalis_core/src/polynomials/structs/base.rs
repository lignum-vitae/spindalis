use crate::polynomials::base::{Expr, Token};
use std::iter::Peekable;
use std::vec::IntoIter;

pub type TokenStream = Peekable<IntoIter<Token>>;

#[derive(Debug, PartialEq)]
pub struct Polynomial {
    expr: Expr,
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

// impl PolynomialTraits for Polynomial {
//     fn parse(input: &str) -> Result<Self, PolynomialError>{
//         let tokens = lexer(input)?;
//         let polynomial = parser(tokens)?;
//         // TODO add a pass to combine 0*_ as 0 and 1*_ as _
//         Some(polynomial)
//     }
//     fn eval_univariate<F>(&self, point: F) -> Result<f64, PolynomialError>{
//     }
//     fn eval_multivariate<V, S, F>(&self, vars: &V) -> Result<f64, PolynomialError> {
//     }
//     fn derivate_univariate(&self) -> Result<Self, PolynomialError>{
//     }
//     fn derivate_multivariate<S>(&self, var: S) -> Self{
//     }
//     fn indefinite_integral_univariate(&self) -> Result<Self, PolynomialError>{
//     }
//     fn indefinite_integral_multivariate<S>(&self, var: S) -> Self{
//     }
//
// }
