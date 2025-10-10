use crate::polynomials::Term;

pub fn parse_multivar(expr: &str, ascii_letters: &str) -> Option<Vec<Term>> {
    let normalized = expr.replace(" ", "").replace("-", "+-");
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
            coeff.parse::<f64>().unwrap_or(0.0)
        };

        while let Some(ch) = chars.next() {
            pow_str.clear();
            if ascii_letters.contains(ch) {
                let var = ch.to_string();
                let mut power = 1; // default power
                if let Some('^') = chars.peek() {
                    chars.next(); // consume '^'
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_ascii_digit() {
                            pow_str.push(next_char);
                            chars.next(); // consume current digit
                        } else {
                            break;
                        }
                    }
                    if let Ok(pow) = pow_str.parse::<usize>() {
                        power = pow
                    } else {
                        // Return none if a positive integer is not found
                        return None;
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
    Some(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ASCII_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    #[test]
    fn test_parse_single_variable() {
        let expr = "3x^2";
        let result = parse_multivar(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 3.0);
        assert_eq!(result[0].variables, vec![("x".into(), 2)]);
    }

    #[test]
    fn test_parse_multiple_variables() {
        let expr = "4x^2y^3";
        let result = parse_multivar(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 4.0);
        assert_eq!(result[0].variables, vec![("x".into(), 2), ("y".into(), 3)]);
    }

    #[test]
    fn test_parse_no_coefficient() {
        let expr = "x^3";
        let result = parse_multivar(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 1.0);
        assert_eq!(result[0].variables, vec![("x".into(), 3)]);
    }

    #[test]
    fn test_parse_negative_coefficient() {
        let expr = "-2x^2";
        let result = parse_multivar(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, -2.0);
        assert_eq!(result[0].variables, vec![("x".into(), 2)]);
    }

    #[test]
    fn test_parse_negative_variable() {
        let expr = "-x^2";
        let result = parse_multivar(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, -1.0);
        assert_eq!(result[0].variables, vec![("x".into(), 2)]);
    }

    #[test]
    fn test_parse_multiple_terms() {
        let expr = "2x^2+3y-4z^3";
        let result = parse_multivar(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 3);

        assert_eq!(result[0].coefficient, 2.0);
        assert_eq!(result[0].variables, vec![("x".into(), 2)]);

        assert_eq!(result[1].coefficient, 3.0);
        assert_eq!(result[1].variables, vec![("y".into(), 1)]);

        assert_eq!(result[2].coefficient, -4.0);
        assert_eq!(result[2].variables, vec![("z".into(), 3)]);
    }

    #[test]
    fn test_parse_missing_power_defaults_to_one() {
        let expr = "5x";
        let result = parse_multivar(expr, ASCII_LETTERS).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 5.0);
        assert_eq!(result[0].variables, vec![("x".into(), 1)]);
    }

    #[test]
    fn test_parse_invalid_power_returns_none() {
        let expr = "2x^a";
        let result = parse_multivar(expr, ASCII_LETTERS);

        assert!(result.is_none());
    }
}
