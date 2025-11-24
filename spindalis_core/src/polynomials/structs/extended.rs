use crate::derivatives::extended::partial_derivative;
use crate::polynomials::extended::{eval_polynomial_extended, parse_polynomial_extended};
use crate::polynomials::structs::PolynomialTraits;
use crate::polynomials::{PolynomialError, Term};

struct PolynomialExtended {
    pub terms: Vec<Term>,
    pub variables: Vec<String>,
}
/*
impl PolynomialTraits for PolynomialExtended {
    fn parse(&self, input: &str) -> Result<Self, PolynomialError>
    where
        Self: std::marker::Sized,
    {
        let parsed = parse_polynomial_extended(input)?;
        let variables = parsed
            .iter()
            .flat_map(|term| term.variables.iter())
            .map(|(var_name, _)| var_name.clone())
            .collect();
        Ok(PolynomialExtended {
            terms: parsed,
            variables,
        })
    }

    fn eval_univariate<F>(&self, point: F) -> f64
    where
        F: Into<f64>,
    {

    }
    fn eval_multivariate<V, S, F>(&self, vars: &V) -> Option<f64>
    where
        V: IntoIterator<Item = (S, F)> + std::fmt::Debug + Clone,
        S: AsRef<str>,
        F: Into<f64>,
    {
    }
    fn derivate_univariate(&self) -> Self {}
    fn derivate_multivariate<S>(&self, var: S) -> Option<Self>
    where
        Self: std::marker::Sized,
        S: AsRef<str>,
    {
    }
}
*/
