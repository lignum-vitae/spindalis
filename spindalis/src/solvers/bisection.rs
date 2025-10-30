use crate::solvers::{SolveMode, SolverError};
use crate::{derivative, eval_simple_polynomial, parse_simple_polynomial};

pub fn bisection(
    poly: &str,
    lower_bound: f64,
    upper_bound: f64,
    error_tol: f64,
    itermax: usize,
    x_init: f64,
    mode: SolveMode,
) -> Result<f64, SolverError> {
    let mut iter = 0;
    let mut approx_err = 100.0;
    let mut upper_bound = upper_bound;
    let mut lower_bound = lower_bound;
    let mut x_curr = x_init;
    let poly_vec = {
        let parsed = parse_simple_polynomial(poly).map_err(SolverError::InvalidPolynomial)?;
        match mode {
            SolveMode::Root => parsed,
            SolveMode::Extrema => derivative(&parsed),
        }
    };
    loop {
        let old_x_curr = x_curr;
        x_curr = (lower_bound + upper_bound) / 2_f64;
        if x_curr != 0 as f64 {
            approx_err = {
                let absv = x_curr - old_x_curr;
                (absv.abs() / x_curr) * 100_f64
            };
        }
        let test = eval_simple_polynomial(lower_bound, &poly_vec)
            * eval_simple_polynomial(x_curr, &poly_vec);
        if test < 0 as f64 {
            upper_bound = x_curr;
        } else if test > 0 as f64 {
            lower_bound = x_curr;
        } else {
            approx_err = 0.0;
        }
        if approx_err.abs() < error_tol || iter >= itermax {
            break;
        }
        iter += 1;
    }
    if iter >= itermax {
        return Err(SolverError::MaxIterationsReached);
    }
    Ok(x_curr)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERROR_TOL: f64 = 0.0001;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    #[test]
    fn test_root_success() {
        let poly = "x^2 - 4";
        let result = bisection(poly, 0.0, 3.0, ERROR_TOL, 100, 1.0, SolveMode::Root);
        assert!(result.is_ok());
        let root = result.unwrap();
        assert!(approx_eq(root, 2.0, ERROR_TOL));
    }

    #[test]
    fn test_extrema_success() {
        // f(x) = -x^2 + 4x + 1 has a maximum at x = 2
        let poly = "-1x^2 + 4x + 1";
        let result = bisection(poly, 0.0, 4.0, ERROR_TOL, 100, 2.0, SolveMode::Extrema);
        assert!(result.is_ok());
        let max_x = result.unwrap();
        assert!(approx_eq(max_x, 2.0, ERROR_TOL));
    }

    #[test]
    fn test_invalid_polynomial() {
        let poly = "2x^ + 3x"; // invalid syntax
        let result = bisection(poly, 0.0, 1.0, ERROR_TOL, 100, 0.5, SolveMode::Root);
        assert!(matches!(result, Err(SolverError::InvalidPolynomial(_))));
    }

    #[test]
    fn test_no_convergence() {
        // Flat function on interval: no sign change => no root
        let poly = "1";
        let result = bisection(poly, -1.0, 1.0, ERROR_TOL, 5, 0.0, SolveMode::Root);
        assert!(matches!(result, Err(SolverError::InvalidPolynomial(_))));
    }

    #[test]
    fn test_negative_bounds() {
        // f(x) = x^2 - 1 has root at x = -1
        let poly = "x^2 - 1";
        let result = bisection(poly, -2.0, 0.0, ERROR_TOL, 100, -1.0, SolveMode::Root);
        assert!(result.is_ok());
        let root = result.unwrap();
        assert!(approx_eq(root, -1.0, ERROR_TOL));
    }

    #[test]
    fn test_known_maxima() {
        let polynomial = "-2x^6 - 1.6x^4 + 12x + 1";
        let expected = 0.90449;

        let res = bisection(polynomial, 0.0, 1.0, 5.0, 1000, 0.6, SolveMode::Extrema).unwrap();

        assert!(
            (res - expected).abs() < 0.01,
            "Expected {} but got {}. Difference is too large.",
            expected,
            res
        );
    }
}
