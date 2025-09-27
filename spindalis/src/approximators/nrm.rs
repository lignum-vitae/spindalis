use crate::{derivative, eval_polynomial, parse_polynomial};

pub fn newton_raphson_method(poly: &str, x0: f64, itermax: usize, es: f64) -> Option<f64> {
    let mut iter = 0;
    let mut xr = x0;
    let mut ea = 100 as f64;
    let parsed = parse_polynomial(&poly);
    let first_dx = derivative(&parsed);
    loop {
        let xr_old = xr;
        xr = xr_old - (eval_polynomial(xr, &parsed) / eval_polynomial(xr, &first_dx));
        let poss_root = eval_polynomial(xr, &parsed);
        iter += 1;
        if xr != 0 as f64 {
            ea = ((xr - xr_old).abs() / xr) * 100.0;
        }
        if (ea < es || iter >= itermax) && poss_root.abs() < 1e-9 {
            break;
        }
    }
    if iter >= itermax {
        return None;
    }
    Some(xr)
}
