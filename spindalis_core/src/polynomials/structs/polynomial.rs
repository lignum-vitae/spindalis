use crate::derivatives::polynomial::partial_derivative;
use crate::integrals::polynomial_indefinite::indefinite_integral_polynomial;
use crate::polynomials::polynomial::{eval_polynomial, parse_polynomial};
use crate::polynomials::structs::PolynomialTraits;
use crate::polynomials::{PolynomialError, Term};

#[derive(Debug, PartialEq)]
pub struct Polynomial {
    pub terms: Vec<Term>,
    pub variables: Vec<String>,
}

impl Polynomial {
    pub fn sort_poly(&mut self) {
        // Sort variables inside each individual term
        for term in &mut self.terms {
            term.variables.sort_by(|a, b| a.0.cmp(&b.0));
        }
        // Sort the variables list
        self.variables.sort();
    }
}

impl PartialEq<Vec<Term>> for Polynomial {
    fn eq(&self, other: &Vec<Term>) -> bool {
        self.terms == other.clone()
    }
}

impl PolynomialTraits for Polynomial {
    fn parse(input: &str) -> Result<Polynomial, PolynomialError> {
        parse_polynomial(input)
    }

    fn eval_univariate<F>(&self, point: F) -> Result<f64, PolynomialError>
    where
        F: Into<f64> + std::clone::Clone + std::fmt::Debug,
    {
        if self.variables.len() > 1 {
            return Err(PolynomialError::TooManyVariables {
                variables: self.variables.clone(),
            });
        }
        let evaluated = eval_polynomial(&self.terms, &[(self.variables[0].clone(), point)])?;
        Ok(evaluated)
    }

    fn eval_multivariate<V, S, F>(&self, vars: &V) -> Result<f64, PolynomialError>
    where
        V: IntoIterator<Item = (S, F)> + std::fmt::Debug + Clone,
        S: AsRef<str>,
        F: Into<f64>,
    {
        let evaluated = eval_polynomial(&self.terms, vars)?;
        Ok(evaluated)
    }
    fn derivate_univariate(&self) -> Result<Self, PolynomialError> {
        if self.variables.len() > 1 {
            return Err(PolynomialError::TooManyVariables {
                variables: self.variables.clone(),
            });
        }
        Ok(Self {
            terms: partial_derivative(&self.terms, &self.variables[0]).terms,
            variables: self.variables.clone(),
        })
    }
    fn indefinite_integral_univariate(&self) -> Result<Self, PolynomialError> {
        if self.variables.len() > 1 {
            return Err(PolynomialError::TooManyVariables {
                variables: self.variables.clone(),
            });
        }
        let var = if self.variables.is_empty() {
            "x"
        } else {
            &self.variables[0]
        };

        let integrated_poly = indefinite_integral_polynomial(&self.terms, var);

        Ok(Self {
            terms: integrated_poly.terms,
            variables: integrated_poly.variables,
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

    fn indefinite_integral_multivariate<S>(&self, var: S) -> Self
    where
        S: AsRef<str>,
    {
        let integrated_poly = indefinite_integral_polynomial(&self.terms, var);
        Self {
            terms: integrated_poly.terms,
            variables: integrated_poly.variables,
        }
    }
}
impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.terms.is_empty() {
            return write!(f, "0");
        }

        for (i, term) in self.terms.iter().enumerate() {
            // 1. Handle the operator (+ or -) between terms
            if i > 0 {
                if term.coefficient >= 0.0 {
                    write!(f, " + ")?;
                } else {
                    write!(f, " - ")?;
                }
            } else if term.coefficient < 0.0 {
                write!(f, "-")?;
            }

            // 2. Format the coefficient magnitude
            let abs_coeff = term.coefficient.abs();
            let has_vars = !term.variables.is_empty();

            // Only print coefficient if it's not 1.0, or if it's a constant term
            if abs_coeff != 1.0 || !has_vars {
                write!(f, "{}", abs_coeff)?;
            }

            // 3. Format the variables for this specific term
            for (var_name, exponent) in &term.variables {
                write!(f, "{}", var_name)?;
                if *exponent != 1.0 {
                    write!(f, "^{}", exponent)?;
                }
            }
        }

        Ok(())
    }
}
