#[cfg(test)]
mod tests {
    use spindalis::integrals::{analytical_integral, definite_integral, romberg_definite};
    use spindalis::polynomials::{PolynomialExtended, PolynomialTraits, SimplePolynomial};
    use spindalis_macros::analytical_integral;

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

    #[test]
    fn test_macro_vs_runtime_simple() {
        // Runtime calculation
        // x^2 + x + 1
        let parsed = SimplePolynomial::parse("x^2 + x + 1").unwrap();
        let runtime_result = analytical_integral!("x^2 + x + 1", 0.0, 1.0);

        // Macro calculation (evaluated at compile time)
        let macro_result = analytical_integral(&parsed, 0.0, 1.0).unwrap();

        // They should be identical
        assert_eq!(
            runtime_result, macro_result,
            "Macro result {} did not match runtime result {}",
            macro_result, runtime_result
        );
    }

    #[test]
    fn test_macro_vs_runtime_complex() {
        // f(x) = 64x^3 - 144x^2 + 108x - 27
        let parsed = SimplePolynomial::parse("64x^3 - 144x^2 + 108x - 27").unwrap();
        let start = -3.0;
        let end = 5.0;

        let runtime_result = analytical_integral(&parsed, start, end).unwrap();
        let macro_result = analytical_integral!("64x^3 - 144x^2 + 108x - 27", -3.0, 5.0);

        // Using a small epsilon due to floating point precision in integration
        assert!(
            (runtime_result - macro_result).abs() < 1e-10,
            "Macro: {}, Runtime: {}",
            macro_result,
            runtime_result
        );
    }

    #[test]
    fn test_macro_const_compatibility() {
        // One of the biggest advantages of the macro
        const RESULT: f64 = analytical_integral!("3x^2", 0.0, 2.0);
        assert_eq!(RESULT, 8.0);
    }
}
