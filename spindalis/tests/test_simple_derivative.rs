#[cfg(test)]
mod tests {
    use spindalis::derivatives::simple_derivative;
    use spindalis::polynomials::{PolynomialTraits, SimplePolynomial};

    #[test]
    fn test_derivative_simple() {
        let poly = SimplePolynomial::parse("2x^2 + 3x + 4").unwrap(); // 4 + 3x + 2x^2
        let deriv = simple_derivative(&poly);

        // simple_derivative: 3 + 4x => [3.0, 4.0]
        assert_eq!(deriv, vec![3.0, 4.0]);
    }

    #[test]
    fn test_derivative_constant() {
        let poly = SimplePolynomial::parse("5").unwrap(); // 5
        let deriv = simple_derivative(&poly);

        assert!(deriv.is_empty()); // simple_derivative of constant is zero-length
    }

    #[test]
    fn test_derivative_linear() {
        let poly = SimplePolynomial::parse("3x + 2").unwrap(); // 2 + 3x
        let deriv = simple_derivative(&poly);

        assert_eq!(deriv, vec![3.0]); // simple_derivative: 3
    }

    #[test]
    fn test_derivative_zero_poly() {
        let poly = SimplePolynomial::parse("0").unwrap();
        let deriv = simple_derivative(&poly);

        assert!(deriv.is_empty());
    }

    #[test]
    fn test_derivative_higher_degree() {
        let poly = SimplePolynomial::parse("2x^3 + 3x^2 -4x + 1").unwrap(); // 1 - 4x + 3x^2 + 2x^3
        let deriv = simple_derivative(&poly);

        // simple_derivative: -4 + 6x + 6x^2 => [-4.0, 6.0, 6.0]
        assert_eq!(deriv, vec![-4.0, 6.0, 6.0]);
    }

    #[test]
    fn test_derivative_with_zero_coefficients() {
        let poly = SimplePolynomial::parse("5x^2").unwrap(); // 0 + 0x + 5x^2
        let deriv = simple_derivative(&poly);

        assert_eq!(deriv, vec![0.0, 10.0]);
    }

    #[test]
    fn test_derivative_large_coeffs() {
        let poly = SimplePolynomial::parse("3000000x^2 - 2000000x + 1000000").unwrap();
        let deriv = simple_derivative(&poly);

        assert_eq!(deriv, vec![-2e6, 6e6]);
    }
}
