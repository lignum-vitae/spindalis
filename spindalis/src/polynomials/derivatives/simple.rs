pub fn simple_derivative(poly: &[f64]) -> Vec<f64> {
    let mut deriv = Vec::with_capacity(poly.len().saturating_sub(1));
    for (power, &coeff) in poly.iter().enumerate().skip(1) {
        deriv.push(coeff * power as f64);
    }
    deriv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derivative_simple() {
        let poly = vec![4.0, 3.0, 2.0]; // 4 + 3x + 2x^2
        let deriv = simple_derivative(&poly);

        // simple_derivative: 3 + 4x => [3.0, 4.0]
        assert_eq!(deriv, vec![3.0, 4.0]);
    }

    #[test]
    fn test_derivative_constant() {
        let poly = vec![5.0]; // 5
        let deriv = simple_derivative(&poly);

        assert!(deriv.is_empty()); // simple_derivative of constant is zero-length
    }

    #[test]
    fn test_derivative_linear() {
        let poly = vec![2.0, 3.0]; // 2 + 3x
        let deriv = simple_derivative(&poly);

        assert_eq!(deriv, vec![3.0]); // simple_derivative: 3
    }

    #[test]
    fn test_derivative_zero_poly() {
        let poly = vec![];
        let deriv = simple_derivative(&poly);

        assert!(deriv.is_empty());
    }

    #[test]
    fn test_derivative_higher_degree() {
        let poly = vec![1.0, -4.0, 3.0, 2.0]; // 1 - 4x + 3x^2 + 2x^3
        let deriv = simple_derivative(&poly);

        // simple_derivative: -4 + 6x + 6x^2 => [-4.0, 6.0, 6.0]
        assert_eq!(deriv, vec![-4.0, 6.0, 6.0]);
    }

    #[test]
    fn test_derivative_with_zero_coefficients() {
        let poly = vec![0.0, 0.0, 5.0]; // 0 + 0x + 5x^2
        let deriv = simple_derivative(&poly);

        assert_eq!(deriv, vec![0.0, 10.0]);
    }

    #[test]
    fn test_derivative_large_coeffs() {
        let poly = vec![1e6, -2e6, 3e6]; // 1e6 - 2e6x + 3e6x^2
        let deriv = simple_derivative(&poly);

        assert_eq!(deriv, vec![-2e6, 6e6]);
    }
}
