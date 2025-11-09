use crate::polynomials::Term;

pub fn partial_derivative<S>(poly: &[Term], var: S) -> Vec<Term>
where
    S: AsRef<str>,
{
    let var = var.as_ref();
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
