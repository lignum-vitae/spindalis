#[cfg(test)]
mod tests {
    use spindalis::polynomials::{PolynomialExtended, PolynomialTraits, SimplePolynomial};

    #[test]
    fn test_known_indefinite() {
        let parsed = PolynomialExtended::parse("x ^ 3 - x").unwrap();
        let result = parsed.indefinite_integral_univariate().unwrap();
        let expected = PolynomialExtended::parse("1/4x^4 - 1/2x^2").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_simplification() {
        let parsed = SimplePolynomial::parse("8x ^ 3 - 2x").unwrap();
        let result = parsed.indefinite_integral_univariate().unwrap();
        let expected = SimplePolynomial::parse("2x ^ 4 - x ^ 2").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_constant() {
        let parsed = PolynomialExtended::parse("8x ^ 3 - 2x + 6").unwrap();
        let result = parsed.indefinite_integral_univariate().unwrap();
        let expected = PolynomialExtended::parse("2x ^ 4 - x ^ 2 + 6x").unwrap();

        assert_eq!(result, expected);
    }
}
