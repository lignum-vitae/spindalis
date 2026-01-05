use crate::integrals::IntegralError;
use crate::polynomials::structs::PolynomialTraits;

pub fn definite_integral<P>(
    poly: &P,
    start: f64,
    end: f64,
    segments: usize,
) -> Result<f64, IntegralError>
where
    P: PolynomialTraits,
{
    let segment_width = (end - start) / segments as f64;
    let mut sum = 0.0;
    if segments == 1 {
        let res = trapezoidal_rule(poly, start, end, segments)?;
        return Ok(res);
    }
    let mut remaining_segments = segments;
    if !segments.is_multiple_of(2) {
        // Last four points encapsulate last three segments
        let mut points = [0.0; 4];
        for (i, point) in points.iter_mut().enumerate().skip(1) {
            *point = end - segment_width * i as f64;
        }
        points[0] = end;
        points.reverse();
        sum += simpson38(segment_width, poly, points)?;
        remaining_segments -= 3;
    }
    if remaining_segments > 1 {
        sum += simpson13(segment_width, poly, start, remaining_segments)?;
    }
    Ok(sum)
}

pub fn romberg_definite<P>(
    poly: &P,
    start: f64,
    end: f64,
    maxiter: u32,
    tolerance: f64,
) -> Result<f64, IntegralError>
where
    P: PolynomialTraits,
{
    let maxiter = maxiter as usize;
    let mut romberg_table: Vec<Vec<f64>> = vec![vec![0.0; 10]; 10];
    let mut iter = 0_usize;
    let mut segments = 1;
    romberg_table[1][1] = trapezoidal_rule(poly, start, end, segments)?;

    loop {
        iter += 1;
        segments = 2_u32.pow(iter as u32) as usize;

        romberg_table[iter + 1][1] = trapezoidal_rule(poly, start, end, segments)?;
        for k in 2..=iter + 1 {
            let j = 2 + iter - k;
            let p = 4_usize.pow((k as u32) - 1) as f64;

            romberg_table[j][k] =
                (p * romberg_table[j + 1][k - 1] - romberg_table[j][k - 1]) / (p - 1.0);
        }
        let approx_err = ((romberg_table[1][iter + 1] - romberg_table[2][iter]).abs()
            / romberg_table[1][iter + 1])
            .abs()
            * 100.0;
        if iter >= maxiter || approx_err <= tolerance {
            break;
        }
    }
    if iter >= maxiter {
        return Err(IntegralError::MaxIterationsReached);
    }
    Ok(romberg_table[1][iter + 1])
}

fn trapezoidal_rule<P>(
    poly: &P,
    start: f64,
    end: f64,
    segments: usize,
) -> Result<f64, IntegralError>
where
    P: PolynomialTraits,
{
    let mut xi = start;
    let segment_width = (end - start) / segments as f64;
    let mut sum = poly.eval_univariate(xi)?;
    for _ in 1..segments {
        xi += segment_width;
        sum += 2_f64 * poly.eval_univariate(xi)?;
    }
    sum += poly.eval_univariate(end)?;

    Ok(segment_width * sum / 2_f64)
}

// Handles an odd number of segments
fn simpson38<P>(segment_width: f64, poly: &P, points: [f64; 4]) -> Result<f64, IntegralError>
where
    P: PolynomialTraits,
{
    let mut f: Vec<f64> = Vec::new();
    for point in points {
        let x = poly.eval_univariate(point)?;
        f.push(x)
    }
    Ok(3_f64 * segment_width * (f[0] + 3_f64 * f[1] + 3_f64 * f[2] + f[3]) / 8_f64)
}

// Handles an even number of segments
fn simpson13<P>(
    segment_width: f64,
    poly: &P,
    start: f64,
    segments: usize,
) -> Result<f64, IntegralError>
where
    P: PolynomialTraits,
{
    let mut xi = start;
    let mut sum = poly.eval_univariate(xi)?;
    for _ in 1..segments / 2 {
        xi += 2_f64 * segment_width;
        sum +=
            4_f64 * poly.eval_univariate(xi - segment_width)? + 2_f64 * poly.eval_univariate(xi)?;
    }
    xi += 2_f64 * segment_width;
    sum += 4_f64 * poly.eval_univariate(xi - segment_width)? + poly.eval_univariate(xi)?;

    Ok(segment_width * sum / 3_f64)
}

pub fn analytical_integral<P>(poly: &P, a: f64, b: f64) -> Result<f64, IntegralError>
where
    P: PolynomialTraits,
{
    // ∫ₐᵇ x dx = F(b) − F(a)

    let integrated_polynomial = poly.indefinite_integral_univariate()?;
    let fa = integrated_polynomial.eval_univariate(a)?;
    let fb = integrated_polynomial.eval_univariate(b)?;

    Ok(fb - fa)
}
