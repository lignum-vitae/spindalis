use crate::polynomials::PolynomialTraits;
use crate::solvers::{SolveMode, SolverError};

pub fn newton_raphson_method<P>(
    polynomial: &P,
    x_init: f64,
    itermax: usize,
    error_tol: f64,
    mode: SolveMode,
) -> Result<f64, SolverError>
where
    P: PolynomialTraits,
{
    let mut iter = 0;
    let mut x_curr = x_init;
    let mut approx_err = 100_f64;
    let polynomial = {
        match mode {
            SolveMode::Root => polynomial,
            SolveMode::Extrema => &polynomial.derivate_univariate()?,
        }
    };
    let polynomial_dx = polynomial.derivate_univariate()?;
    loop {
        let xr_old = x_curr;
        x_curr = xr_old
            - (polynomial.eval_univariate(x_curr)? / polynomial_dx.eval_univariate(x_curr)?);
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
    use crate::polynomials::{PolynomialExtended, SimplePolynomial};

    const ERROR_TOL: f64 = 0.0001;

    #[test]
    fn test_root_success() {
        let poly = "x^2 - 4";
        let parsed = SimplePolynomial::parse(poly).unwrap();
        let result = newton_raphson_method(&parsed, 2.0, 100, ERROR_TOL, SolveMode::Root);
        assert!(result.is_ok());
        let root = result.unwrap();
        assert!((root - 2.0).abs() < ERROR_TOL);
    }

    #[test]
    fn test_root_success_extended_poly() {
        let poly = "x^2 - 4";
        let parsed = PolynomialExtended::parse(poly).unwrap();
        let result = newton_raphson_method(&parsed, 2.0, 100, ERROR_TOL, SolveMode::Root);
        assert!(result.is_ok());
        let root = result.unwrap();
        assert!((root - 2.0).abs() < ERROR_TOL);
    }

    #[test]
    fn test_no_convergence() {
        let poly = "x^2 + 4";
        let parsed = SimplePolynomial::parse(poly).unwrap();
        let result = newton_raphson_method(&parsed, 2.0, 100, ERROR_TOL, SolveMode::Root);
        assert!(matches!(result, Err(SolverError::MaxIterationsReached)));
    }

    #[test]
    fn test_extrema_success() {
        let poly = "-x^2 + 4x"; // Derivative: -2x + 4 = 0 → x = 2
        let parsed = SimplePolynomial::parse(poly).unwrap();
        let result = newton_raphson_method(&parsed, 0.0, 100, ERROR_TOL, SolveMode::Extrema);
        assert!(result.is_ok());
        let x = result.unwrap();
        assert!((x - 2.0).abs() < ERROR_TOL);
    }

    #[test]
    fn test_multiple_roots() {
        let guesses = [0.0, 1.0, 2.0];
        let poly = "0.5x^3-3.9x^2+6x-1.5";
        let parsed = SimplePolynomial::parse(poly).unwrap();
        let expected_results = [0.30997, 5.82992, 1.66011];
        let mut results: Vec<f64> = Vec::new();

        for guess in guesses {
            let res = newton_raphson_method(&parsed, guess, 100, ERROR_TOL, SolveMode::Root);
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
        let parsed = SimplePolynomial::parse(poly).unwrap();
        let expected_results = [0.93868, 4.26132];
        let mut results: Vec<f64> = Vec::new();

        for guess in guesses {
            let res = newton_raphson_method(&parsed, guess, 100, 0.01, SolveMode::Extrema);
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
