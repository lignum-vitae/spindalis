pub fn derivative(poly: &Vec<f64>) -> Vec<f64> {
    let mut deriv = Vec::with_capacity(poly.len().saturating_sub(1));
    for (power, &coeff) in poly.iter().enumerate().skip(1) {
        deriv.push(coeff * power as f64);
    }
    deriv
}
