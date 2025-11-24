use crate::derivatives::extended::partial_derivative;
use crate::polynomials::extended::{eval_polynomial_extended, parse_polynomial_extended};
use crate::polynomials::structs::PolynomialTraits;
use crate::polynomials::{PolynomialError, Term};
use std::collections::HashSet;

pub struct PolynomialExtended {
    pub terms: Vec<Term>,
    pub variables: Vec<String>,
}

impl PolynomialTraits for PolynomialExtended {
    fn parse(input: &str) -> Result<PolynomialExtended, PolynomialError> {
        let parsed = parse_polynomial_extended(input)?;
        let unique_variables: HashSet<String> = parsed
            .iter()
            .flat_map(|term| term.variables.iter())
            .map(|(var_name, _)| var_name.clone())
            .collect();
        let variables: Vec<String> = unique_variables.into_iter().collect();
        Ok(PolynomialExtended {
            terms: parsed,
            variables,
        })
    }

    fn eval_univariate<F>(&self, point: F) -> Result<f64, PolynomialError>
    where
        F: Into<f64> + std::clone::Clone + std::fmt::Debug,
    {
        if self.variables.len() != 1 {
            return Err(PolynomialError::TooManyVariables {
                variables: self.variables.clone(),
            });
        }
        let evaluated =
            eval_polynomial_extended(&self.terms, &[(self.variables[0].clone(), point)])?;
        Ok(evaluated)
    }

    fn eval_multivariate<V, S, F>(&self, vars: &V) -> Result<f64, PolynomialError>
    where
        V: IntoIterator<Item = (S, F)> + std::fmt::Debug + Clone,
        S: AsRef<str>,
        F: Into<f64>,
    {
        let evaluated = eval_polynomial_extended(&self.terms, vars)?;
        Ok(evaluated)
    }
    fn derivate_univariate(&self) -> Result<Self, PolynomialError> {
        if self.variables.len() != 1 {
            return Err(PolynomialError::TooManyVariables {
                variables: self.variables.clone(),
            });
        }
        Ok(Self {
            terms: partial_derivative(&self.terms, &self.variables[0]),
            variables: self.variables.clone(),
        })
    }
    fn derivate_multivariate<S>(&self, var: S) -> Self
    where
        S: AsRef<str>,
    {
        let derived = partial_derivative(&self.terms, var);
        let variables = derived
            .iter()
            .flat_map(|term| term.variables.iter())
            .map(|(var_name, _)| var_name.clone())
            .collect();
        Self {
            terms: derived,
            variables,
        }
    }
}
