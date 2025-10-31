use crate::solvers::{SolveMode, SolverError};

pub struct Bounds {
    lower: f64,
    init: f64,
    upper: f64,
}

pub fn bisection<F, G, T>(
    polynomial: T,
    derivative_func: F,
    eval: G,
    bounds: Bounds,
    error_tol: f64,
    itermax: usize,
    mode: SolveMode,
) -> Result<f64, SolverError>
where
    F: Fn(&[f64]) -> Vec<f64>,
    G: Fn(f64, &[f64]) -> f64,
    T: AsRef<[f64]>,
{
    let mut iter = 0;
    let mut approx_err = 100.0;
    let mut lower_bound = bounds.lower;
    let mut x_curr = bounds.init;
    let mut upper_bound = bounds.upper;

    if x_curr < lower_bound || x_curr > upper_bound {
        return Err(SolverError::XInitOutOfBounds);
    }

    let poly = polynomial.as_ref().to_vec();
    let poly_vec = {
        match mode {
            SolveMode::Root => poly,
            SolveMode::Extrema => derivative_func(&poly),
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
        let test = eval(lower_bound, &poly_vec) * eval(x_curr, &poly_vec);
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

    let poss_sol = eval(x_curr, &poly_vec);
    if poss_sol.abs() < 1e-4 {
        Ok(x_curr)
    } else {
        Err(SolverError::NoConvergence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivatives::simple_derivative;
    use crate::polynomials::{eval_simple_polynomial, parse_simple_polynomial};

    const ERROR_TOL: f64 = 0.00001;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    #[test]
    fn test_root_success() {
        let poly = "x^2 - 4";
        let parsed = parse_simple_polynomial(poly).unwrap();
        let result = bisection(
            &parsed,
            simple_derivative,
            eval_simple_polynomial,
            Bounds {
                lower: 0.0,
                init: 1.0,
                upper: 3.0,
            },
            ERROR_TOL,
            100,
            SolveMode::Root,
        );
        assert!(result.is_ok());
        let root = result.unwrap();
        assert!(approx_eq(root, 2.0, ERROR_TOL));
    }

    #[test]
    fn test_extrema_success() {
        // f(x) = -x^2 + 4x + 1 has a maximum at x = 2
        let poly = "-1x^2 + 4x + 1";
        let parsed = parse_simple_polynomial(poly).unwrap();
        let result = bisection(
            &parsed,
            simple_derivative,
            eval_simple_polynomial,
            Bounds {
                lower: 0.0,
                init: 2.0,
                upper: 4.0,
            },
            ERROR_TOL,
            100,
            SolveMode::Extrema,
        );
        assert!(result.is_ok());
        let max_x = result.unwrap();
        assert!(approx_eq(max_x, 2.0, ERROR_TOL));
    }

    #[test]
    fn test_no_convergence() {
        let poly = "x^2 + 10";
        let parsed = parse_simple_polynomial(poly).unwrap();
        let result = bisection(
            &parsed,
            simple_derivative,
            eval_simple_polynomial,
            Bounds {
                lower: -1.0,
                init: 0.0,
                upper: 1.0,
            },
            ERROR_TOL,
            100,
            SolveMode::Root,
        );
        assert!(matches!(result, Err(SolverError::NoConvergence)));
    }

    #[test]
    fn test_no_convergence_2() {
        let poly = "x^2 + 5x + 10";
        let parsed = parse_simple_polynomial(poly).unwrap();
        let result = bisection(
            &parsed,
            simple_derivative,
            eval_simple_polynomial,
            Bounds {
                lower: -1.0,
                init: 0.0,
                upper: 1.0,
            },
            ERROR_TOL,
            100,
            SolveMode::Root,
        );
        assert!(matches!(result, Err(SolverError::NoConvergence)));
    }

    #[test]
    fn test_extrema_success_2() {
        let poly = "x^2 + 10";
        let parsed = parse_simple_polynomial(poly).unwrap();
        let result = bisection(
            &parsed,
            simple_derivative,
            eval_simple_polynomial,
            Bounds {
                lower: -1.0,
                init: 0.0,
                upper: 1.0,
            },
            ERROR_TOL,
            100,
            SolveMode::Extrema,
        );
        assert!(approx_eq(result.unwrap(), 0.0, ERROR_TOL));
    }

    #[test]
    fn test_extrema_success_3() {
        let poly = "x^2 - 5x + 10";
        let parsed = parse_simple_polynomial(poly).unwrap();
        let result = bisection(
            &parsed,
            simple_derivative,
            eval_simple_polynomial,
            Bounds {
                lower: -1.0,
                init: 0.0,
                upper: 3.0,
            },
            ERROR_TOL,
            100,
            SolveMode::Extrema,
        );
        assert!(approx_eq(result.unwrap(), 2.5, ERROR_TOL));
    }

    #[test]
    fn test_invalid_bounds() {
        // Minima is at 2.5, not between -1 and 1
        let poly = "x^2 - 5x + 10";
        let parsed = parse_simple_polynomial(poly).unwrap();
        let result = bisection(
            &parsed,
            simple_derivative,
            eval_simple_polynomial,
            Bounds {
                lower: -1.0,
                init: 0.0,
                upper: 1.0,
            },
            ERROR_TOL,
            100,
            SolveMode::Extrema,
        );
        assert!(matches!(result, Err(SolverError::NoConvergence)));
    }

    #[test]
    fn test_negative_bounds() {
        // f(x) = x^2 - 1 has root at x = -1
        let poly = "x^2 - 1";
        let parsed = parse_simple_polynomial(poly).unwrap();
        let result = bisection(
            &parsed,
            simple_derivative,
            eval_simple_polynomial,
            Bounds {
                lower: -2.0,
                init: -1.0,
                upper: 0.0,
            },
            ERROR_TOL,
            100,
            SolveMode::Root,
        );
        assert!(result.is_ok());
        let root = result.unwrap();
        assert!(approx_eq(root, -1.0, ERROR_TOL));
    }

    #[test]
    fn test_known_maxima() {
        let poly = "-2x^6 - 1.6x^4 + 12x + 1";
        let parsed = parse_simple_polynomial(poly).unwrap();
        let expected = 0.90449;

        let res = bisection(
            &parsed,
            simple_derivative,
            eval_simple_polynomial,
            Bounds {
                lower: 0.0,
                init: 0.6,
                upper: 1.0,
            },
            ERROR_TOL,
            1000,
            SolveMode::Extrema,
        )
        .unwrap();

        assert!(
            (res - expected).abs() < 0.01,
            "Expected {} but got {}. Difference is too large.",
            expected,
            res
        );
    }
}
