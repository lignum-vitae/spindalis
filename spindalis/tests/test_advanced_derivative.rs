// REMEMBER THAT THESE TESTS SHOULD RETURN THE DERIVATIVES
// NOT JUST THE RESULT AFTER EXTRA VARS ARE REMOVED

#[cfg(test)]
mod tests {
    use spindalis::derivatives::advanced_derivative;
    use spindalis::polynomials::Polynomial;

    #[test]
    fn univariate_derivative() {
        let parsed = Polynomial::parse("3x + 2").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3x").unwrap();
        //let expected = Polynomial::parse("3").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn univariate_derivative_2() {
        let parsed = Polynomial::parse("2 + 3x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3x").unwrap();
        //let expected = Polynomial::parse("3").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn unary_prefix() {
        let parsed = Polynomial::parse("-3x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("-3x").unwrap();
        //let expected = Polynomial::parse("-3").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_derivative() {
        let parsed = Polynomial::parse("3xy + 2z").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3xy").unwrap();
        //let expected = Polynomial::parse("3y").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_derivative_2() {
        let parsed = Polynomial::parse("2z + 3xy").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3xy").unwrap();
        //let expected = Polynomial::parse("3y").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn univariate_with_exponent() {
        let parsed = Polynomial::parse("3x^2 + 3x - 3").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3x^2 + 3x").unwrap();
        //let expected = Polynomial::parse("6x + 3").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn univariate_with_exponent_2() {
        let parsed = Polynomial::parse("- 3 + 3x^2 + 3x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3x^2 + 3x").unwrap();
        //let expected = Polynomial::parse("6x + 3").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_with_exponent() {
        let parsed = Polynomial::parse("3x^2y + 3xz - 3a + 2d^5").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3x^2y + 3xz").unwrap();
        //let expected = Polynomial::parse("6xy + 3z").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_with_exponent_2() {
        let parsed = Polynomial::parse("3xy^2 + 3xz - 3a + 2d^5").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3xy^2 + 3xz").unwrap();
        //let expected = Polynomial::parse("3y^2 + 3z").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_with_exponent_3() {
        let parsed = Polynomial::parse("- 3a + 3x^2y + 2d^5 + 3xz").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3x^2y + 3xz").unwrap();
        //let expected = Polynomial::parse("6xy + 3z").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_with_exponent_4() {
        let parsed = Polynomial::parse("- 3a + 3xy^2 + 2d^5 + 3xz").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3xy^2 + 3xz").unwrap();
        //let expected = Polynomial::parse("3y^2 + 3z").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn univariate_div_deriv() {
        let parsed = Polynomial::parse("3x/4 + 3x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3x/4 + 3x").unwrap();
        //let expected = Polynomial::parse("3/4 + 3z").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn univariate_div_deriv_2() {
        let parsed = Polynomial::parse("3/x + 3x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3/x + 3x").unwrap();
        //let expected = Polynomial::parse("-3/x^2 + 3z").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_div_deriv() {
        let parsed = Polynomial::parse("3x/y + 3xz").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3x/y + 3xz").unwrap();
        //let expected = Polynomial::parse("3/y + 3z").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_div_deriv_2() {
        let parsed = Polynomial::parse("3/x + 3xz").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3/x + 3xz").unwrap();
        //let expected = Polynomial::parse("-3/x^2 + 3z").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_div_deriv_3() {
        let parsed = Polynomial::parse("3/xy + 3xz").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3/xy + 3xz").unwrap();
        //let expected = Polynomial::parse("-3/x^2y + 3z").unwrap();

        assert_eq!(result, expected);
    }
}
