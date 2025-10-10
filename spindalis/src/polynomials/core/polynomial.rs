pub fn parse_polynomial(input: &str) -> Vec<f64> {
    let normalized = input.replace(" ", "").replace("-", "+-");
    let parts: Vec<&str> = normalized.split('+').filter(|s| !s.is_empty()).collect();

    let mut terms: Vec<(f64, usize)> = Vec::new();
    for part in parts {
        let term = {
            if let Some(x) = part.find('x') {
                let coeff_str = &part[..x];
                let coeff = if coeff_str.is_empty() || coeff_str == "+" {
                    1.0
                } else if coeff_str == "-" {
                    -1.0
                } else {
                    coeff_str.parse::<f64>().unwrap()
                };

                if let Some(pow) = part.find('^') {
                    let pow_str = &part[pow + 1..];
                    let power = pow_str.parse::<usize>().unwrap();
                    (coeff, power)
                } else {
                    // x^1 value
                    (coeff, 1)
                }
            } else {
                // No 'x' aka num is constant
                (part.parse::<f64>().unwrap(), 0)
            }
        };
        terms.push(term);
    }

    let max_power = terms.iter().map(|&(_, power)| power).max().unwrap_or(0);
    let mut coeffs = vec![0.0; max_power + 1];
    for (coeff, power) in terms {
        coeffs[power] += coeff;
    }
    coeffs
}

pub fn eval_polynomial(x: f64, coeffs: &[f64]) -> f64 {
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
        let coeffs = parse_polynomial("2x^2 + 3x + 4");

        assert_eq!(coeffs.len(), 3);
        assert_eq!(coeffs[0], 4.0); // constant term
        assert_eq!(coeffs[1], 3.0); // x^1 term
        assert_eq!(coeffs[2], 2.0); // x^2 term
    }

    #[test]
    fn test_parse_polynomial_negative_coeffs() {
        let coeffs = parse_polynomial("-2x^3 - 4x + 1");

        assert_eq!(coeffs.len(), 4);
        assert_eq!(coeffs[0], 1.0); // constant
        assert_eq!(coeffs[1], -4.0); // x^1
        assert_eq!(coeffs[2], 0.0); // x^2 missing → 0
        assert_eq!(coeffs[3], -2.0); // x^3
    }

    #[test]
    fn test_parse_polynomial_implicit_coeff() {
        let coeffs = parse_polynomial("x^2 + x + 1");

        assert_eq!(coeffs.len(), 3);
        assert_eq!(coeffs[0], 1.0);
        assert_eq!(coeffs[1], 1.0);
        assert_eq!(coeffs[2], 1.0);
    }

    #[test]
    fn test_parse_polynomial_constants_only() {
        let coeffs = parse_polynomial("5");

        assert_eq!(coeffs.len(), 1);
        assert_eq!(coeffs[0], 5.0);
    }

    #[test]
    fn test_parse_polynomial_missing_powers() {
        let coeffs = parse_polynomial("2x + 3");

        assert_eq!(coeffs.len(), 2);
        assert_eq!(coeffs[0], 3.0);
        assert_eq!(coeffs[1], 2.0);
    }

    #[test]
    fn test_parse_polynomial_multiple_terms_same_power() {
        let coeffs = parse_polynomial("2x^2 + 3x^2");

        assert_eq!(coeffs.len(), 3);
        assert_eq!(coeffs[0], 0.0); // constant
        assert_eq!(coeffs[1], 0.0); // x^1 missing
        assert_eq!(coeffs[2], 5.0); // x^2 term: 2+3
    }

    #[test]
    fn test_eval_polynomial_simple() {
        let coeffs = parse_polynomial("2x^2 + 3x + 4");
        let result = eval_polynomial(2.0, &coeffs);

        // 2*4 + 3*2 + 4 = 8 + 6 + 4 = 18
        assert_eq!(result, 18.0);
    }

    #[test]
    fn test_eval_polynomial_negative() {
        let coeffs = parse_polynomial("-x^2 + 4x - 5");
        let result = eval_polynomial(3.0, &coeffs);

        // -9 + 12 - 5 = -2
        assert_eq!(result, -2.0);
    }

    #[test]
    fn test_eval_polynomial_constant() {
        let coeffs = parse_polynomial("7");
        let result = eval_polynomial(10.0, &coeffs);

        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_parse_and_eval_combined() {
        let expr = "x^3 - 2x + 1";
        let coeffs = parse_polynomial(expr);

        let result_at_2 = eval_polynomial(2.0, &coeffs);
        // 8 - 4 + 1 = 5
        assert_eq!(result_at_2, 5.0);

        let result_at_0 = eval_polynomial(0.0, &coeffs);
        assert_eq!(result_at_0, 1.0);
    }
}
