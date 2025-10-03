use crate::{derivative, eval_polynomial, parse_polynomial};

pub fn bisection(poly: &str, xl: f64, xu: f64, es: f64, imax: usize, xr: f64) -> f64 {
    let mut iter = 0;
    let mut ea = 100.0;
    let mut xu = xu;
    let mut xl = xl;
    let mut xr = xr;
    let first_dx = {
        let parsed = parse_polynomial(poly);
        derivative(&parsed)
    };
    loop {
        let xr_old = xr;
        xr = (xl + xu) / 2 as f64;
        if xr != 0 as f64 {
            ea = {
                let absv = xr - xr_old;
                (absv.abs() / xr) * 100 as f64
            };
        }
        let test = eval_polynomial(xl, &first_dx) * eval_polynomial(xr, &first_dx);
        if test < 0 as f64 {
            xu = xr;
        } else if test > 0 as f64 {
            xl = xr;
        } else {
            ea = 0.0;
        }
        if ea < es || iter >= imax {
            break;
        }
        iter += 1;
    }
    xr
}
