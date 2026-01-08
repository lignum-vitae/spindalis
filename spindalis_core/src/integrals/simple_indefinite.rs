use crate::polynomials::structs::SimplePolynomial;

// Uses Chain Rule to find integral
pub fn indefinite_integral_simple(poly: &SimplePolynomial) -> SimplePolynomial {
    let mut anti_deriv = Vec::with_capacity(poly.coefficients.len().saturating_add(1));
    anti_deriv.push(0_f64); // This represents the + C and can be modified later
    for (power, &coeff) in poly.coefficients.iter().enumerate() {
        anti_deriv.push(coeff / (power as f64 + 1_f64));
    }
    SimplePolynomial {
        coefficients: anti_deriv,
        variable: poly.variable,
    }
}
