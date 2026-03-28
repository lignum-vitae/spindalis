use crate::polynomials::Term;
use crate::polynomials::structs::IntermediatePolynomial;
use std::collections::HashMap;

/// Multiply two terms together.
///
/// Coefficients are multiplied; variables with matching names have their
/// exponents summed; variables unique to one term are carried through.
fn multiply_terms(a: &Term, b: &Term) -> Term {
    let coefficient = a.coefficient * b.coefficient;

    let mut var_map: HashMap<&str, f64> = HashMap::new();
    for (name, exp) in &a.variables {
        *var_map.entry(name.as_str()).or_insert(0.0) += exp;
    }
    for (name, exp) in &b.variables {
        *var_map.entry(name.as_str()).or_insert(0.0) += exp;
    }

    let mut variables: Vec<(String, f64)> = var_map
        .into_iter()
        .filter(|(_, exp)| *exp != 0.0)
        .map(|(name, exp)| (name.to_string(), exp))
        .collect();
    variables.sort_by(|a, b| a.0.cmp(&b.0));

    Term {
        coefficient,
        variables,
    }
}

/// Build a string key for a term's variable signature, used to identify like terms.
fn var_key(variables: &[(String, f64)]) -> String {
    let mut sorted = variables.to_vec();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    sorted
        .iter()
        .map(|(name, exp)| format!("{}:{}", name, exp.to_bits()))
        .collect::<Vec<_>>()
        .join(",")
}

/// Combine like terms in a list of terms.
///
/// Terms are "like" if they have the same set of (variable, exponent) pairs.
/// Like terms have their coefficients summed.
fn combine_like_terms(terms: Vec<Term>) -> Vec<Term> {
    let mut map: HashMap<String, (f64, Vec<(String, f64)>)> = HashMap::new();

    for term in terms {
        let key = var_key(&term.variables);
        let entry = map.entry(key).or_insert((0.0, term.variables.clone()));
        entry.0 += term.coefficient;
    }

    let mut result: Vec<Term> = map
        .into_values()
        .filter(|(coeff, _)| *coeff != 0.0)
        .map(|(coefficient, variables)| Term {
            coefficient,
            variables,
        })
        .collect();

    // Sort for deterministic output: by total degree desc, then variable names
    result.sort_by(|a, b| {
        let a_vars: Vec<&str> = a.variables.iter().map(|(n, _)| n.as_str()).collect();
        let b_vars: Vec<&str> = b.variables.iter().map(|(n, _)| n.as_str()).collect();
        let a_degree: f64 = a.variables.iter().map(|(_, e)| e).sum();
        let b_degree: f64 = b.variables.iter().map(|(_, e)| e).sum();
        b_degree
            .partial_cmp(&a_degree)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a_vars.cmp(&b_vars))
    });

    result
}

/// Collect unique variable names from a list of terms.
fn collect_variables(terms: &[Term]) -> Vec<String> {
    let mut vars: Vec<String> = terms
        .iter()
        .flat_map(|t| t.variables.iter().map(|(n, _)| n.clone()))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    vars.sort();
    vars
}

/// Multiply two intermediate polynomials together.
///
/// Every term in `a` is multiplied by every term in `b`, then like terms
/// are combined.
pub fn multiply_intermediate(
    a: &IntermediatePolynomial,
    b: &IntermediatePolynomial,
) -> IntermediatePolynomial {
    if a.is_empty() || b.is_empty() {
        return IntermediatePolynomial {
            terms: vec![],
            variables: vec![],
        };
    }

    let mut terms = Vec::with_capacity(a.terms.len() * b.terms.len());
    for at in &a.terms {
        for bt in &b.terms {
            terms.push(multiply_terms(at, bt));
        }
    }

    let terms = combine_like_terms(terms);
    let variables = collect_variables(&terms);

    IntermediatePolynomial { terms, variables }
}

