#[cfg(test)]
mod tests {
    use spindalis::polynomials::Term;
    use spindalis::polynomials::{
        PolynomialError, eval_polynomial_extended, parse_polynomial_extended,
    };
    use std::collections::HashMap;

    // test positive ints
    #[test]
    fn test_parse_single_variable() {
        let terms = parse_polynomial_extended("3x^2").unwrap().terms;
        let terms_macro = parse_polynomial_extended!(3x ^ 2);

        assert_eq!(terms.len(), 1);
        assert_eq!(terms_macro.len(), 1);
        let result = vec![Term {
            coefficient: 3.0,
            variables: vec![("x".into(), 2.0)],
        }];
        assert_eq!(terms, result);
        assert_eq!(terms_macro, result);
    }

    #[test]
    fn test_parse_multiple_variables() {
        let terms = parse_polynomial_extended("4x^2y^3").unwrap().terms;
        let terms_macro = parse_polynomial_extended!(4x ^ 2y ^ 3);

        let result = vec![Term {
            coefficient: 4.0,
            variables: vec![("x".into(), 2.0), ("y".into(), 3.0)],
        }];

        assert_eq!(terms, result);
        assert_eq!(terms_macro, result);
    }

    #[test]
    fn test_parse_no_coefficient() {
        let terms = parse_polynomial_extended("x^3").unwrap().terms;
        let terms_macro = parse_polynomial_extended!(x ^ 3);

        let result = vec![Term {
            coefficient: 1.0,
            variables: vec![("x".into(), 3.0)],
        }];

        assert_eq!(terms, result);
        assert_eq!(terms_macro, result);
    }

    #[test]
    fn test_parse_negative_coefficient() {
        let terms = parse_polynomial_extended("-2x^2").unwrap().terms;
        let terms_macro = parse_polynomial_extended!(-2x ^ 2);

        let result = vec![Term {
            coefficient: -2.0,
            variables: vec![("x".into(), 2.0)],
        }];

        assert_eq!(terms, result);
        assert_eq!(terms_macro, result);
    }

    #[test]
    fn test_parse_negative_variable() {
        let terms = parse_polynomial_extended("-x^2").unwrap().terms;
        let terms_macro = parse_polynomial_extended!(-x ^ 2);

        let result = vec![Term {
            coefficient: -1.0,
            variables: vec![("x".into(), 2.0)],
        }];

        assert_eq!(terms, result);
        assert_eq!(terms_macro, result);
    }

    #[test]
    fn test_parse_multiple_terms() {
        let terms = parse_polynomial_extended("2x^2+3y-4z^3").unwrap().terms;
        let terms_macro = parse_polynomial_extended!(2x ^ 2 + 3y - 4z ^ 3);

        let result = vec![
            Term {
                coefficient: 2.0,
                variables: vec![("x".into(), 2.0)],
            },
            Term {
                coefficient: 3.0,
                variables: vec![("y".into(), 1.0)],
            },
            Term {
                coefficient: -4.0,
                variables: vec![("z".into(), 3.0)],
            },
        ];

        assert_eq!(terms.len(), 3);
        assert_eq!(terms_macro.len(), 3);

        assert_eq!(terms, result);
        assert_eq!(terms_macro, result);
    }

    #[test]
    fn test_parse_missing_power_defaults_to_one() {
        let terms = parse_polynomial_extended("5x").unwrap().terms;
        let terms_macro = parse_polynomial_extended!(5x);

        let result = vec![Term {
            coefficient: 5.0,
            variables: vec![("x".into(), 1.0)],
        }];

        assert_eq!(terms, result);
        assert_eq!(terms_macro, result);
    }

    #[test]
    fn test_parse_invalid_power_returns_none() {
        let expr = "2x^a";
        let result = parse_polynomial_extended(expr);

        assert!(result.is_err());
    }

    // Test floats

    #[test]
    fn test_parse_pos_decimal() {
        let terms = parse_polynomial_extended("5x^0.5").unwrap().terms;
        let terms_macro = parse_polynomial_extended!(5x ^ 0.5);

        let result = vec![Term {
            coefficient: 5.0,
            variables: vec![("x".into(), 0.5)],
        }];

        assert_eq!(terms, result);
        assert_eq!(terms_macro, result);
    }

