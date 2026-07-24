//Change some raw function calls with `.derive_multivariate` and .`derive_univariate`

#[cfg(test)]
mod tests {
    use spindalis::derivatives::advanced_derivative;
    use spindalis::polynomials::Polynomial;

    #[test]
    fn univariate_derivative() {
        let parsed = Polynomial::parse("3x + 2").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn univariate_derivative_2() {
        let parsed = Polynomial::parse("2 + 3x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn unary_prefix() {
        let parsed = Polynomial::parse("-3x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("-3").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_derivative() {
        let parsed = Polynomial::parse("3xy + 2z").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3y").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_derivative_2() {
        let parsed = Polynomial::parse("2z + 3xy").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3y").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn univariate_with_exponent() {
        let parsed = Polynomial::parse("3x^2 + 3x - 3").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("6x + 3").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn univariate_with_exponent_2() {
        let parsed = Polynomial::parse("- 3 + 3x^2 + 3x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("6x + 3").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn univariate_with_exponent_3() {
        let parsed = Polynomial::parse("- 3 - 3x^2 + 3x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("-6x + 3").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_with_exponent() {
        let parsed = Polynomial::parse("3x^2y + 3xz - 3a + 2d^5").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("6xy + 3z").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_with_exponent_2() {
        let parsed = Polynomial::parse("3xy^2 + 3xz - 3a + 2d^5").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3y^2 + 3z").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_with_exponent_3() {
        let parsed = Polynomial::parse("- 3a + 3x^2y + 2d^5 + 3xz").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("6xy + 3z").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_with_exponent_4() {
        let parsed = Polynomial::parse("- 3a + 3xy^2 + 2d^5 + 3xz").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3y^2 + 3z").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn univariate_div_deriv() {
        let parsed = Polynomial::parse("3x/4 + 3x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3/4 + 3").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn univariate_div_deriv_2() {
        let parsed = Polynomial::parse("3/x + 3x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("-3/x^2 + 3").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_div_deriv() {
        let parsed = Polynomial::parse("3x/y + 3xz").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3/y + 3z").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_div_deriv_2() {
        let parsed = Polynomial::parse("3/x + 3xz").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("-3/x^2 + 3z").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn multivariate_div_deriv_3() {
        let parsed = Polynomial::parse("3/xy + 3xz").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("-3/x^2y + 3z").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn many_vars() {
        let parsed = Polynomial::parse("4xyz - 7yz").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("4yz").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn many_vars_with_exponent() {
        let parsed = Polynomial::parse("4x^2yz - 7yz").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("8xyz").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn sin_deriv() {
        let parsed = Polynomial::parse("sin(3x)").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3*cos(3x)").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn cos_deriv() {
        let parsed = Polynomial::parse("cos(3x)").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3*-sin(3x)").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn tan_deriv() {
        let parsed = Polynomial::parse("tan(3x)").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3*sec(3x)^2").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn cot_deriv() {
        let parsed = Polynomial::parse("cot(3x)").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3*-csc(3x)^2").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn sec_deriv() {
        let parsed = Polynomial::parse("sec(3x)").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3(sec(3x)*tan(3x))").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn csc_deriv() {
        let parsed = Polynomial::parse("csc(3x)").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3*-csc(3x)*cot(3x)").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn ln_deriv() {
        let parsed = Polynomial::parse("ln(x)").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("1/x").unwrap(); // 3/3x -> 1/x
        assert_eq!(result, expected);
    }

    #[test]
    fn log_deriv() {
        let parsed = Polynomial::parse("log(x)").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("1/x").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn e_x_deriv() {
        let parsed = Polynomial::parse("e^x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("e^x").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn e_a_deriv() {
        let parsed = Polynomial::parse("3x + e^a").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn plus_e_x_deriv() {
        let parsed = Polynomial::parse("3x + e^x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3 + e^x").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn mul_e_a_deriv() {
        let parsed = Polynomial::parse("3xe^a").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3e^a").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn a_x_deriv() {
        let parsed = Polynomial::parse("a^x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("ln(a)*a^x").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn num_a_x_deriv() {
        let parsed = Polynomial::parse("3a^x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3 * ln(a)*a^x").unwrap();
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn num_a_b_x_deriv() {
        let parsed = Polynomial::parse("3ba^x").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3b * ln(a)*a^x").unwrap();
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn sqrt_deriv() {
        let parsed = Polynomial::parse("sqrt(3x)").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("1.5 * 3^(-0.5) * x^(-0.5)").unwrap();
        // The AST is slightly different but the display string is the same
        // and the two ASTs are functionally the same
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn add_div_deriv() {
        let parsed = Polynomial::parse("(x+45)/(34+y)").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("1/(34+y)").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_div_div_deriv() {
        let parsed = Polynomial::parse("(x+45)/(34+y)/y").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("y/(34+y)").unwrap();
        assert_eq!(result, expected);
    }
}
