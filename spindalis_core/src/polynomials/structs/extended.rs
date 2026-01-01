use crate::derivatives::extended::partial_derivative;
use crate::polynomials::extended::{eval_polynomial_extended, parse_polynomial_extended};
use crate::polynomials::structs::PolynomialTraits;
use crate::polynomials::{PolynomialError, Term};
pub struct PolynomialExtended {
    pub terms: Vec<Term>,
    pub variables: Vec<String>,
}

impl PolynomialTraits for PolynomialExtended {
    fn parse(input: &str) -> Result<PolynomialExtended, PolynomialError> {
        parse_polynomial_extended(input)
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
            terms: partial_derivative(&self.terms, &self.variables[0]).terms,
            variables: self.variables.clone(),
        })
    }
    fn derivate_multivariate<S>(&self, var: S) -> Self
    where
        S: AsRef<str>,
    {
        let derived = partial_derivative(&self.terms, var).terms;
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
impl std::fmt::Display for PolynomialExtended {
    // This trait requires the fmt method with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(PolynomialExtended{{terms:{:?}, variables:{:?}}})",
            self.terms, self.variables
        )?;

        // Return Ok(()) on success, as required by fmt::Result
        Ok(())
    }
}
