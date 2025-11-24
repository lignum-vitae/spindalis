use crate::polynomials::PolynomialTraits;
use crate::solvers::{Bounds, SolveMode, SolverError};

pub fn bisection<P>(
    polynomial: &P,
    bounds: Bounds,
    error_tol: f64,
    itermax: usize,
    mode: SolveMode,
) -> Result<f64, SolverError>
where
    P: PolynomialTraits,
{
    let mut iter = 0;
    let mut approx_err = 100.0;
    let mut lower_bound: f64 = bounds.lower;
    let mut x_curr: f64 = bounds.init;
    let mut upper_bound: f64 = bounds.upper;

    if x_curr < lower_bound || x_curr > upper_bound {
        return Err(SolverError::XInitOutOfBounds);
    }

    let polynomial = {
        match mode {
            SolveMode::Root => polynomial,
            SolveMode::Extrema => &polynomial.derivate_univariate()?,
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
        let test = polynomial.eval_univariate(lower_bound)? * polynomial.eval_univariate(x_curr)?;
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

    let poss_sol = polynomial.eval_univariate(x_curr)?;
    if poss_sol.abs() < 1e-4 {
        Ok(x_curr)
    } else {
        Err(SolverError::NoConvergence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::polynomials::{PolynomialExtended, SimplePolynomial};

    const ERROR_TOL: f64 = 0.00001;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    #[test]
    fn test_root_success() {
        let poly = "x^2 - 4";
        let parsed = SimplePolynomial::parse(poly).unwrap();
        let result = bisection(
            &parsed,
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
    fn test_root_success_extended_poly() {
        let poly = "x^2 - 4";
        let parsed = PolynomialExtended::parse(poly).unwrap();
        let result = bisection(
            &parsed,
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
        let parsed = SimplePolynomial::parse(poly).unwrap();
        let result = bisection(
            &parsed,
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
    fn test_extrema_success_extended_poly() {
        // f(x) = -x^2 + 4x + 1 has a maximum at x = 2
        let poly = "-1x^2 + 4x + 1";
        let parsed = PolynomialExtended::parse(poly).unwrap();
        let result = bisection(
            &parsed,
            Bounds {
                lower: 0.0,
                init: 2.0,
                upper: 4.0,
            },
            ERROR_TOL,
            100,
            SolveMode::Extrema,
        );
        // assert!(result.is_ok());
        let max_x = result.unwrap();
        assert!(approx_eq(max_x, 2.0, ERROR_TOL));
    }

    #[test]
    fn test_no_convergence() {
        let poly = "x^2 + 10";
        let parsed = SimplePolynomial::parse(poly).unwrap();
        let result = bisection(
            &parsed,
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
        let parsed = SimplePolynomial::parse(poly).unwrap();
        let result = bisection(
            &parsed,
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
        let parsed = SimplePolynomial::parse(poly).unwrap();
        let result = bisection(
            &parsed,
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
        let parsed = SimplePolynomial::parse(poly).unwrap();
        let result = bisection(
            &parsed,
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
        let parsed = SimplePolynomial::parse(poly).unwrap();
        let result = bisection(
            &parsed,
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
        let parsed = SimplePolynomial::parse(poly).unwrap();
        let result = bisection(
            &parsed,
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
        let parsed = SimplePolynomial::parse(poly).unwrap();
        let expected = 0.90449;

        let res = bisection(
            &parsed,
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
            "Expected {expected} but got {res}. Difference is too large.",
        );
    }
}
