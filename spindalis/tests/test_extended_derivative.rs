#[cfg(test)]
mod tests {
    use spindalis::derivatives::partial_derivative;
    use spindalis::polynomials::Term;

    #[test]
    fn test_partial_derivative_single_variable() {
        let poly = vec![Term {
            coefficient: 3.0,
            variables: vec![("x".into(), 2.0)],
        }];

        let result = partial_derivative(&poly, "x").terms;

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 6.0);
        assert_eq!(result[0].variables, vec![("x".into(), 1.0)]);
    }

    #[test]
    fn test_partial_derivative_multiple_variables() {
        let poly = vec![Term {
            coefficient: 4.0,
            variables: vec![("x".into(), 2.0), ("y".into(), 3.0)],
        }];

        let result = partial_derivative(&poly, "x").terms;

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 8.0);
        assert_eq!(
            result[0].variables,
            vec![("x".into(), 1.0), ("y".into(), 3.0)]
        );
    }

    #[test]
    fn test_partial_derivative_remove_variable() {
        let poly = vec![Term {
            coefficient: 5.0,
            variables: vec![("x".into(), 1.0)],
        }];

        let result = partial_derivative(&poly, "x").terms;

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].coefficient, 5.0);
        assert_eq!(result[0].variables.len(), 0);
    }

    #[test]
    fn test_partial_derivative_variable_not_found() {
        let poly = vec![Term {
            coefficient: 7.0,
            variables: vec![("y".into(), 3.0)],
        }];

        let result = partial_derivative(&poly, "x");

        assert!(result.is_empty());
    }

    #[test]
    fn test_partial_derivative_multiple_terms() {
        let poly = vec![
            Term {
                coefficient: 2.0,
                variables: vec![("x".into(), 2.0)],
            },
            Term {
                coefficient: 3.0,
                variables: vec![("y".into(), 3.0)],
            },
            Term {
                coefficient: 4.0,
                variables: vec![("x".into(), 1.0), ("y".into(), 1.0)],
            },
        ];

        let result = partial_derivative(&poly, "x").terms;

        assert_eq!(result.len(), 2);

        assert_eq!(result[0].coefficient, 4.0);
        assert_eq!(result[0].variables, vec![("x".into(), 1.0)]);

        assert_eq!(result[1].coefficient, 4.0);
        assert_eq!(result[1].variables, vec![("y".into(), 1.0)]);
    }
}
