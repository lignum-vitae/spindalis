use crate::polynomials::{ascii_letters, core::PolynomialError};

pub fn parse_simple_polynomial(input: &str) -> Result<Vec<f64>, PolynomialError> {
    let normalized = input.replace(" ", "").replace("-", "+-");
    let mut parts: Vec<&str> = normalized.split('+').collect();

    // Handles instance of the first value of poly being negative
    // Prevents throwing syntax error for "-x + 4" etc
    if parts.first() == Some(&"") {
        parts.remove(0);
    }

    if parts.iter().any(|s| s.is_empty()) {
        return Err(PolynomialError::PolynomialSyntaxError);
    }

    let letters = ascii_letters();
    // unwrap_or to maintain functionality of parsing just a constant
    // (might replace with '.map_err()?' to bubble up Error)
    let var = normalized
        .chars()
        .find(|&c| letters.contains(c))
        .ok_or(PolynomialError::MissingVariable)?;

    let mut terms: Vec<(f64, usize)> = Vec::new();
    for part in parts {
        let term = {
            if let Some(x) = part.find(var) {
                let coeff_str = &part[..x];
                let coeff = if coeff_str.is_empty() || coeff_str == "+" {
                    1.0
                } else if coeff_str == "-" {
                    -1.0
                } else {
                    coeff_str
                        .parse::<f64>()
                        .map_err(|_| PolynomialError::InvalidCoefficient)?
                };

                if let Some(pow) = part.find('^') {
                    let pow_str = &part[pow + 1..];
                    let power = pow_str
                        .parse::<usize>()
                        .map_err(|_| PolynomialError::InvalidExponent)?;
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
        };
        terms.push(term);
    }

    let max_power = terms.iter().map(|&(_, power)| power).max().unwrap_or(0);
    let mut coeffs = vec![0.0; max_power + 1];
    for (coeff, power) in terms {
        coeffs[power] += coeff;
    }
    Ok(coeffs)
}

pub fn eval_simple_polynomial(x: f64, coeffs: &[f64]) -> f64 {
    coeffs
        .iter()
        .enumerate()
        .map(|(i, &c)| c * x.powi(i as i32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_polynomial_simple() {
        let coeffs = parse_simple_polynomial("2x^2 + 3x + 4").unwrap();

        assert_eq!(coeffs.len(), 3);
        assert_eq!(coeffs[0], 4.0); // constant term
        assert_eq!(coeffs[1], 3.0); // x^1 term
        assert_eq!(coeffs[2], 2.0); // x^2 term
    }

    #[test]
    fn test_parse_polynomial_negative_coeffs() {
        let coeffs = parse_simple_polynomial("-2x^3 - 4x + 1").unwrap();

        assert_eq!(coeffs.len(), 4);
        assert_eq!(coeffs[0], 1.0); // constant
        assert_eq!(coeffs[1], -4.0); // x^1
        assert_eq!(coeffs[2], 0.0); // x^2 missing → 0
        assert_eq!(coeffs[3], -2.0); // x^3
    }

    #[test]
    fn test_parse_polynomial_implicit_coeff() {
        let coeffs = parse_simple_polynomial("x^2 + x + 1").unwrap();

        assert_eq!(coeffs.len(), 3);
        assert_eq!(coeffs[0], 1.0);
        assert_eq!(coeffs[1], 1.0);
        assert_eq!(coeffs[2], 1.0);
    }

    #[test]
    fn test_parse_polynomial_constants_only() {
        let result = parse_simple_polynomial("5");
        assert!(matches!(result, Err(PolynomialError::MissingVariable)));
    }

    #[test]
    fn test_parse_polynomial_missing_powers() {
        let coeffs = parse_simple_polynomial("2x + 3").unwrap();

        assert_eq!(coeffs.len(), 2);
        assert_eq!(coeffs[0], 3.0);
        assert_eq!(coeffs[1], 2.0);
    }

    #[test]
    fn test_parse_polynomial_multiple_terms_same_power() {
        let coeffs = parse_simple_polynomial("2x^2 + 3x^2").unwrap();

        assert_eq!(coeffs.len(), 3);
        assert_eq!(coeffs[0], 0.0); // constant
        assert_eq!(coeffs[1], 0.0); // x^1 missing
        assert_eq!(coeffs[2], 5.0); // x^2 term: 2+3
    }

    #[test]
    fn test_eval_polynomial_simple() {
        let coeffs = parse_simple_polynomial("2x^2 + 3x + 4").unwrap();
        let result = eval_simple_polynomial(2.0, &coeffs);

        // 2*4 + 3*2 + 4 = 8 + 6 + 4 = 18
        assert_eq!(result, 18.0);
    }

    #[test]
    fn test_eval_polynomial_negative() {
        let coeffs = parse_simple_polynomial("-x^2 + 4x - 5").unwrap();
        let result = eval_simple_polynomial(3.0, &coeffs);

        // -9 + 12 - 5 = -2
        assert_eq!(result, -2.0);
    }

    #[test]
    fn test_parse_polynomial_constant_fails() {
        let result = parse_simple_polynomial("7");
        assert!(matches!(result, Err(PolynomialError::MissingVariable)));
    }

    #[test]
    fn test_parse_and_eval_combined() {
        let expr = "x^3 - 2x + 1";
        let coeffs = parse_simple_polynomial(expr).unwrap();

        let result_at_2 = eval_simple_polynomial(2.0, &coeffs);
        // 8 - 4 + 1 = 5
        assert_eq!(result_at_2, 5.0);

        let result_at_0 = eval_simple_polynomial(0.0, &coeffs);
        assert_eq!(result_at_0, 1.0);
    }
}
