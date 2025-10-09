use crate::solvers::SolveMode;
use crate::{derivative, eval_polynomial, parse_polynomial};

pub fn bisection(
    poly: &str,
    lower_bound: f64,
    upper_bound: f64,
    error_tol: f64,
    itermax: usize,
    x_init: f64,
    mode: SolveMode,
) -> Option<f64> {
    let mut iter = 0;
    let mut approx_err = 100.0;
    let mut upper_bound = upper_bound;
    let mut lower_bound = lower_bound;
    let mut x_curr = x_init;
    let poly_vec = {
        let parsed = parse_polynomial(poly);
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
        let test = eval_polynomial(lower_bound, &poly_vec) * eval_polynomial(x_curr, &poly_vec);
        if test < 0 as f64 {
            upper_bound = x_curr;
        } else if test > 0 as f64 {
            lower_bound = x_curr;
        } else {
            approx_err = 0.0;
        }
        if approx_err < error_tol || iter >= itermax {
            break;
        }
        iter += 1;
    }
    if iter >= itermax {
        return None;
    }
    Some(x_curr)
}
