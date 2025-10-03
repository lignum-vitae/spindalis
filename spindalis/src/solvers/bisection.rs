use crate::solvers::SolveMode;
use crate::{derivative, eval_polynomial, parse_polynomial};

pub fn bisection(
    poly: &str,
    lower_bound: f64,
    upper_bound: f64,
    error_tol: f64,
    itermax: usize,
    root: f64,
    mode: SolveMode,
) -> f64 {
    let mut iter = 0;
    let mut approx_err = 100.0;
    let mut upper_bound = upper_bound;
    let mut lower_bound = lower_bound;
    let mut root = root;
    let poly_vec = {
        let parsed = parse_polynomial(poly);
        match mode {
            SolveMode::Root => parsed,
            SolveMode::Extrema => derivative(&parsed),
        }
    };
    loop {
        let old_root = root;
        root = (lower_bound + upper_bound) / 2 as f64;
        if root != 0 as f64 {
            approx_err = {
                let absv = root - old_root;
                (absv.abs() / root) * 100 as f64
            };
        }
        let test = eval_polynomial(lower_bound, &poly_vec) * eval_polynomial(root, &poly_vec);
        if test < 0 as f64 {
            upper_bound = root;
        } else if test > 0 as f64 {
            lower_bound = root;
        } else {
            approx_err = 0.0;
        }
        if approx_err < error_tol || iter >= itermax {
            break;
        }
        iter += 1;
    }
    root
}
