use crate::solvers::SolveMode;
use crate::{derivative, eval_polynomial, parse_polynomial};

pub fn newton_raphson_method(
    poly: &str,
    x_init: f64,
    itermax: usize,
    error_tol: f64,
    mode: SolveMode,
) -> Option<f64> {
    let mut iter = 0;
    let mut x_curr = x_init;
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
        let xr_old = x_curr;
        x_curr =
            xr_old - (eval_polynomial(x_curr, &poly_vec) / eval_polynomial(x_curr, &poly_vec_dx));
        let poss_x_curr = eval_polynomial(x_curr, &poly_vec);
        iter += 1;
        if x_curr != 0 as f64 {
            approx_err = ((x_curr - xr_old).abs() / x_curr) * 100.0;
        }
        if (approx_err < error_tol || iter >= itermax) && poss_x_curr.abs() < 1e-9 {
            break;
        }
    }
    if iter >= itermax {
        return None;
    }
    Some(x_curr)
}
