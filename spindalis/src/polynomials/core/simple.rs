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

    Ok(order_polynomial(&terms))
}

// eg: 3x^2 + 4x + 2 => [2, 4, 3]
pub fn order_polynomial(terms: &[(f64, usize)]) -> Vec<f64> {
    let max_power = terms.iter().map(|&(_, power)| power).max().unwrap_or(0);
    let mut coeffs = vec![0.0; max_power + 1];
    for &(coeff, power) in terms {
        coeffs[power] += coeff;
    }
    coeffs
}

#[macro_export]
macro_rules! parse_simple_polynomial {
    // entry point of the macro
    ($($polynomial:tt)*) => {
        $crate::polynomials::core::simple::order_polynomial(
            &$crate::_parse_simple_polynomial!(
                !entry $($polynomial)*
            )
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _parse_simple_polynomial {
    // first entry can omit the operation (+/-)
    // no coeff, only power
    (!entry $term:ident ^ $power:literal $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [(1, $power)] $($polynomial)*)
    };
    (!entry - $term:ident ^ $power:literal $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [(-1, $power)] $($polynomial)*)
    };
    // no coeff, power implicit
    (!entry $term:ident $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [(1, $power)] $($polynomial)*)
    };
    (!entry - $term:ident $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [(-1, $power)] $($polynomial)*)
    };
    // coeff and power
    (!entry $coeff:literal $term:ident ^ $power:literal  $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [($coeff, $power)] $($polynomial)*)
    };
    // coeff, power implicit
    (!entry $coeff:literal $term:ident $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [($coeff, 1)] $($polynomial)*)
    };
    // coeff, no power
    (!entry $coeff:literal $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [($coeff, 0)] $($polynomial)*)
    };

    // no coeff, only power
    (!parse [$(($parsed_coeff:literal, $parsed_power:literal)),*] - $term:ident ^ $power:literal $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [$(($parsed_coeff, $parsed_power)),* , (-1, $power)] $($polynomial)*)
    };
    // no coeff, only power with subtaction
    (!parse [$(($parsed_coeff:literal, $parsed_power:literal)),*] + $term:ident ^ $power:literal $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [$(($parsed_coeff, $parsed_power)),* , (1, $power)] $($polynomial)*)
    };

    // no coeff, power implicit
    (!parse [$(($parsed_coeff:literal, $parsed_power:literal)),*] + $term:ident $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [$(($parsed_coeff, $parsed_power)),* , (1, 1)] $($polynomial)*)
    };
    // no coeff, power implicit with sutraction
    (!parse [$(($parsed_coeff:literal, $parsed_power:literal)),*] - $term:ident $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [$(($parsed_coeff, $parsed_power)),* , (-1, 1)] $($polynomial)*)
    };

    // coeff and power
    (!parse [$(($parsed_coeff:literal, $parsed_power:literal)),*] + $coeff:literal $term:ident ^ $power:literal  $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [$(($parsed_coeff, $parsed_power)),* , ($coeff, $power)] $($polynomial)*)
    };
    // coeff and power with sutraction
    (!parse [$(($parsed_coeff:literal, $parsed_power:literal)),*] - $coeff:literal $term:ident ^ $power:literal  $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [$(($parsed_coeff, $parsed_power)),* , (-$coeff, $power)] $($polynomial)*)
    };

    // coeff, power implicit
    (!parse [$(($parsed_coeff:literal, $parsed_power:literal)),*] + $coeff:literal $term:ident $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [$(($parsed_coeff, $parsed_power)),* , ($coeff, 1)] $($polynomial)*)
    };
    // coeff, power implicit with sutraction
    (!parse [$(($parsed_coeff:literal, $parsed_power:literal)),*] - $coeff:literal $term:ident $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [$(($parsed_coeff, $parsed_power)),* , (-$coeff, 1)] $($polynomial)*)
    };

    // coeff, no power
    (!parse [$(($parsed_coeff:literal, $parsed_power:literal)),*] + $coeff:literal $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [$(($parsed_coeff, $parsed_power)),* , ($coeff, 0)] $($polynomial)*)
    };
    // coeff, no power with sutraction
    (!parse [$(($parsed_coeff:literal, $parsed_power:literal)),*] - $coeff:literal $($polynomial:tt)*) => {
        $crate::_parse_simple_polynomial!(!parse [$(($parsed_coeff, $parsed_power)),* , (-$coeff, 0)] $($polynomial)*)
    };

    // no more elements
    (!parse [$(($parsed_coeff:literal, $parsed_power:literal)),*]) => {
        [$(($parsed_coeff as f64, $parsed_power)),*]
    };
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
    fn test_parse_simple_polynomial_simple() {
        let coeffs = parse_simple_polynomial("2x^2 + 3x + 4").unwrap();
        let coeffs_macro = parse_simple_polynomial!(2 x ^ 2 + 3 x + 4);
        let result = vec![
            4.0, // constant term
            3.0, // x^1 term
            2.0, // x^2 term
        ];

        assert_eq!(coeffs, result);
        assert_eq!(coeffs_macro, result);
    }

    #[test]
    fn test_parse_simple_polynomial_negative_coeffs() {
        let coeffs = parse_simple_polynomial("-2x^3 - 4x + 1").unwrap();
        let coeffs_macro = parse_simple_polynomial!(-2 x ^3 - 4 x + 1);

        let result = vec![
            1.0,  // constant term
            -4.0, // x^1 term
            0.0,  // x^2 missing → 0
            -2.0, // x^3 term
        ];
        assert_eq!(coeffs, result);
        assert_eq!(coeffs_macro, result);
    }

    #[test]
    fn test_parse_simple_polynomial_implicit_coeff() {
        let coeffs = parse_simple_polynomial("x^2 + x + 1").unwrap();
        let coeffs_macro = parse_simple_polynomial!(x ^ 2 + x + 1);

        let result = vec![1.0, 1.0, 1.0];
        assert_eq!(coeffs, result);
        assert_eq!(coeffs_macro, result);
    }

    #[test]
    fn test_parse_simple_polynomial_constants_only() {
        let result = parse_simple_polynomial("5");
        
        assert!(matches!(result, Err(PolynomialError::MissingVariable)));
    }

    #[test]
    fn test_parse_simple_polynomial_missing_powers() {
        let coeffs = parse_simple_polynomial("2x + 3").unwrap();
        let coeffs_macro = parse_simple_polynomial!(2 x + 3);

        let result = vec![3.0, 2.0];
        assert_eq!(coeffs, result);
        assert_eq!(coeffs_macro, result);
    }

    #[test]
    fn test_parse_simple_polynomial_multiple_terms_same_power() {
        let coeffs = parse_simple_polynomial("2x^2+3x^2")
        let coeffs_macro = parse_simple_polynomial!(2 x^2 + 3 x^2);

        let result = vec![
            0.0, // constant missing
            0.0, // x^1 missing
            5.0, // x^2 term: 2+3
        ];
        assert_eq!(coeffs, result);
        assert_eq!(coeffs_macro, result);
    }

    #[test]
    fn test_eval_polynomial_simple() {
        let coeffs = parse_simple_polynomial("2x^2 + 3x + 4").unwrap();
        let coeffs_macro = parse_simple_polynomial!(2 x^2 + 3 x + 4);
        assert_eq!(coeffs, coeffs_macro);

        let result = eval_simple_polynomial(2.0, &coeffs);

        // 2*4 + 3*2 + 4 = 8 + 6 + 4 = 18
        assert_eq!(result, 18.0);
    }

    #[test]
    fn test_eval_polynomial_negative() {
        let coeffs = parse_simple_polynomial("-x^2 + 4x - 5").unwrap();
        let coeffs_macro = parse_simple_polynomial!(-x ^ 2 + 4 x - 5);
        assert_eq!(coeffs, coeffs_macro);

        let result = eval_simple_polynomial(3.0, &coeffs);

        // -9 + 12 - 5 = -2
        assert_eq!(result, -2.0);
    }

    #[test]
    fn test_parse_simple_polynomial_constant_fails() {
        let result = parse_simple_polynomial("7");
        assert!(matches!(result, Err(PolynomialError::MissingVariable)));
    }

    #[test]
    fn test_parse_and_eval_combined() {
        let coeffs = parse_simple_polynomial("x^3 - 2x + 1").unwrap();
        let coeffs_macro = parse_simple_polynomial!(x^3 - 2 x + 1);
        assert_eq!(coeffs, coeffs_macro);

        let result_at_2 = eval_simple_polynomial(2.0, &coeffs);
        // 8 - 4 + 1 = 5
        assert_eq!(result_at_2, 5.0);

        let result_at_0 = eval_simple_polynomial(0.0, &coeffs);
        assert_eq!(result_at_0, 1.0);
    }

    #[test]
    fn test_invalid_polynomial() {
        let poly = "2x^ + 3x"; // invalid syntax
        let parsed = parse_simple_polynomial(poly);
        assert!(matches!(parsed, Err(PolynomialError::InvalidExponent)));
    }

    #[test]
    fn test_invalid_polynomial_2() {
        let poly = "x^2 + +"; // Invalid syntax
        let parsed = parse_simple_polynomial(poly);
        assert!(matches!(
            parsed,
            Err(PolynomialError::PolynomialSyntaxError)
        ));
    }
}
