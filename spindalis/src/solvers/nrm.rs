use crate::solvers::SolveMode;
use crate::{derivative, eval_polynomial, parse_polynomial};

pub fn newton_raphson_method(
    poly: &str,
    x0: f64,
    itermax: usize,
    error_tol: f64,
    mode: SolveMode,
) -> Option<f64> {
    let mut iter = 0;
    let mut root = x0;
    let mut approx_err = 100 as f64;
    let poly_vec = {
        let parsed = parse_polynomial(&poly);
        match mode {
            SolveMode::Root => parsed,
            SolveMode::Extrema => derivative(&parsed),
        }
    };
    let poly_vec_dx = derivative(&poly_vec);
    loop {
        let xr_old = root;
        root = xr_old - (eval_polynomial(root, &poly_vec) / eval_polynomial(root, &poly_vec_dx));
        let poss_root = eval_polynomial(root, &poly_vec);
        iter += 1;
        if root != 0 as f64 {
            approx_err = ((root - xr_old).abs() / root) * 100.0;
        }
        if (approx_err < error_tol || iter >= itermax) && poss_root.abs() < 1e-9 {
            break;
        }
    }
    if iter >= itermax {
        return None;
    }
    Some(root)
}
