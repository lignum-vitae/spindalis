use crate::reduction::matrix::francis::{complex, primitives, symmetric};
use crate::solvers::SolverError;
use jedvek::Matrix2D;

const DEFAULT_MAX_ITERS: usize = 100;
const DEFAULT_TOLERANCE: f64 = 1e-10;
const DEFAULT_ABSOLUTE: f64 = 1e-12;

// TODO: make test which enforces that the full_* or renamed fns are direct mirrors
// without any epsilon ie they should be direct copies for all matrices

// INVARIANT: every "full_*" function in this file is a 1:1 mirror of its
// non-"full_" counterpart (same reduction logic, same reflectors, same
// order of operations) — the only difference is that "full_*" versions
// additionally accumulate the rotation/Q matrix, at ~2x the flop cost.
// Non-"full_" versions exist purely for callers who don't need Q.
//
// If you change the core reduction logic in one member of a pair, you
// MUST make the identical change in its counterpart. There is currently
// no compiler-level enforcement of this — a change to only one side will
// build and pass unrelated tests silently.
//
// Enforced only by: <name of the N-random-matrix agreement test(s)>.
// Run that test after touching ANY function here before considering
// the change complete.

// Recommended parameters in constants
// Note: For real-world inputs like A^T*A covariance matrices, explicit forming
// squares condition numbers, so a pre-QR step or an explicit symmetrization
// pass on the tridiagonal output can help keep float drift in check.

pub fn auto_francis_qr_sym(matrix: &Matrix2D<f64>) -> Result<Matrix2D<f64>, SolverError> {
    if matrix.height != matrix.width {
        return Err(SolverError::NonSquareMatrix);
    }
    let n = matrix.height;
    let stride = n;

    let mut h: Vec<f64> = matrix.rows().flatten().copied().collect();
    let mut p = vec![0f64; n];
    let mut w = vec![0f64; n];

    francis_qr_sym(
        &mut h,
        &mut p,
        &mut w,
        n,
        n,
        stride,
        DEFAULT_MAX_ITERS,
        DEFAULT_TOLERANCE,
        DEFAULT_ABSOLUTE,
    );
    Matrix2D::from_flat(h, 0.0, n, n).map_err(SolverError::InvalidVector)
}
pub fn auto_francis_qr_cpx(matrix: &Matrix2D<f64>) -> Result<Matrix2D<f64>, SolverError> {
    if matrix.height != matrix.width {
        return Err(SolverError::NonSquareMatrix);
    }
    let n = matrix.height;
    let stride = n;

    let mut h: Vec<f64> = matrix.rows().flatten().copied().collect();
    let mut p = vec![0f64; n];
    let mut w = vec![0f64; n];
    francis_qr_cpx(
        &mut h,
        &mut p,
        &mut w,
        n,
        n,
        stride,
        DEFAULT_MAX_ITERS,
        DEFAULT_TOLERANCE,
    );
    Matrix2D::from_flat(h, 0.0, n, n).map_err(SolverError::InvalidVector)
}
/// francis_qr_sym
///
/// * p: projection vector
/// * w: workspace for a givens rotation
/// * range: number of rows in active window
/// * size: static number of rows for rotations
/// * max_iters: number of iterations per eigen vector recoups half on success
/// * tolerance: error tolerance which is used as a bound for non relative error
/// * absolute: absolute bound on error minimum should be less than tolerance
pub fn francis_qr_sym(
    lin_matrix: &mut [f64],
    projection: &mut [f64],
    workspace: &mut [f64],
    range: usize,
    size: usize,
    stride: usize,
    max_iters: usize,
    tolerance: f64,
    absolute: f64,
) {
    primitives::hessenberg(lin_matrix, projection, workspace, size, range, stride);
    symmetric::decomp_sym(
        lin_matrix, range, size, stride, max_iters, tolerance, absolute,
    );
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
    lin_matrix: &mut [f64],
    projection: &mut [f64],
    workspace: &mut [f64],
    range: usize,
    size: usize,
    stride: usize,
    max_iters: usize,
    tolerance: f64,
) {
    primitives::hessenberg(lin_matrix, projection, workspace, size, range, stride);
    complex::decomp_cpx(
        lin_matrix, projection, workspace, range, size, stride, max_iters, tolerance,
    );
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
    fn generate_random_vector(n: usize) -> Vec<f64> {
        let mut rng = rand::rng();
        let mut data = vec![0f64; n];
        for d in data.iter_mut().take(n) {
            *d = rng.sample(StandardNormal);
        }
        data
    }
    /// Creates some f64 style noise in order to replicate working with matrices
    fn generate_approx_symmetric_vector(n: usize) -> Vec<f64> {
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
