use crate::integrals::IntegralError;
use crate::integrals::simple_indefinite::indefinite_integral;
use crate::polynomials::simple::eval_simple_polynomial;

pub fn definite_integral<G>(
    poly: impl AsRef<[f64]>,
    eval: G,
    start: f64,
    end: f64,
    segments: usize,
) -> f64
where
    G: Fn(f64, &[f64]) -> f64,
{
    let poly = poly.as_ref();
    let segment_width = (end - start) / segments as f64;
    let mut sum = 0.0;
    if segments == 1 {
        return trapezoidal_rule(poly, &eval, start, end, segments);
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
        sum += simpson38(segment_width, poly, &eval, points);
        remaining_segments -= 3;
    }
    if remaining_segments > 1 {
        sum += simpson13(segment_width, poly, &eval, start, remaining_segments);
    }
    sum
}

pub fn romberg_definite<G>(
    poly: impl AsRef<[f64]>,
    eval: G,
    start: f64,
    end: f64,
    maxiter: u32,
    tolerance: f64,
) -> Result<f64, IntegralError>
where
    G: Fn(f64, &[f64]) -> f64,
{
    let poly = poly.as_ref();
    let maxiter = maxiter as usize;
    let mut romberg_table: Vec<Vec<f64>> = vec![vec![0.0; 10]; 10];
    let mut iter = 0_usize;
    let mut segments = 1;
    romberg_table[1][1] = trapezoidal_rule(poly, &eval, start, end, segments);

    loop {
        iter += 1;
        segments = 2_u32.pow(iter as u32) as usize;

        romberg_table[iter + 1][1] = trapezoidal_rule(poly, &eval, start, end, segments);
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

fn trapezoidal_rule<G>(poly: &[f64], eval: G, start: f64, end: f64, segments: usize) -> f64
where
    G: Fn(f64, &[f64]) -> f64,
{
    let mut xi = start;
    let segment_width = (end - start) / segments as f64;
    let mut sum = eval(xi, poly);
    for _ in 1..segments {
        xi += segment_width;
        sum += 2_f64 * eval(xi, poly);
    }
    sum += eval(end, poly);

    segment_width * sum / 2_f64
}

// Handles an odd number of segments
fn simpson38<G>(segment_width: f64, poly: &[f64], eval: G, points: [f64; 4]) -> f64
where
    G: Fn(f64, &[f64]) -> f64,
{
    let mut f: Vec<f64> = Vec::new();
    for point in points {
        let x = eval(point, poly);
        f.push(x)
    }
    3_f64 * segment_width * (f[0] + 3_f64 * f[1] + 3_f64 * f[2] + f[3]) / 8_f64
}

// Handles an even number of segments
fn simpson13<G>(segment_width: f64, poly: &[f64], eval: G, start: f64, segments: usize) -> f64
where
    G: Fn(f64, &[f64]) -> f64,
{
    let mut xi = start;
    let mut sum = eval(xi, poly);
    for _ in 1..segments / 2 {
        xi += 2_f64 * segment_width;
        sum += 4_f64 * eval(xi - segment_width, poly) + 2_f64 * eval(xi, poly);
    }
    xi += 2_f64 * segment_width;
    sum += 4_f64 * eval(xi - segment_width, poly) + eval(xi, poly);

    segment_width * sum / 3_f64
}

pub fn analytical_integral(poly: impl AsRef<[f64]>, a: f64, b: f64) -> f64 {
    // ∫ₐᵇ x dx = F(b) − F(a)

    let poly = poly.as_ref();
    let integrated_polynomial = indefinite_integral(poly);
    let fa = eval_simple_polynomial(a, &integrated_polynomial);
    let fb = eval_simple_polynomial(b, &integrated_polynomial);

    fb - fa
}
