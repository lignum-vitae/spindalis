use crate::polynomials::{PolynomialError, structs::PolynomialExtended};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    pub coefficient: f64,
    pub variables: Vec<(String, f64)>,
}
impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 1. Print the coefficient if it's not 1.0 (unless there are no variables)
        let has_vars = !self.variables.is_empty();
        if self.coefficient != 1.0 || !has_vars {
            write!(f, "{}", self.coefficient)?;
        }

        // 2. Print variables and their exponents
        for (var, exp) in &self.variables {
            write!(f, "{}", var)?;
            if *exp != 1.0 {
                write!(f, "^{}", exp)?;
            }
        }
        Ok(())
    }
}

static SPECIAL_CHARS: &[char] = &['.', '/', '-'];

pub fn parse_polynomial_extended<S>(expr: S) -> Result<PolynomialExtended, PolynomialError>
where
    S: AsRef<str>,
{
    let expr = expr.as_ref();
    let ascii_letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let normalized = expr
        .replace(" ", "")
        .replace("^-", "^@")
        .replace("-", "+-")
        .replace("^@", "^-"); // ^- -> ^@ protects negative exponents
    let parts: Vec<&str> = normalized.split('+').filter(|s| !s.is_empty()).collect();

    let mut parsed = Vec::new();
    let mut coeff = String::new();
    let mut vars = Vec::new();
    let mut pow_str = String::new();

    for part in parts {
        coeff.clear();
        vars.clear();
        // Add ability to look at next char without consuming
        let mut chars = part.chars().peekable();

        // Extract coefficient
        while let Some(&ch) = chars.peek() {
            if ch.is_numeric() || ch == '.' || (coeff.is_empty() && ch == '-') || ch == '/' {
                coeff.push(ch);
                chars.next();
            } else {
                break;
            }
        }

        // If no explicit coefficient (e.g., "-x" instead of "-1x"), set it to 1 or -1
        let coeff = if coeff.is_empty() || coeff == "-" {
            if coeff == "-" { -1.0 } else { 1.0 }
        } else if coeff.contains('/') {
            let fraction: Vec<&str> = coeff.split('/').collect();
            if fraction.len() != 2 {
                return Err(PolynomialError::InvalidFraction { frac: coeff });
            }
            match (fraction[0].parse::<f64>(), fraction[1].parse::<f64>()) {
                (Ok(x), Ok(y)) if y != 0.0 => x / y,
                _ => {
                    return Err(PolynomialError::InvalidFraction { frac: coeff });
                }
            }
        } else {
            let parsed = coeff.parse::<f64>();
            match parsed {
                Ok(x) => x,
                Err(_) => return Err(PolynomialError::InvalidCoefficient { coeff }),
            }
        };

        while let Some(ch) = chars.next() {
            pow_str.clear();
            if ascii_letters.contains(ch) {
                let var = ch.to_string();
                let mut power = 1.0; // default power
                if let Some('^') = chars.peek() {
                    chars.next(); // consume '^'
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_ascii_digit() {
                            pow_str.push(next_char);
                            chars.next(); // consume current digit
                        } else if SPECIAL_CHARS.contains(&next_char) {
                            // Handles fractions and decimals in exponent
                            pow_str.push(next_char);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if pow_str.contains('/') {
                        let fraction: Vec<&str> = pow_str.split('/').collect();
                        if fraction.len() != 2 {
                            return Err(PolynomialError::InvalidFractionalExponent {
                                pow: pow_str,
                            });
                        }
                        match (fraction[0].parse::<f64>(), fraction[1].parse::<f64>()) {
                            (Ok(x), Ok(y)) if y != 0.0 => power = x / y,
                            _ => {
                                return Err(PolynomialError::InvalidFractionalExponent {
                                    pow: pow_str,
                                });
                            }
                        }
                    } else if let Ok(pow) = pow_str.parse::<f64>() {
                        power = pow
                    } else {
                        return Err(PolynomialError::InvalidExponent { pow: pow_str });
                    };
                };
                vars.push((var, power));
            }
        }
        vars.sort_by(|a, b| a.0.cmp(&b.0));
        parsed.push(Term {
            coefficient: coeff,
            variables: vars.clone(),
        });
    }
    let unique_variables: HashSet<String> = parsed
        .iter()
        .flat_map(|term| term.variables.iter())
        .map(|(var_name, _)| var_name.clone())
        .collect();
    let mut variables: Vec<String> = unique_variables.into_iter().collect();
    variables.sort();
    Ok(PolynomialExtended {
        terms: parsed,
        variables,
    })
}

pub fn eval_polynomial_extended<V, S, F>(terms: &[Term], vars: &V) -> Result<f64, PolynomialError>
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

    let mut result = 0.0;

    for term in terms {
        let mut term_value = term.coefficient;

        for (var, pow) in &term.variables {
            if let Some(value) = vars_map.get(var) {
                term_value *= value.powf(*pow);
            } else {
                return Err(PolynomialError::VariableNotFound {
                    variable: var.to_string(),
                });
            }
        }
        result += term_value;
    }
    Ok(result)
}
