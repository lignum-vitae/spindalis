use crate::polynomials::PolynomialError;
use crate::polynomials::advanced::{Expr, Token};
use crate::polynomials::advanced::{
    eval_advanced_polynomial, evaluate_numerical_expression, identify_univariance, lexer, parser,
};
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
impl Polynomial {
    pub fn parse(input: &str) -> Result<Polynomial, PolynomialError> {
        let tokens = lexer(input)?;
        parser(tokens)
    }

    pub fn eval_univariate<F>(&self, point: F) -> Result<f64, PolynomialError>
    where
        F: Into<f64> + std::clone::Clone + std::fmt::Debug,
    {
        let variable = match identify_univariance(&self.expr) {
            Err(PolynomialError::MissingVariable) => {
                return evaluate_numerical_expression(&self.expr)
                    .ok_or(PolynomialError::MissingVariable);
            }
            e => e,
        }?;
        eval_advanced_polynomial(self, &[(variable, point)])
    }

    pub fn eval_multivariate<V, S, F>(&self, vars: &V) -> Result<f64, PolynomialError>
    where
        V: IntoIterator<Item = (S, F)> + std::fmt::Debug + Clone,
        S: AsRef<str>,
        F: Into<f64>,
    {
        let evaluated = eval_advanced_polynomial(self, vars)?;
        Ok(evaluated)
    }
}
