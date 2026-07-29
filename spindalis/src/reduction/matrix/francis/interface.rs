use crate::decomposition::francis::{complex, primitives, symmetric};

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
    h: &mut [f32],
    p: &mut [f32],
    w: &mut [f32],
    range: usize,
    size: usize,
    stride: usize,
    max_iters: usize,
    tolerance: f32,
    absolute: f32,
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
    h: &mut [f32],
    p: &mut [f32],
    w: &mut [f32],
    range: usize,
    size: usize,
    stride: usize,
    max_iters: usize,
    tolerance: f32,
) {
    primitives::hessenberg(h, p, w, size, range, stride);
    complex::decomp_cpx(h, w, range, size, stride, max_iters, tolerance);
}

mod test_francis_interface {
    use super::*;

    use crate::decomposition::francis::constants::{ABSOLUTE_CAP, MAX_ITERS, TOLERANCE};
    use crate::equality::approximate::approx_scalar_eq;
    use crate::random::generation::{generate_approx_symmetric_vector, generate_random_vector};
    fn trace(data: &[f32], n: usize, stride: usize) -> f32 {
        (0..n).map(|i| data[i * stride + i]).sum()
    }
    fn check_francis_qr_sym() -> (bool, bool) {
        let c = 6;
        let stride = c;
        let mut h = generate_approx_symmetric_vector(c);
        let mut p = vec![0f32; c];
        let mut w = vec![0f32; c];

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
        let mut p = vec![0f32; c];
        let mut w = vec![0f32; c];

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
