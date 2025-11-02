pub fn simple_derivative(poly: &[f64]) -> Vec<f64> {
    let mut deriv = Vec::with_capacity(poly.len().saturating_sub(1));
    for (power, &coeff) in poly.iter().enumerate().skip(1) {
        deriv.push(coeff * power as f64);
    }
    deriv
}

#[cfg(test)]
mod tests {
    use super::*;
    use spindalis_core::polynomials::PolynomialError;
    use spindalis_core::polynomials::simple::{eval_simple_polynomial, parse_simple_polynomial};
    use spindalis_derive::parse_simple_polynomial;

    #[test]
    fn test_derivative_simple() {
        let poly = vec![4.0, 3.0, 2.0]; // 4 + 3x + 2x^2
        let deriv = simple_derivative(&poly);

        // simple_derivative: 3 + 4x => [3.0, 4.0]
        assert_eq!(deriv, vec![3.0, 4.0]);
    }

    #[test]
    fn test_derivative_constant() {
        let poly = vec![5.0]; // 5
        let deriv = simple_derivative(&poly);

        assert!(deriv.is_empty()); // simple_derivative of constant is zero-length
    }

    #[test]
    fn test_derivative_linear() {
        let poly = vec![2.0, 3.0]; // 2 + 3x
        let deriv = simple_derivative(&poly);

        assert_eq!(deriv, vec![3.0]); // simple_derivative: 3
    }

    #[test]
    fn test_derivative_zero_poly() {
        let poly = vec![];
        let deriv = simple_derivative(&poly);

        assert!(deriv.is_empty());
    }

    #[test]
    fn test_derivative_higher_degree() {
        let poly = vec![1.0, -4.0, 3.0, 2.0]; // 1 - 4x + 3x^2 + 2x^3
        let deriv = simple_derivative(&poly);

        // simple_derivative: -4 + 6x + 6x^2 => [-4.0, 6.0, 6.0]
        assert_eq!(deriv, vec![-4.0, 6.0, 6.0]);
    }

    #[test]
    fn test_derivative_with_zero_coefficients() {
        let poly = vec![0.0, 0.0, 5.0]; // 0 + 0x + 5x^2
        let deriv = simple_derivative(&poly);

        assert_eq!(deriv, vec![0.0, 10.0]);
    }

    #[test]
    fn test_derivative_large_coeffs() {
        let poly = vec![1e6, -2e6, 3e6]; // 1e6 - 2e6x + 3e6x^2
        let deriv = simple_derivative(&poly);

        assert_eq!(deriv, vec![-2e6, 6e6]);
    }

    #[test]
    fn test_parse_simple_polynomial_simple() {
        let coeffs = parse_simple_polynomial("2x^2 + 3x + 4").unwrap();
        let coeffs_macro = parse_simple_polynomial!(2 x ^ 2 + 3 x + 4).to_vec();
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
        let coeffs_macro = parse_simple_polynomial!(-2 x ^3 - 4 x + 1).to_vec();

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
        let coeffs_macro = parse_simple_polynomial!(x ^ 2 + x + 1).to_vec();

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
        let coeffs_macro = parse_simple_polynomial!(2 x + 3).to_vec();

        let result = vec![3.0, 2.0];
        assert_eq!(coeffs, result);
        assert_eq!(coeffs_macro, result);
    }

    #[test]
    fn test_parse_simple_polynomial_multiple_terms_same_power() {
        let coeffs = parse_simple_polynomial("2x^2+3x^2").unwrap();
        let coeffs_macro = parse_simple_polynomial!(2 x^2 + 3 x^2).to_vec();

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
        let coeffs_macro = parse_simple_polynomial!(2 x^2 + 3 x + 4).to_vec();
        assert_eq!(coeffs, coeffs_macro);

        let result = eval_simple_polynomial(2.0, &coeffs);

        // 2*4 + 3*2 + 4 = 8 + 6 + 4 = 18
        assert_eq!(result, 18.0);
    }

    #[test]
    fn test_eval_polynomial_negative() {
        let coeffs = parse_simple_polynomial("-x^2 + 4x - 5").unwrap();
        let coeffs_macro = parse_simple_polynomial!(-x ^ 2 + 4 x - 5).to_vec();
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