    #[test]
    fn test_parse_neg_decimal() {
        let terms = parse_polynomial_extended("5x^-0.5").unwrap().terms;
        let terms_macro = parse_polynomial_extended!(5x ^ -0.5);

        let result = vec![Term {
            coefficient: 5.0,
            variables: vec![("x".into(), -0.5)],
        }];

        assert_eq!(terms, result);
        assert_eq!(terms_macro, result);
    }

    #[test]
    fn test_parse_err_decimal() {
        let expr = "5x^-0.5.0";
        let result = parse_polynomial_extended(expr);

        assert!(result.is_err());
    }

    // Test fractions

    #[test]
    fn test_parse_fraction() {
        let terms = parse_polynomial_extended("5x^1/2").unwrap().terms;
        let terms_macro = parse_polynomial_extended!(5x ^ 1 / 2);

        let result = vec![Term {
            coefficient: 5.0,
            variables: vec![("x".into(), 0.5)],
        }];

        assert_eq!(terms, result);
        assert_eq!(terms_macro, result);
    }

    #[test]
    fn test_parse_float_fraction() {
        let terms = parse_polynomial_extended("5x^0.5/1").unwrap().terms;
        let terms_macro = parse_polynomial_extended!(5x ^ 0.5 / 1);

        let result = vec![Term {
            coefficient: 5.0,
            variables: vec![("x".into(), 0.5)],
        }];

        assert_eq!(terms, result);
        assert_eq!(terms_macro, result);
    }

    #[test]
    fn test_parse_err_fraction() {
        let expr = "5x^0.5/1.0/1.0";
        let result = parse_polynomial_extended(expr);

        assert!(result.is_err());
    }

    // Test eval function

    #[test]
    fn test_single_variable() {
        let terms = vec![
            Term {
                coefficient: 3.0,
                variables: vec![("x".to_string(), 2.0)],
            }, // 3x^2
            Term {
                coefficient: -2.0,
                variables: vec![("x".to_string(), 1.0)],
            }, // -2x
            Term {
                coefficient: 5.0,
                variables: vec![],
            },
        ];

        let vars = vec![("x", 2)];

        let result = eval_polynomial_extended(&terms, &vars).unwrap();
        // 3*2^2 - 2*2 + 5 = 12 - 4 + 5 = 13
        assert_eq!(result, 13.0);
    }

    #[test]
    fn test_multiple_variables() {
        let terms = vec![
            Term {
                coefficient: 2.0,
                variables: vec![("x".to_string(), 1.0), ("y".to_string(), 2.0)],
            }, // 2xy^2
            Term {
                coefficient: 4.0,
                variables: vec![("y".to_string(), 1.0)],
            }, // 4y
        ];

        let vars = vec![("x", 3), ("y", 2)];
        let result = eval_polynomial_extended(&terms, &vars).unwrap();
        // 2*3*2^2 + 4*2 = 2*3*4 + 8 = 24 + 8 = 32
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_fractional_exponent() {
        let terms = vec![Term {
            coefficient: 1.0,
            variables: vec![("x".to_string(), 0.5)],
        }];

        let mut vars = HashMap::new();
        vars.insert("x".to_string(), 16.0);

        let result = eval_polynomial_extended(&terms, &vars).unwrap();
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_missing_variable_panics() {
        let terms = vec![Term {
            coefficient: 1.0,
            variables: vec![("z".to_string(), 1.0)],
        }];

        let vars: Vec<(&str, f64)> = vec![];
        let result = eval_polynomial_extended(&terms, &vars);
        assert!(matches!(
            result,
            Err(PolynomialError::VariableNotFound { variable: _ })
        ))
    }

    #[test]
    fn test_constant_only_term() {
        let terms = vec![
            Term {
                coefficient: 7.5,
                variables: vec![],
            }, // constant term
        ];

        let vars: Vec<(&str, f64)> = vec![];
        let result = eval_polynomial_extended(&terms, &vars).unwrap();
        assert_eq!(result, 7.5);
    }

    #[test]
    fn test_neg_exponent() {
        let terms = vec![Term {
            coefficient: 1.0,
            variables: vec![("x".to_string(), -0.5)],
        }];

        let vars = [("x", 16)];

        let result = eval_polynomial_extended(&terms, &vars).unwrap();
        assert_eq!(result, 0.25);
    }
}
