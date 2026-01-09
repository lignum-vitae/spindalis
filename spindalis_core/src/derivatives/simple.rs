use crate::polynomials::structs::SimplePolynomial;

pub fn simple_derivative(poly: &SimplePolynomial) -> SimplePolynomial {
    let mut deriv = Vec::with_capacity(poly.coefficients.len().saturating_sub(1));
    for (power, &coeff) in poly.coefficients.iter().enumerate().skip(1) {
        deriv.push(coeff * power as f64);
    }
    SimplePolynomial {
        coefficients: deriv,
        variable: poly.variable,
    }
}
