use crate::reduction::matrix::francis::{complex, primitives, symmetric};

// Recommended parameters in constants
// Note: For real-world inputs like A^T*A covariance matrices, explicit forming
// squares condition numbers, so a pre-QR step or an explicit symmetrization
// pass on the tridiagonal output can help keep float drift in check.

/// francis_qr_sym
///
/// * h: householder
/// * p: projection vector
/// * w: workspace for a givens rotation
/// * range: number of rows in active window
/// * size: static number of rows for rotations
/// * max_iters: number of iterations per eigen vector recoups half on success
/// * tolerance: error tolerance which is used as a bound for non relative error
/// * absolute: absolute bound on error minimum should be less than tolerance
pub fn francis_qr_sym(
    h: &mut [f64],
    p: &mut [f64],
    w: &mut [f64],
    range: usize,
    size: usize,
    stride: usize,
    max_iters: usize,
    tolerance: f64,
    absolute: f64,
) {
    primitives::hessenberg(h, p, w, size, range, stride);
    symmetric::decomp_sym(h, range, size, stride, max_iters, tolerance, absolute);
}
/// francis_qr_complex
///
/// * h: householder
/// * p: projection vector
/// * w: workspace for a givens rotation
/// * range: number of rows in active window
/// * size: static number of rows for rotations
/// * max_iters: number of iterations per eigen vector recoups half on success
/// * tolerance: error tolerance which is used as a bound for non relative error
pub fn francis_qr_cpx(
    h: &mut [f64],
    p: &mut [f64],
    w: &mut [f64],
    range: usize,
    size: usize,
    stride: usize,
    max_iters: usize,
    tolerance: f64,
) {
    primitives::hessenberg(h, p, w, size, range, stride);
    complex::decomp_cpx(h, w, range, size, stride, max_iters, tolerance);
}
#[cfg(test)]
mod test_francis_interface {
    use super::*;

    use crate::reduction::matrix::francis::constants::{ABSOLUTE_CAP, MAX_ITERS, TOLERANCE};
    use rand::prelude::*;

    use rand_distr::StandardNormal;
    fn approx_scalar_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < TOLERANCE
    }
    fn approx_vector_eq(a: &[f64], b: &[f64]) -> bool {
        let n = a.len();
        let mut error = 0f64;
        for i in 0..n {
            if a[i].is_nan() || b[i].is_nan() {
                return false;
            }
            error += (a[i] - b[i]).abs();
        }
        error / (n as f64).sqrt() < TOLERANCE
    }
    //  NOTE: This should also be weighted towards the size of the dimensionality
    //  of the decomposition ie the condition number not a flat tolerance level
    fn generate_random_vector(n: usize) -> Vec<f64> {
        let mut rng = rand::rng();
        let mut data = vec![0f64; n];
        for i in 0..n {
            data[i] = rng.sample(StandardNormal);
        }
        data
    }
    fn generate_identity_vector(m: usize, n: usize) -> Vec<f64> {
        let mut vector = vec![0f64; m * n];
        let mut idx = 0;
        for _ in 0..m {
            vector[idx] = 1f64;
            idx += 1 + n;
        }
        vector
    }
    fn generate_strict_symmetric_vector(n: usize) -> Vec<f64> {
        let mut data = generate_random_vector(n * n);
        for i in 0..n {
            for j in 0..i {
                let val = data[i * n + j];
                data[j * n + i] = val;
            }
        }
        data
    }
    /// Creates some f64 style noise in order to replicate working with matrices
    pub fn generate_approx_symmetric_vector(n: usize) -> Vec<f64> {
        let a = generate_random_vector(n * n); // flat, row-major, stride = n
        let mut result = vec![0f64; n * n];
        for i in 0..n {
            for j in 0..n {
                let mut sum = 0f64;
                for k in 0..n {
                    // A[i][k] * A[j][k]  ==  (A * A^T)[i][j]
                    sum += a[i * n + k] * a[j * n + k];
                }
                result[i * n + j] = sum;
            }
        }
        result
    }
    fn trace(data: &[f64], n: usize, stride: usize) -> f64 {
        (0..n).map(|i| data[i * stride + i]).sum()
    }
    fn check_francis_qr_sym() -> (bool, bool) {
        let c = 6;
        let stride = c;
        let mut h = generate_approx_symmetric_vector(c);
        let mut p = vec![0f64; c];
        let mut w = vec![0f64; c];

        let original_trace = trace(&h, c, stride);

        francis_qr_sym(
            &mut h,
            &mut p,
            &mut w,
            c,
            c,
            stride,
            MAX_ITERS,
            TOLERANCE,
            ABSOLUTE_CAP,
        );

        let final_trace = trace(&h, c, stride);
        let trace_ok = approx_scalar_eq(original_trace, final_trace);
        (true, trace_ok)
    }
    fn check_francis_qr_cpx() -> (bool, bool) {
        let c = 6;
        let stride = c;
        let mut h = generate_random_vector(c * c);
        let mut p = vec![0f64; c];
        let mut w = vec![0f64; c];

        let original_trace = trace(&h, c, stride);

        francis_qr_cpx(&mut h, &mut p, &mut w, c, c, stride, MAX_ITERS, TOLERANCE);

        let final_trace = trace(&h, c, stride);
        let trace_ok = approx_scalar_eq(original_trace, final_trace);

        (true, trace_ok)
    }
    #[test]
    fn test_francis_qr_sym() {
        let trials = 10_000;
        let mut trace_failures = 0;
        for _ in 0..trials {
            let (_, trace_ok) = check_francis_qr_sym();
            if !trace_ok {
                trace_failures += 1;
            }
        }
        println!("francis_qr_sym: {trace_failures} trace mismatches / {trials}");
        assert!(
            trace_failures < 10,
            "too many trace mismatches: {trace_failures}"
        );
    }
    #[test]
    fn test_francis_qr_cpx() {
        let trials = 10_000;
        let mut trace_failures = 0;
        for _ in 0..trials {
            let (_, trace_ok) = check_francis_qr_cpx();
            if !trace_ok {
                trace_failures += 1;
            }
        }
        println!("francis_qr_cpx: {trace_failures} trace mismatches / {trials}");
        assert!(
            trace_failures < 10,
            "too many trace mismatches: {trace_failures}"
        );
    }
}
