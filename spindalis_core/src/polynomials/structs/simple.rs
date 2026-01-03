use crate::derivatives::simple::simple_derivative;
use crate::polynomials::PolynomialError;
use crate::polynomials::simple::{eval_simple_polynomial, parse_simple_polynomial};
use crate::polynomials::structs::PolynomialTraits;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SimplePolynomial {
    pub coefficients: Vec<f64>,
}

impl PartialEq<Vec<f64>> for SimplePolynomial {
    fn eq(&self, other: &Vec<f64>) -> bool {
        &self.coefficients == other
    }
}

impl PolynomialTraits for SimplePolynomial {
    fn parse(input: &str) -> Result<SimplePolynomial, PolynomialError> {
        parse_simple_polynomial(input)
    }

    fn eval_univariate<F>(&self, point: F) -> Result<f64, PolynomialError>
    where
        F: Into<f64> + std::clone::Clone + std::fmt::Debug,
    {
        Ok(eval_simple_polynomial(point, &self.coefficients))
    }

    fn derivate_univariate(&self) -> Result<Self, PolynomialError> {
        let derivative = simple_derivative(&self.coefficients);
        Ok(derivative)
    }

    // Simple Polynomial can only handle univariate inputs
    #[allow(unused_variables)]
    fn eval_multivariate<V, S, F>(&self, vars: &V) -> Result<f64, PolynomialError>
    where
        V: IntoIterator<Item = (S, F)> + std::fmt::Debug + Clone,
        S: AsRef<str>,
        F: Into<f64>,
    {
        let vars_map: HashMap<String, f64> = vars
            .clone()
            .into_iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.into()))
            .collect();
        if vars_map.len() != 1 {
            let vars: Vec<String> = vars_map.keys().cloned().collect();
            return Err(PolynomialError::TooManyVariables { variables: vars });
        }
        let point: f64 = *vars_map.values().next().unwrap_or(&0_f64);
        Ok(eval_simple_polynomial(point, &self.coefficients))
    }

    #[allow(unused_variables)]
    fn derivate_multivariate<S>(&self, var: S) -> Self {
        simple_derivative(&self.coefficients)
    }
}

impl std::fmt::Display for SimplePolynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut first = true;
        // Assume coefficients are ordered from highest degree to lowest
        for (i, &coeff) in self.coefficients.iter().enumerate().rev() {
            if coeff == 0.0 {
                continue;
            }

            // Handle signs and spacing
            if !first && coeff > 0.0 {
                write!(f, " + ")?;
            } else if coeff < 0.0 {
                write!(f, " - ")?;
            }

            let abs_coeff = coeff.abs();

            // Print the coefficient if it's not 1 (or if it's the constant term)
            write!(f, "{}", abs_coeff)?;

            match i {
                0 => {} // Constant term
                1 => write!(f, "x")?,
                _ => write!(f, "x^{}", i)?,
            }
            first = false;
        }

        if first {
            write!(f, "0")?;
        }

        Ok(())
    }
}
