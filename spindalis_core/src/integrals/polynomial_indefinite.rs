use crate::polynomials::Term;
use crate::polynomials::structs::polynomial::Polynomial;
use std::collections::HashSet;

// Uses Chain Rule to find integral
pub fn indefinite_integral_polynomial<S>(poly: &[Term], var: S) -> Polynomial
where
    S: AsRef<str>,
{
    let var = var.as_ref();
    let mut integrated = Vec::new();
    for part in poly {
        let mut new_part = part.clone();
        let mut is_constant = true;
        for i in 0..part.variables.len() {
            let (v, pow) = &part.variables[i];
            if v == var {
                is_constant = false;
                new_part.coefficient = part.coefficient / (*pow + 1.0);
                new_part.variables[i].1 = pow + 1.0;
                break;
            }
        }
        if is_constant {
            new_part.variables.push((var.to_string(), 1.0));
        }
        integrated.push(new_part);
    }
    let unique_variables: HashSet<String> = integrated
        .iter()
        .flat_map(|term| term.variables.iter())
        .map(|(var_name, _)| var_name.clone())
        .collect();
    Polynomial {
        terms: integrated,
        variables: unique_variables.into_iter().collect(),
    }
}
