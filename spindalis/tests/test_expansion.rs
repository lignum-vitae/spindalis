#[cfg(test)]
mod tests {
    use spindalis::expansion::{
        expand_intermediate, expand_simple, multiply_intermediate, multiply_simple,
    };
    use spindalis::polynomials::{
        parse_intermediate_polynomial, parse_simple_polynomial, IntermediatePolynomial,
        SimplePolynomial, Term,
    };

    // ───── SimplePolynomial tests ─────

    #[test]
    fn test_simple_multiply_binomials() {
        // (x + 1)(x - 1) = x^2 - 1
        let a = parse_simple_polynomial("x + 1").unwrap();
        let b = parse_simple_polynomial("x - 1").unwrap();
        let result = multiply_simple(&a, &b);
        assert_eq!(result.coefficients, vec![-1.0, 0.0, 1.0]);
    }

    #[test]
    fn test_simple_expand_4x_minus_3_cubed() {
        // (4x - 3)^3 = 64x^3 - 144x^2 + 108x - 27
        let p = parse_simple_polynomial("4x - 3").unwrap();
        let result = expand_simple(&p, 3);
        assert_eq!(result.coefficients, vec![-27.0, 108.0, -144.0, 64.0]);
        assert_eq!(format!("{}", result), "64x^3 - 144x^2 + 108x - 27");
    }

    #[test]
    fn test_simple_expand_x_plus_1_squared() {
        // (x + 1)^2 = x^2 + 2x + 1
        let p = parse_simple_polynomial("x + 1").unwrap();
        let result = expand_simple(&p, 2);
        assert_eq!(result.coefficients, vec![1.0, 2.0, 1.0]);
    }

    #[test]
    fn test_simple_expand_power_zero() {
        let p = parse_simple_polynomial("3x^2 + 2x - 5").unwrap();
        let result = expand_simple(&p, 0);
        assert_eq!(result.coefficients, vec![1.0]);
    }

    #[test]
    fn test_simple_expand_power_one() {
        let p = parse_simple_polynomial("3x^2 + 2x - 5").unwrap();
        let result = expand_simple(&p, 1);
        assert_eq!(result.coefficients, vec![-5.0, 2.0, 3.0]);
    }

    #[test]
    fn test_simple_expand_trinomial_fourth() {
        // (x^2 + x + 1)^2 = x^4 + 2x^3 + 3x^2 + 2x + 1
        let p = parse_simple_polynomial("x^2 + x + 1").unwrap();
        let result = expand_simple(&p, 2);
        assert_eq!(result.coefficients, vec![1.0, 2.0, 3.0, 2.0, 1.0]);
    }

    #[test]
    fn test_simple_expand_2x_plus_1_fourth() {
        // (2x + 1)^4 = 16x^4 + 32x^3 + 24x^2 + 8x + 1
        let p = parse_simple_polynomial("2x + 1").unwrap();
        let result = expand_simple(&p, 4);
        assert_eq!(result.coefficients, vec![1.0, 8.0, 24.0, 32.0, 16.0]);
    }

    // ───── IntermediatePolynomial tests ─────

    fn find_coeff(poly: &IntermediatePolynomial, vars: Vec<(&str, f64)>) -> f64 {
        let target: Vec<(String, f64)> = vars
            .into_iter()
            .map(|(n, e)| (n.to_string(), e))
            .collect();
        poly.terms
            .iter()
            .find(|t| t.variables == target)
            .map(|t| t.coefficient)
            .unwrap_or(0.0)
    }

    #[test]
    fn test_intermediate_multiply_binomials() {
        // (x + 1)(x - 1) = x^2 - 1
        let a = parse_intermediate_polynomial("x + 1").unwrap();
        let b = parse_intermediate_polynomial("x - 1").unwrap();
        let result = multiply_intermediate(&a, &b);

        assert_eq!(find_coeff(&result, vec![("x", 2.0)]), 1.0);
        assert_eq!(find_coeff(&result, vec![("x", 1.0)]), 0.0); // cancelled
        assert_eq!(find_coeff(&result, vec![]), -1.0);
    }

    #[test]
    fn test_intermediate_expand_4x_minus_3_cubed() {
        // (4x - 3)^3 = 64x^3 - 144x^2 + 108x - 27
        let p = parse_intermediate_polynomial("4x - 3").unwrap();
        let result = expand_intermediate(&p, 3);

        assert_eq!(find_coeff(&result, vec![("x", 3.0)]), 64.0);
        assert_eq!(find_coeff(&result, vec![("x", 2.0)]), -144.0);
        assert_eq!(find_coeff(&result, vec![("x", 1.0)]), 108.0);
        assert_eq!(find_coeff(&result, vec![]), -27.0);
    }

    #[test]
    fn test_intermediate_expand_multivariate_squared() {
        // (x + y)^2 = x^2 + 2xy + y^2
        let p = parse_intermediate_polynomial("x + y").unwrap();
        let result = expand_intermediate(&p, 2);

        assert_eq!(find_coeff(&result, vec![("x", 2.0)]), 1.0);
        assert_eq!(find_coeff(&result, vec![("x", 1.0), ("y", 1.0)]), 2.0);
        assert_eq!(find_coeff(&result, vec![("y", 2.0)]), 1.0);
    }

    #[test]
    fn test_intermediate_expand_multivariate_cubed() {
        // (x + y)^3 = x^3 + 3x^2y + 3xy^2 + y^3
        let p = parse_intermediate_polynomial("x + y").unwrap();
        let result = expand_intermediate(&p, 3);

        assert_eq!(find_coeff(&result, vec![("x", 3.0)]), 1.0);
        assert_eq!(find_coeff(&result, vec![("x", 2.0), ("y", 1.0)]), 3.0);
        assert_eq!(find_coeff(&result, vec![("x", 1.0), ("y", 2.0)]), 3.0);
        assert_eq!(find_coeff(&result, vec![("y", 3.0)]), 1.0);
    }

    #[test]
    fn test_intermediate_expand_three_variable_squared() {
        // (x + y + z)^2 = x^2 + y^2 + z^2 + 2xy + 2xz + 2yz
        let p = parse_intermediate_polynomial("x + y + z").unwrap();
        let result = expand_intermediate(&p, 2);

        assert_eq!(find_coeff(&result, vec![("x", 2.0)]), 1.0);
        assert_eq!(find_coeff(&result, vec![("y", 2.0)]), 1.0);
        assert_eq!(find_coeff(&result, vec![("z", 2.0)]), 1.0);
        assert_eq!(find_coeff(&result, vec![("x", 1.0), ("y", 1.0)]), 2.0);
        assert_eq!(find_coeff(&result, vec![("x", 1.0), ("z", 1.0)]), 2.0);
        assert_eq!(find_coeff(&result, vec![("y", 1.0), ("z", 1.0)]), 2.0);
    }

    #[test]
    fn test_intermediate_expand_power_zero() {
        let p = parse_intermediate_polynomial("4x - 3").unwrap();
        let result = expand_intermediate(&p, 0);
        assert_eq!(result.terms.len(), 1);
        assert_eq!(result.terms[0].coefficient, 1.0);
        assert!(result.terms[0].variables.is_empty());
    }
}
