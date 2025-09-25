pub fn newton_raphson_method(x0: f64, itermax: usize, es: f64) -> Option<f64> {
    let mut iter = 0;
    let mut xr = x0;
    let mut ea = 100 as f64;
    loop {
        let xr_old = xr;
        xr = xr_old - (eval_polynomial(xr) / first_dx(xr));
        let poss_root = eval_polynomial(xr);
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
