#[cfg(test)]
mod tests {
    use spindalis::integrals::{definite_integral, romberg_definite};
    use spindalis::polynomials::{PolynomialExtended, PolynomialTraits, SimplePolynomial};

    #[test]
    fn test_known_solution_std_integral() {
        let parsed = SimplePolynomial::parse("64x ^ 3 - 144x ^ 2 + 108x - 27").unwrap();
        let result = definite_integral(&parsed, -3.0, 5.0, 5).unwrap();
        let expected = 2056_f64;

        assert!(
            (result - expected).abs() < 1e-5,
            "Expected {expected} but got {result}. Difference is too large.",
        );
    }

    #[test]
    fn test_known_solution_romberg() {
        let parsed = PolynomialExtended::parse("64x ^ 3 - 144x ^ 2 + 108x - 27").unwrap();
        let result = romberg_definite(&parsed, -3.0, 5.0, 100, 1e-5).unwrap();
        let expected = 2056_f64;

        assert!(
            (result - expected).abs() < 1e-5,
            "Expected {expected} but got {result}. Difference is too large.",
        );
    }

    #[test]
    fn test_integral_quadratic() {
        let parsed = SimplePolynomial::parse("x ^ 2").unwrap();
        let result = definite_integral(&parsed, 0.0, 3.0, 6).unwrap();
        let expected = 9.0;
        assert!(
            (result - expected).abs() < 1e-5,
            "expected {expected}, got {result}"
        );
    }

    #[test]
    fn test_integral_cubic_mixed_segments() {
        let parsed = PolynomialExtended::parse("x ^ 3 - 2x ^ 2 + 3x - 1").unwrap();
        let result = definite_integral(&parsed, 0.0, 2.0, 5).unwrap();
        let expected = 2.666666666;
        assert!(
            (result - expected).abs() < 1e-5,
            "expected {expected}, got {result}"
        );
    }

    #[test]
    fn test_integral_single_segment_trapezoid() {
        let poly = SimplePolynomial::parse("2x").unwrap();
        let result = definite_integral(&poly, 0.0, 4.0, 1).unwrap();
        let expected = 16.0;
        assert!((result - expected).abs() < 1e-5);
    }

    #[test]
    fn test_romberg_polynomial() {
        let parsed = PolynomialExtended::parse("3x ^ 2").unwrap();
        let result = romberg_definite(&parsed, 0.0, 1.0, 8, 1e-5).unwrap();
        assert!((result - 1.0).abs() < 1e-6);
    }
}
