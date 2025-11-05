#[cfg(test)]
mod tests {
    use spindalis::integrals::indefinite_integral;
    use spindalis::polynomials::parse_simple_polynomial;

    #[test]
    fn test_known_indefinite() {
        let parsed = parse_simple_polynomial!(x ^ 3 - x);
        let result = indefinite_integral(&parsed);
        let expected = vec![0.0, 0.0, -0.5, 0.0, 0.25]; // 1/4x^4 - 1/2x^2

        assert_eq!(result, expected);
    }

    #[test]
    fn test_simplification() {
        let parsed = parse_simple_polynomial!(8x ^ 3 - 2x);
        let result = indefinite_integral(&parsed);
        let expected = parse_simple_polynomial!(2x ^ 4 - x ^ 2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_constant() {
        let parsed = parse_simple_polynomial!(8x ^ 3 - 2x + 6);
        let result = indefinite_integral(&parsed);
        let expected = parse_simple_polynomial!(2x ^ 4 - x ^ 2 + 6x);

        assert_eq!(result, expected);
    }
}
