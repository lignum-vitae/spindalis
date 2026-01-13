use std::collections::HashSet;

use crate::polynomials::{Term, structs::IntermediatePolynomial};

pub fn partial_derivative<S>(poly: &[Term], var: S) -> IntermediatePolynomial
where
    S: AsRef<str>,
{
    let var = var.as_ref();
    let mut parsed_deriv = Vec::new();
    for part in poly {
        let mut new_part = part.clone();
        for i in 0..part.variables.len() {
            let (v, pow) = &part.variables[i];
            if v == var {
                // Power rule
                new_part.coefficient = part.coefficient * (*pow);
                let new_power = pow - 1.0;
                if new_power == 0.0 {
                    // Remove variable portion of Term (coefficient remains)
                    new_part.variables.remove(i);
                } else {
                    new_part.variables[i].1 = new_power;
                }
                parsed_deriv.push(new_part);
                break; // Only differentiate once per term
            }
        }
    }

    let unique_variables: HashSet<String> = parsed_deriv
        .iter()
        .flat_map(|term| term.variables.iter())
        .map(|(var_name, _)| var_name.clone())
        .collect();
    let variables: Vec<String> = unique_variables.into_iter().collect();
    let mut res_poly = IntermediatePolynomial {
        terms: parsed_deriv,
        variables,
    };
    res_poly.sort_poly();
    res_poly
}
