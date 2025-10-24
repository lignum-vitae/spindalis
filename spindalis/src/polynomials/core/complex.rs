use crate::polynomials::{Term, core::ComplexPolyErr};

static SPECIAL_CHARS: &[char] = &['.', '/', '-'];

pub fn parse_complex_poly(expr: &str, ascii_letters: &str) -> Result<Vec<Term>, ComplexPolyErr> {
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
            if ch.is_numeric() || ch == '.' || (coeff.is_empty() && ch == '-') {
                coeff.push(ch);
                chars.next();
            } else {
                break;
            }
        }

        // If no explicit coefficient (e.g., "-x"), set it to 1 or -1
        let coeff = if coeff.is_empty() || coeff == "-" {
            if coeff == "-" { -1.0 } else { 1.0 }
        } else {
            let parsed = coeff.parse::<f64>();
            match parsed {
                Ok(x) => x,
                Err(_) => return Err(ComplexPolyErr::InvalidCoefficient { coeff }),
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
                            return Err(ComplexPolyErr::InvalidFractionalExponent { pow: pow_str });
                        }
                        match (fraction[0].parse::<f64>(), fraction[1].parse::<f64>()) {
                            (Ok(x), Ok(y)) if y != 0.0 => power = x / y,
                            _ => {
                                return Err(ComplexPolyErr::InvalidFractionalExponent {
                                    pow: pow_str,
                                });
                            }
                        }
                    } else if let Ok(pow) = pow_str.parse::<f64>() {
                        power = pow
                    } else {
                        return Err(ComplexPolyErr::InvalidExponent { pow: pow_str });
                    };
                };
                vars.push((var, power));
            }
        }
        parsed.push(Term {
            coefficient: coeff,
            variables: vars.clone(),
        });
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ASCII_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    // test positive ints

    #[test]
    fn test_parse_single_variable() {
        let expr = "3x^2";
        let result = parse_complex_poly(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 3.0);
        assert_eq!(result[0].variables, vec![("x".into(), 2.0)]);
    }

    #[test]
    fn test_parse_multiple_variables() {
        let expr = "4x^2y^3";
        let result = parse_complex_poly(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 4.0);
        assert_eq!(
            result[0].variables,
            vec![("x".into(), 2.0), ("y".into(), 3.0)]
        );
    }

    #[test]
    fn test_parse_no_coefficient() {
        let expr = "x^3";
        let result = parse_complex_poly(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 1.0);
        assert_eq!(result[0].variables, vec![("x".into(), 3.0)]);
    }

    #[test]
    fn test_parse_negative_coefficient() {
        let expr = "-2x^2";
        let result = parse_complex_poly(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, -2.0);
        assert_eq!(result[0].variables, vec![("x".into(), 2.0)]);
    }

    #[test]
    fn test_parse_negative_variable() {
        let expr = "-x^2";
        let result = parse_complex_poly(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, -1.0);
        assert_eq!(result[0].variables, vec![("x".into(), 2.0)]);
    }

    #[test]
    fn test_parse_multiple_terms() {
        let expr = "2x^2+3y-4z^3";
        let result = parse_complex_poly(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 3);

        assert_eq!(result[0].coefficient, 2.0);
        assert_eq!(result[0].variables, vec![("x".into(), 2.0)]);

        assert_eq!(result[1].coefficient, 3.0);
        assert_eq!(result[1].variables, vec![("y".into(), 1.0)]);

        assert_eq!(result[2].coefficient, -4.0);
        assert_eq!(result[2].variables, vec![("z".into(), 3.0)]);
    }

    #[test]
    fn test_parse_missing_power_defaults_to_one() {
        let expr = "5x";
        let result = parse_complex_poly(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 5.0);
        assert_eq!(result[0].variables, vec![("x".into(), 1.0)]);
    }

    #[test]
    fn test_parse_invalid_power_returns_none() {
        let expr = "2x^a";
        let result = parse_complex_poly(expr, ASCII_LETTERS);

        assert!(result.is_err());
    }

    // test floats

    #[test]
    fn test_parse_pos_decimal() {
        let expr = "5x^0.5";
        let result = parse_complex_poly(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 5.0);
        assert_eq!(result[0].variables, vec![("x".into(), 0.5)]);
    }

    #[test]
    fn test_parse_neg_decimal() {
        let expr = "5x^-0.5";
        let result = parse_complex_poly(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 5.0);
        assert_eq!(result[0].variables, vec![("x".into(), -0.5)]);
    }

    #[test]
    fn test_parse_err_decimal() {
        let expr = "5x^-0.5.0";
        let result = parse_complex_poly(expr, ASCII_LETTERS);

        assert!(result.is_err());
    }

    // test fractions

    #[test]
    fn test_parse_fraction() {
        let expr = "5x^1/2";
        let result = parse_complex_poly(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 5.0);
        assert_eq!(result[0].variables, vec![("x".into(), 0.5)]);
    }

    #[test]
    fn test_parse_float_fraction() {
        let expr = "5x^0.5/1.0";
        let result = parse_complex_poly(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 5.0);
        assert_eq!(result[0].variables, vec![("x".into(), 0.5)]);
    }

    #[test]
    fn test_parse_err_fraction() {
        let expr = "5x^0.5/1.0/1.0";
        let result = parse_complex_poly(expr, ASCII_LETTERS);

        assert!(result.is_err());
    }
}
