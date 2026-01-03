use crate::polynomials::structs::SimplePolynomial;

pub fn simple_derivative(poly: &[f64]) -> SimplePolynomial {
    let mut deriv = Vec::with_capacity(poly.len().saturating_sub(1));
    for (power, &coeff) in poly.iter().enumerate().skip(1) {
        deriv.push(coeff * power as f64);
    }
    SimplePolynomial {
        coefficients: deriv,
    }
}
