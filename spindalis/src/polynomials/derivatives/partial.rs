use crate::polynomials::Term;

pub fn partial_derivative(poly: &Vec<Term>, var: &str) -> Vec<Term> {
    let mut parsed_deriv = Vec::new();

    for part in poly {
        let mut new_part = part.clone();
        for i in 0..part.variables.len() {
            let (v, pow) = &part.variables[i];

            if v == var && *pow > 0.0 {
                // Power rule
                new_part.coefficient = part.coefficient * (*pow);
                let new_power = pow - 1.0;

                if new_power == 0.0 {
                    new_part.variables.remove(i);
                } else {
                    new_part.variables[i].1 = new_power;
                }

                parsed_deriv.push(new_part);
                break; // Only differentiate once per term
            }
        }
    }
    parsed_deriv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_derivative_single_variable() {
        let poly = vec![Term {
            coefficient: 3.0,
            variables: vec![("x".into(), 2.0)],
        }];

        let result = partial_derivative(&poly, "x");

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

        let result = partial_derivative(&poly, "x");

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

        let result = partial_derivative(&poly, "x");

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

        assert_eq!(result.len(), 0);
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

        let result = partial_derivative(&poly, "x");

        assert_eq!(result.len(), 2);

        assert_eq!(result[0].coefficient, 4.0);
        assert_eq!(result[0].variables, vec![("x".into(), 1.0)]);

        assert_eq!(result[1].coefficient, 4.0);
        assert_eq!(result[1].variables, vec![("y".into(), 1.0)]);
    }
}
