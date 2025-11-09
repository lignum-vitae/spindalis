use crate::solvers::{SolveMode, SolverError};

pub fn newton_raphson_method<F, G, T>(
    polynomial: T,
    derivative_func: F,
    eval: G,
    x_init: f64,
    itermax: usize,
    error_tol: f64,
    mode: SolveMode,
) -> Result<f64, SolverError>
where
    F: Fn(&[f64]) -> Vec<f64>,
    G: Fn(f64, &[f64]) -> f64,
    T: AsRef<[f64]>,
{
    let mut iter = 0;
    let mut x_curr = x_init;
    let mut approx_err = 100_f64;
    let poly = polynomial.as_ref().to_vec();
    let poly_vec = {
        match mode {
            SolveMode::Root => poly,
            SolveMode::Extrema => derivative_func(&poly),
        }
    };
    let poly_vec_dx = derivative_func(&poly_vec);
    loop {
        let xr_old = x_curr;
        x_curr = xr_old - (eval(x_curr, &poly_vec) / eval(x_curr, &poly_vec_dx));
        iter += 1;
        if x_curr != 0 as f64 {
            approx_err = ((x_curr - xr_old).abs() / x_curr) * 100.0;
        }
        if approx_err.abs() < error_tol || iter >= itermax {
            break;
        }
    }
    if iter >= itermax {
        return Err(SolverError::MaxIterationsReached);
    }
    Ok(x_curr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivatives::simple_derivative;
    use crate::polynomials::{eval_simple_polynomial, parse_simple_polynomial};

    const ERROR_TOL: f64 = 0.0001;

    #[test]
    fn test_root_success() {
        let poly = "x^2 - 4";
        let parsed = parse_simple_polynomial(poly).unwrap();
        let result = newton_raphson_method(
            parsed,
            simple_derivative,
            eval_simple_polynomial,
            2.0,
            100,
            ERROR_TOL,
            SolveMode::Root,
        );
        assert!(result.is_ok());
        let root = result.unwrap();
        assert!((root - 2.0).abs() < ERROR_TOL);
    }

    #[test]
    fn test_no_convergence() {
        let poly = "x^2 + 4";
        let parsed = parse_simple_polynomial(poly).unwrap();
        let result = newton_raphson_method(
            parsed,
            simple_derivative,
            eval_simple_polynomial,
            2.0,
            100,
            ERROR_TOL,
            SolveMode::Root,
        );
        assert!(matches!(result, Err(SolverError::MaxIterationsReached)));
    }

    #[test]
    fn test_extrema_success() {
        let poly = "-x^2 + 4x"; // Derivative: -2x + 4 = 0 → x = 2
        let parsed = parse_simple_polynomial(poly).unwrap();
        let result = newton_raphson_method(
            parsed,
            simple_derivative,
            eval_simple_polynomial,
            0.0,
            100,
            ERROR_TOL,
            SolveMode::Extrema,
        );
        assert!(result.is_ok());
        let x = result.unwrap();
        assert!((x - 2.0).abs() < ERROR_TOL);
    }

    #[test]
    fn test_multiple_roots() {
        let guesses = [0.0, 1.0, 2.0];
        let poly = "0.5x^3-3.9x^2+6x-1.5";
        let parsed = parse_simple_polynomial(poly).unwrap();
        let expected_results = [0.30997, 5.82992, 1.66011];
        let mut results: Vec<f64> = Vec::new();

        for guess in guesses {
            let res = newton_raphson_method(
                &parsed,
                simple_derivative,
                eval_simple_polynomial,
                guess,
                100,
                ERROR_TOL,
                SolveMode::Root,
            );
            match res {
                Ok(x) => results.push(x),
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
        }
        for (result, &expected) in results.iter().zip(expected_results.iter()) {
            // Check if the absolute difference between result and expected is less than 0.01 (for 2 decimal places)
            assert!(
                (result - expected).abs() < 0.01,
                "Expected {expected} but got {result}. Difference is too large.",
            );
        }
    }

    #[test]
    fn test_multiple_extrema() {
        let guesses = [0.0, 5.0];
        let poly = "0.5x^3-3.9x^2+6x-1.5";
        let parsed = parse_simple_polynomial(poly).unwrap();
        let expected_results = [0.93868, 4.26132];
        let mut results: Vec<f64> = Vec::new();

        for guess in guesses {
            let res = newton_raphson_method(
                &parsed,
                simple_derivative,
                eval_simple_polynomial,
                guess,
                100,
                0.01,
                SolveMode::Extrema,
            );
            match res {
                Ok(x) => results.push(x),
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
        }
        for (result, &expected) in results.iter().zip(expected_results.iter()) {
            // Check if the absolute difference between result and expected is less than 0.01 (for 2 decimal places)
            assert!(
                (result - expected).abs() < 0.01,
                "Expected {expected} but got {result}. Difference is too large.",
            );
        }
    }
}
