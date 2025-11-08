pub fn indefinite_integral(poly: impl AsRef<[f64]>) -> Vec<f64> {
    let poly = poly.as_ref();
    let mut anti_deriv = Vec::with_capacity(poly.len().saturating_add(1));
    anti_deriv.push(0_f64); // This represents the + C and can be modified later
    for (power, &coeff) in poly.iter().enumerate() {
        anti_deriv.push(coeff / (power as f64 + 1_f64));
    }
    anti_deriv
}
