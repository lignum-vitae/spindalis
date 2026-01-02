#[cfg(test)]
mod tests {
    use spindalis::integrals::{analytical_integral, definite_integral, romberg_definite};
    use spindalis::polynomials::{eval_simple_polynomial, parse_simple_polynomial};
    use spindalis_macros::definite_integral;

    #[test]
    fn test_known_solution_std_integral() {
        let parsed = parse_simple_polynomial!(64x ^ 3 - 144x ^ 2 + 108x - 27);
        let result = definite_integral(&parsed, eval_simple_polynomial, -3.0, 5.0, 5);
        let expected = 2056_f64;

        assert!(
            (result - expected).abs() < 1e-5,
            "Expected {expected} but got {result}. Difference is too large.",
        );
    }

    #[test]
    fn test_known_solution_romberg() {
        let parsed = parse_simple_polynomial!(64x ^ 3 - 144x ^ 2 + 108x - 27);
        let result =
            romberg_definite(&parsed, eval_simple_polynomial, -3.0, 5.0, 100, 1e-5).unwrap();
        let expected = 2056_f64;

        assert!(
            (result - expected).abs() < 1e-5,
            "Expected {expected} but got {result}. Difference is too large.",
        );
    }

    #[test]
    fn test_integral_quadratic() {
        let parsed = parse_simple_polynomial!(x ^ 2);
        let result = definite_integral(&parsed, eval_simple_polynomial, 0.0, 3.0, 6);
        let expected = 9.0;
        assert!(
            (result - expected).abs() < 1e-5,
            "expected {expected}, got {result}"
        );
    }

    #[test]
    fn test_integral_cubic_mixed_segments() {
        let parsed = parse_simple_polynomial!(x ^ 3 - 2x ^ 2 + 3x - 1);
        let result = definite_integral(&parsed, eval_simple_polynomial, 0.0, 2.0, 5);
        let expected = 2.666666666;
        assert!(
            (result - expected).abs() < 1e-5,
            "expected {expected}, got {result}"
        );
    }

    #[test]
    fn test_integral_single_segment_trapezoid() {
        let poly = [0.0, 2.0]; // f(x) = 2x
        let result = definite_integral(poly, eval_simple_polynomial, 0.0, 4.0, 1);
        let expected = 16.0;
        assert!((result - expected).abs() < 1e-5);
    }

    #[test]
    fn test_romberg_polynomial() {
        let parsed = parse_simple_polynomial!(3x ^ 2);
        let result = romberg_definite(&parsed, eval_simple_polynomial, 0.0, 1.0, 8, 1e-5).unwrap();
        assert!((result - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_macro_vs_runtime_simple() {
        // Runtime calculation
        let poly = [1.0, 1.0, 1.0]; // x^2 + x + 1
        let runtime_result = analytical_integral(&poly, 0.0, 1.0);

        // Macro calculation (evaluated at compile time)
        let macro_result = definite_integral!("x^2 + x + 1", 0.0, 1.0);

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
        let poly = [-27.0, 108.0, -144.0, 64.0];
        let start = -3.0;
        let end = 5.0;

        let runtime_result = analytical_integral(&poly, start, end);
        let macro_result = definite_integral!("64x^3 - 144x^2 + 108x - 27", -3.0, 5.0);

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
        const RESULT: f64 = definite_integral!("3x^2", 0.0, 2.0);
        assert_eq!(RESULT, 8.0);
    }
}
