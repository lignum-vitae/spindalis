#[cfg(test)]
mod tests {
    use spindalis::polynomials::{IntermediatePolynomial, PolynomialTraits, SimplePolynomial};

    #[test]
    fn test_known_indefinite() {
        let parsed = IntermediatePolynomial::parse("x^3 - x").unwrap();
        let result = parsed.indefinite_integral_univariate().unwrap();
        let expected = IntermediatePolynomial::parse("1/4x^4 - 1/2x^2").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_simplification() {
        let parsed = SimplePolynomial::parse("8x^3 - 2x").unwrap();
        let result = parsed.indefinite_integral_univariate().unwrap();
        let expected = SimplePolynomial::parse("2x^4 - x^2").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_constant() {
        let parsed = IntermediatePolynomial::parse("8x^3 - 2x + 6").unwrap();
        let result = parsed.indefinite_integral_univariate().unwrap();
        let expected = IntermediatePolynomial::parse("2x^4 - x^2 + 6x").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_poly_multivariate_func() {
        let parsed = SimplePolynomial::parse("6x^2 + 6x^5").unwrap();
        let result = parsed.indefinite_integral_multivariate("x");
        let expected = SimplePolynomial::parse("2x^3 + x^6").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_poly_multivariate_func_wrong_var() {
        let parsed = SimplePolynomial::parse("6x^2 + 6x^5").unwrap();
        // Returns original poly because SimplePolynomial cannot handle multivariate polys
        let result = parsed.indefinite_integral_multivariate("b");
        let expected = SimplePolynomial::parse("6x^2 + 6x^5").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_intermediate_poly_multivariate_func() {
        let parsed = IntermediatePolynomial::parse("6x^2 + 6x^5 + 5y").unwrap();
        let result = parsed.indefinite_integral_multivariate("x");
        let expected = IntermediatePolynomial::parse("2x^3 + x^6 + 5yx").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_intermediate_poly_multivariate_func_random_var() {
        let parsed = IntermediatePolynomial::parse("6x^2 + 6x^5 + 5y").unwrap();
        let result = parsed.indefinite_integral_multivariate("b");
        let expected = IntermediatePolynomial::parse("6x^2b + 6x^5b + 5yb").unwrap();

        assert_eq!(result, expected);
    }
}
