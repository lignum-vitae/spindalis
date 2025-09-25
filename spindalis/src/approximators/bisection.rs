pub fn bisection(xl: f64, xu: f64, es: f64, imax: usize, xr: f64) -> f64 {
    let mut iter = 0;
    let mut ea = 100.0;
    let mut xu = xu;
    let mut xl = xl;
    let mut xr = xr;
    loop {
        let xr_old = xr;
        xr = (xl + xu) / 2 as f64;
        if xr != 0 as f64 {
            ea = {
                let absv = xr - xr_old;
                (absv.abs() / xr) * 100 as f64
            };
        }
        let test = first_dx(xl) * first_dx(xr);
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