/// Expand (raise to a power) an intermediate polynomial using repeated squaring.
///
/// Returns the polynomial raised to the given non-negative integer exponent.
/// `expand_intermediate(p, 0)` returns the constant polynomial `1`.
pub fn expand_intermediate(poly: &IntermediatePolynomial, exponent: u32) -> IntermediatePolynomial {
    if exponent == 0 {
        return IntermediatePolynomial {
            terms: vec![Term {
                coefficient: 1.0,
                variables: vec![],
            }],
            variables: vec![],
        };
    }

    let one = IntermediatePolynomial {
        terms: vec![Term {
            coefficient: 1.0,
            variables: vec![],
        }],
        variables: vec![],
    };

    let mut result = one;
    let mut base = IntermediatePolynomial {
        terms: poly.terms.clone(),
        variables: poly.variables.clone(),
    };
    let mut exp = exponent;

    while exp > 0 {
        if exp % 2 == 1 {
            result = multiply_intermediate(&result, &base);
        }
        base = multiply_intermediate(&base, &base);
        exp /= 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn term(coeff: f64, vars: Vec<(&str, f64)>) -> Term {
        Term {
            coefficient: coeff,
            variables: vars.into_iter().map(|(n, e)| (n.to_string(), e)).collect(),
        }
    }

    fn poly(terms: Vec<Term>, vars: Vec<&str>) -> IntermediatePolynomial {
        IntermediatePolynomial {
            terms,
            variables: vars.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn test_multiply_terms_univariate() {
        // 3x^2 * 2x^3 = 6x^5
        let a = term(3.0, vec![("x", 2.0)]);
        let b = term(2.0, vec![("x", 3.0)]);
        let result = multiply_terms(&a, &b);
        assert_eq!(result.coefficient, 6.0);
        assert_eq!(result.variables, vec![("x".to_string(), 5.0)]);
    }

    #[test]
    fn test_multiply_terms_multivariate() {
        // 2xy * 3yz = 6xyz^1 (y exponents sum)
        // Actually: 2xy * 3yz = 6xy^2z
        let a = term(2.0, vec![("x", 1.0), ("y", 1.0)]);
        let b = term(3.0, vec![("y", 1.0), ("z", 1.0)]);
        let result = multiply_terms(&a, &b);
        assert_eq!(result.coefficient, 6.0);
        assert_eq!(
            result.variables,
            vec![
                ("x".to_string(), 1.0),
                ("y".to_string(), 2.0),
                ("z".to_string(), 1.0)
            ]
        );
    }

    #[test]
    fn test_multiply_polynomials() {
        // (x + 1)(x - 1) = x^2 - 1
        let a = poly(
            vec![term(1.0, vec![("x", 1.0)]), term(1.0, vec![])],
            vec!["x"],
        );
        let b = poly(
            vec![term(1.0, vec![("x", 1.0)]), term(-1.0, vec![])],
            vec!["x"],
        );
        let result = multiply_intermediate(&a, &b);

        // Should have x^2 and -1 (the 0*x terms cancel)
        assert_eq!(result.terms.len(), 2);

        let x2_term = result
            .terms
            .iter()
            .find(|t| t.variables == vec![("x".to_string(), 2.0)]);
        assert!(x2_term.is_some());
        assert_eq!(x2_term.unwrap().coefficient, 1.0);

        let const_term = result.terms.iter().find(|t| t.variables.is_empty());
        assert!(const_term.is_some());
        assert_eq!(const_term.unwrap().coefficient, -1.0);
    }

    #[test]
    fn test_expand_power_zero() {
        let p = poly(
            vec![
                term(4.0, vec![("x", 1.0)]),
                term(-3.0, vec![]),
            ],
            vec!["x"],
        );
        let result = expand_intermediate(&p, 0);
        assert_eq!(result.terms.len(), 1);
        assert_eq!(result.terms[0].coefficient, 1.0);
        assert!(result.terms[0].variables.is_empty());
    }

    #[test]
    fn test_expand_power_one() {
        let p = poly(
            vec![
                term(4.0, vec![("x", 1.0)]),
                term(-3.0, vec![]),
            ],
            vec!["x"],
        );
        let result = expand_intermediate(&p, 1);

        let x_term = result
            .terms
            .iter()
            .find(|t| t.variables == vec![("x".to_string(), 1.0)]);
        assert_eq!(x_term.unwrap().coefficient, 4.0);

        let const_term = result.terms.iter().find(|t| t.variables.is_empty());
        assert_eq!(const_term.unwrap().coefficient, -3.0);
    }

    #[test]
    fn test_expand_4x_minus_3_cubed() {
        // (4x - 3)^3 = 64x^3 - 144x^2 + 108x - 27
        let p = poly(
            vec![
                term(4.0, vec![("x", 1.0)]),
                term(-3.0, vec![]),
            ],
            vec!["x"],
        );
        let result = expand_intermediate(&p, 3);

        let find = |vars: Vec<(&str, f64)>| -> f64 {
            let target: Vec<(String, f64)> =
                vars.into_iter().map(|(n, e)| (n.to_string(), e)).collect();
            result
                .terms
                .iter()
                .find(|t| t.variables == target)
                .map(|t| t.coefficient)
                .unwrap_or(0.0)
        };

        assert_eq!(find(vec![("x", 3.0)]), 64.0);
        assert_eq!(find(vec![("x", 2.0)]), -144.0);
        assert_eq!(find(vec![("x", 1.0)]), 108.0);
        assert_eq!(find(vec![]), -27.0);
    }

    #[test]
    fn test_expand_multivariate_squared() {
        // (x + y)^2 = x^2 + 2xy + y^2
        let p = poly(
            vec![
                term(1.0, vec![("x", 1.0)]),
                term(1.0, vec![("y", 1.0)]),
            ],
            vec!["x", "y"],
        );
        let result = expand_intermediate(&p, 2);

        let find = |vars: Vec<(&str, f64)>| -> f64 {
            let target: Vec<(String, f64)> =
                vars.into_iter().map(|(n, e)| (n.to_string(), e)).collect();
            result
                .terms
                .iter()
                .find(|t| t.variables == target)
                .map(|t| t.coefficient)
                .unwrap_or(0.0)
        };

        assert_eq!(find(vec![("x", 2.0)]), 1.0);
        assert_eq!(find(vec![("x", 1.0), ("y", 1.0)]), 2.0);
        assert_eq!(find(vec![("y", 2.0)]), 1.0);
    }
}
