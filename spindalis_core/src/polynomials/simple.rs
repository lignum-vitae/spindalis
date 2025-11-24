use crate::polynomials::PolynomialError;

pub fn parse_simple_polynomial<S>(input: S) -> Result<Vec<f64>, PolynomialError>
where
    S: AsRef<str>,
{
    let input = input.as_ref();
    let normalized = input.replace(" ", "").replace("-", "+-");
    let mut parts: Vec<&str> = normalized.split('+').collect();

    // Handles instance of the first value of poly being negative
    // Prevents throwing a syntax error for "-x + 4" etc
    if parts.first() == Some(&"") {
        parts.remove(0);
    }

    // Handles double plusses and double minuses
    if parts.iter().any(|s| s.is_empty() || *s == "-") {
        return Err(PolynomialError::PolynomialSyntaxError);
    }

    // Allowing parsing just a constant in case someone wants to integral a constant
    let variable = normalized.chars().find(|&c| c.is_alphabetic());

    let mut terms: Vec<(f64, usize)> = Vec::new();
    for part in parts {
        let term = if let Some(var) = variable {
            if let Some(x) = part.find(var) {
                let coeff_str = &part[..x];
                let coeff = if coeff_str.is_empty() || coeff_str == "+" {
                    1.0
                } else if coeff_str == "-" {
                    -1.0
                } else {
                    coeff_str
                        .parse::<f64>()
                        .map_err(|_| PolynomialError::InvalidCoefficient {
                            coeff: coeff_str.to_string(),
                        })?
                };

                if let Some(pow) = part.find('^') {
                    let pow_str = &part[pow + 1..];
                    let power =
                        pow_str
                            .parse::<usize>()
                            .map_err(|_| PolynomialError::InvalidExponent {
                                pow: pow_str.to_string(),
                            })?;
                    (coeff, power)
                } else {
                    // x^1 value
                    (coeff, 1)
                }
            } else {
                // No 'x' aka num is constant
                let constant = part
                    .parse::<f64>()
                    .map_err(|_| PolynomialError::InvalidConstant)?;
                (constant, 0)
            }
        } else {
            // No variable (just constant)
            let constant = part
                .parse::<f64>()
                .map_err(|_| PolynomialError::InvalidConstant)?;
            (constant, 0)
        };
        terms.push(term);
    }

    let terms: &[(f64, usize)] = &terms;
    let max_power = terms.iter().map(|&(_, power)| power).max().unwrap_or(0);
    let mut coeffs = vec![0.0; max_power + 1];
    for &(coeff, power) in terms {
        coeffs[power] += coeff;
    }
    Ok(coeffs)
}

pub fn eval_simple_polynomial<F>(x: F, coeffs: &[f64]) -> f64
where
    F: Into<f64>,
{
    let x: f64 = x.into();
    coeffs
        .iter()
        .enumerate()
        .map(|(i, &c)| c * x.powi(i as i32))
        .sum()
}
