use crate::derivatives::simple::simple_derivative;
use crate::integrals::simple_indefinite::indefinite_integral_simple;
use crate::polynomials::PolynomialError;
use crate::polynomials::simple::{eval_simple_polynomial, parse_simple_polynomial};
use crate::polynomials::structs::PolynomialTraits;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct SimplePolynomial {
    pub coefficients: Vec<f64>,
    pub variable: Option<char>,
}

impl SimplePolynomial {
    pub fn is_empty(&self) -> bool {
        self.coefficients.is_empty()
    }
}

impl std::ops::Deref for SimplePolynomial {
    type Target = [f64];
    fn deref(&self) -> &Self::Target {
        &self.coefficients
    }
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
        Ok(eval_simple_polynomial(point, self))
    }

    fn derivate_univariate(&self) -> Result<Self, PolynomialError> {
        Ok(simple_derivative(self))
    }

    fn indefinite_integral_univariate(&self) -> Result<Self, PolynomialError> {
        Ok(indefinite_integral_simple(self))
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
        Ok(eval_simple_polynomial(point, self))
    }

    fn derivate_multivariate<S>(&self, var: S) -> Self
    where
        S: AsRef<str>,
    {
        let var = var.as_ref();
        if var.chars().next() == self.variable {
            simple_derivative(self)
        } else {
            // Returns original poly because SimplePolynomial cannot handle multivariate
            // polynomials
            self.clone()
        }
    }

    fn indefinite_integral_multivariate<S>(&self, var: S) -> Self
    where
        S: AsRef<str>,
    {
        let var = var.as_ref();
        if var.chars().next() == self.variable {
            indefinite_integral_simple(self)
        } else {
            // Returns original poly because SimplePolynomial cannot handle multivariate
            // polynomials
            self.clone()
        }
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
            if abs_coeff != 1.0 || i == 0 {
                match f.precision() {
                    Some(p) => {
                        let formatted = format!("{:.*}", p, abs_coeff);
                        let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
                        write!(f, "{}", trimmed)?;
                    }
                    None => write!(f, "{}", abs_coeff)?,
                }
            }

            let var = self.variable.unwrap_or('x');
            match i {
                0 => {} // Constant term
                1 => write!(f, "{}", var)?,
                _ => write!(f, "{}^{}", var, i)?,
            }
            first = false;
        }

        if first {
            write!(f, "0")?;
        }

        Ok(())
    }
}
