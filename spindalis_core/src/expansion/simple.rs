use crate::polynomials::structs::SimplePolynomial;

/// Multiply two simple polynomials together.
///
/// Each polynomial is stored as a coefficient vector where index = power.
/// Multiplying term `a*x^i` by `b*x^j` gives `(a*b)*x^(i+j)`.
pub fn multiply_simple(a: &SimplePolynomial, b: &SimplePolynomial) -> SimplePolynomial {
    if a.is_empty() || b.is_empty() {
        return SimplePolynomial {
            coefficients: vec![],
            variable: a.variable.or(b.variable),
        };
    }

    let result_len = a.coefficients.len() + b.coefficients.len() - 1;
    let mut coefficients = vec![0.0; result_len];

    for (i, &ac) in a.coefficients.iter().enumerate() {
        for (j, &bc) in b.coefficients.iter().enumerate() {
            coefficients[i + j] += ac * bc;
        }
    }

    SimplePolynomial {
        coefficients,
        variable: a.variable.or(b.variable),
    }
}

/// Expand (raise to a power) a simple polynomial using repeated squaring.
///
/// Returns the polynomial raised to the given non-negative integer exponent.
/// `expand_simple(p, 0)` returns the constant polynomial `1`.
pub fn expand_simple(poly: &SimplePolynomial, exponent: u32) -> SimplePolynomial {
    if exponent == 0 {
        return SimplePolynomial {
            coefficients: vec![1.0],
            variable: poly.variable,
        };
    }

    let mut result = SimplePolynomial {
        coefficients: vec![1.0],
        variable: poly.variable,
    };
    let mut base = poly.clone();
    let mut exp = exponent;

    while exp > 0 {
        if exp % 2 == 1 {
            result = multiply_simple(&result, &base);
        }
        base = multiply_simple(&base, &base);
        exp /= 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple(coeffs: Vec<f64>, var: Option<char>) -> SimplePolynomial {
        SimplePolynomial {
            coefficients: coeffs,
            variable: var,
        }
    }

    #[test]
    fn test_multiply_constant_by_polynomial() {
        // 3 * (2x + 1) = 6x + 3
        let a = simple(vec![3.0], None);
        let b = simple(vec![1.0, 2.0], Some('x'));
        let result = multiply_simple(&a, &b);
        assert_eq!(result.coefficients, vec![3.0, 6.0]);
    }

    #[test]
    fn test_multiply_two_binomials() {
        // (x + 1)(x - 1) = x^2 - 1
        // coeffs: [1, 1] * [-1, 1] = [-1, 0, 1]
        let a = simple(vec![1.0, 1.0], Some('x'));
        let b = simple(vec![-1.0, 1.0], Some('x'));
        let result = multiply_simple(&a, &b);
        assert_eq!(result.coefficients, vec![-1.0, 0.0, 1.0]);
    }

    #[test]
    fn test_multiply_by_empty() {
        let a = simple(vec![1.0, 2.0], Some('x'));
        let b = simple(vec![], None);
        let result = multiply_simple(&a, &b);
        assert!(result.is_empty());
    }

    #[test]
    fn test_expand_power_zero() {
        let p = simple(vec![-3.0, 4.0], Some('x'));
        let result = expand_simple(&p, 0);
        assert_eq!(result.coefficients, vec![1.0]);
    }

    #[test]
    fn test_expand_power_one() {
        let p = simple(vec![-3.0, 4.0], Some('x'));
        let result = expand_simple(&p, 1);
        assert_eq!(result.coefficients, vec![-3.0, 4.0]);
    }

    #[test]
    fn test_expand_binomial_squared() {
        // (x + 1)^2 = x^2 + 2x + 1
        // coeffs: [1, 1]^2 = [1, 2, 1]
        let p = simple(vec![1.0, 1.0], Some('x'));
        let result = expand_simple(&p, 2);
        assert_eq!(result.coefficients, vec![1.0, 2.0, 1.0]);
    }

    #[test]
    fn test_expand_4x_minus_3_cubed() {
        // (4x - 3)^3 = 64x^3 - 144x^2 + 108x - 27
        // coeffs: [-3, 4]^3 = [-27, 108, -144, 64]
        let p = simple(vec![-3.0, 4.0], Some('x'));
        let result = expand_simple(&p, 3);
        assert_eq!(result.coefficients, vec![-27.0, 108.0, -144.0, 64.0]);
    }

    #[test]
    fn test_expand_trinomial_squared() {
        // (x^2 + x + 1)^2 = x^4 + 2x^3 + 3x^2 + 2x + 1
        // coeffs: [1, 1, 1]^2 = [1, 2, 3, 2, 1]
        let p = simple(vec![1.0, 1.0, 1.0], Some('x'));
        let result = expand_simple(&p, 2);
        assert_eq!(result.coefficients, vec![1.0, 2.0, 3.0, 2.0, 1.0]);
    }
}
