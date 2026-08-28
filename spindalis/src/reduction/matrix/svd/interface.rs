use crate::reduction::matrix::svd::bidiagonalization::{lbidiagonal, ubidiagonal};
use crate::reduction::matrix::svd::bulge_chasing::{decomp_lgivens, decomp_ugivens};
use crate::reduction::matrix::svd::primitives::identity_flat;
#[rustfmt::skip]
use crate::reduction::matrix::svd::verify::{
    full_ubidiagonal,
    full_lbidiagonal,
    full_decomp_ugivens,
    full_decomp_lgivens
};
use crate::reduction::matrix::svd::constants::{
    DEFAULT_ABSOLUTE, DEFAULT_MAX_ITERS, DEFAULT_TOLERANCE,
};
use crate::solvers::SolverError;
use jedvek::Matrix2D;

/// auto_svd
///   full SVD, allocating: A ~= U * Sigma * V'
///
/// Returns (U, Sigma, V):
///   - U is rows x rows, orthogonal.
///   - V is cols x cols, orthogonal.
///   - Sigma is rows x cols, same shape as A. Let card = min(rows, cols).
///     Only the top-left card x card corner of Sigma is nonzero, and
///     within that corner only the diagonal is nonzero (the singular
///     values). Everything else in Sigma — off-diagonal in the corner,
///     and all of it outside the corner — is zero.
///
///   Unlike francis_lq_sym/cpx, this does NOT require a square matrix —
///   wide, tall, and square inputs are all valid.
pub fn auto_svd(
    matrix: &Matrix2D<f64>,
) -> Result<(Matrix2D<f64>, Matrix2D<f64>, Matrix2D<f64>), SolverError> {
    let rows = matrix.height;
    let cols = matrix.width;
    let card = rows.min(cols);
    let stride = cols;
    let maximum = rows.max(cols);

    let mut b: Vec<f64> = matrix.rows().flatten().copied().collect();
    let mut u = identity_flat(rows, rows);
    let mut v = identity_flat(cols, cols);
    let mut p = vec![0f64; maximum];
    let mut w = vec![0f64; maximum];

    full_svd_decomposition(
        &mut b,
        &mut u,
        &mut v,
        &mut p,
        &mut w,
        rows,
        cols,
        card,
        stride,
        DEFAULT_MAX_ITERS,
        DEFAULT_TOLERANCE,
        DEFAULT_ABSOLUTE,
    );

    let umat = Matrix2D::from_flat(u, 0.0, rows, rows).map_err(SolverError::InvalidVector)?;
    let sigma = Matrix2D::from_flat(b, 0.0, rows, cols).map_err(SolverError::InvalidVector)?;
    let vmat = Matrix2D::from_flat(v, 0.0, cols, cols).map_err(SolverError::InvalidVector)?;

    Ok((umat, sigma, vmat))
}
pub fn full_svd_decomposition(
    b: &mut [f64],
    u: &mut [f64],
    v: &mut [f64],
    p: &mut [f64],
    w: &mut [f64],
    rows: usize,
    cols: usize,
    card: usize,
    stride: usize,
    max_iters: usize,
    tolerance: f64,
    absolute: f64,
) {
    if cols > rows {
        full_lbidiagonal(b, u, v, p, w, rows, cols, card, stride);
        if rows > 1 {
            full_decomp_lgivens(
                b, u, v, rows, cols, card, stride, max_iters, tolerance, absolute,
            );
        }
    } else {
        full_ubidiagonal(b, u, v, p, w, rows, cols, card, stride);
        if cols > 1 {
            full_decomp_ugivens(
                b, u, v, rows, cols, card, stride, max_iters, tolerance, absolute,
            );
        }
    }
}
pub fn svd_decomposition(
    b: &mut [f64],
    p: &mut [f64],
    w: &mut [f64],
    rows: usize,
    cols: usize,
    card: usize,
    stride: usize,
    max_iters: usize,
    tolerance: f64,
    absolute: f64,
) {
    if cols > rows {
        lbidiagonal(b, p, w, rows, cols, card, stride);
        if rows > 1 {
            decomp_lgivens(b, card, stride, max_iters, tolerance, absolute);
        }
    } else {
        ubidiagonal(b, p, w, rows, cols, card, stride);
        if cols > 1 {
            decomp_ugivens(b, card, stride, max_iters, tolerance, absolute);
        }
    }
}

#[cfg(test)]
mod test_auto_svd {
    use super::*;
    use jedvek::Matrix2D;
    use rand::SeedableRng;
    use rand::prelude::*;
    use rand::rngs::StdRng;
    use rand_distr::StandardNormal;

    const TOLERANCE: f64 = 1e-2;

    fn approx_vector_eq(a: &[f64], b: &[f64]) -> bool {
        if a.len() != b.len() {
            return false;
        }
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

    fn flat(m: &Matrix2D<f64>) -> Vec<f64> {
        m.rows().flatten().copied().collect()
    }

    fn generate_random_matrix(rows: usize, cols: usize) -> Matrix2D<f64> {
        let mut rng = StdRng::seed_from_u64(42);
        let mut data = vec![0f64; rows * cols];
        for d in data.iter_mut() {
            *d = rng.sample(StandardNormal);
        }
        Matrix2D::from_flat(data, 0.0, rows, cols).unwrap()
    }

    fn check_auto_svd_reconstruct(rows: usize, cols: usize) -> (bool, bool, bool) {
        let original = generate_random_matrix(rows, cols);

        let (u, sigma, v) = auto_svd(&original).expect("auto_svd should not error on any shape");

        let identity_rows = Matrix2D::identity(rows);
        let identity_cols = Matrix2D::identity(cols);

        // U U' ~= I and U' U ~= I
        let uut = u.dot(&u.transpose()).unwrap();
        let utu = u.transpose().dot(&u).unwrap();
        let u_ortho_ok = approx_vector_eq(&flat(&uut), &flat(&identity_rows))
            && approx_vector_eq(&flat(&utu), &flat(&identity_rows));

        // V V' ~= I and V' V ~= I
        let vvt = v.dot(&v.transpose()).unwrap();
        let vtv = v.transpose().dot(&v).unwrap();
        let v_ortho_ok = approx_vector_eq(&flat(&vvt), &flat(&identity_cols))
            && approx_vector_eq(&flat(&vtv), &flat(&identity_cols));

        // U Sigma V' ~= original
        let reconstruct = u.dot(&sigma).unwrap().dot(&v.transpose()).unwrap();
        let recon_ok = approx_vector_eq(&flat(&reconstruct), &flat(&original));

        (u_ortho_ok, v_ortho_ok, recon_ok)
    }

    #[test]
    fn test_auto_svd_square() {
        for dim in [2, 4, 7] {
            let (u_ok, v_ok, r_ok) = check_auto_svd_reconstruct(dim, dim);
            assert!(u_ok, "dim={dim}: U not orthogonal");
            assert!(v_ok, "dim={dim}: V not orthogonal");
            assert!(r_ok, "dim={dim}: reconstruction mismatch");
        }
    }

    #[test]
    fn test_auto_svd_wide() {
        for (rows, cols) in [(1, 2), (2, 4), (4, 6), (4, 8)] {
            let (u_ok, v_ok, r_ok) = check_auto_svd_reconstruct(rows, cols);
            assert!(u_ok, "{rows}x{cols}: U not orthogonal");
            assert!(v_ok, "{rows}x{cols}: V not orthogonal");
            assert!(r_ok, "{rows}x{cols}: reconstruction mismatch");
        }
    }

    #[test]
    fn test_auto_svd_tall() {
        for (rows, cols) in [(2, 1), (4, 2), (6, 4), (8, 4)] {
            let (u_ok, v_ok, r_ok) = check_auto_svd_reconstruct(rows, cols);
            assert!(u_ok, "{rows}x{cols}: U not orthogonal");
            assert!(v_ok, "{rows}x{cols}: V not orthogonal");
            assert!(r_ok, "{rows}x{cols}: reconstruction mismatch");
        }
    }

    #[test]
    fn test_auto_svd_dimensions() {
        // shapes come back with the documented dimensions, not just
        // numerically plausible ones
        let (rows, cols) = (5, 3);
        let original = generate_random_matrix(rows, cols);
        let (u, sigma, v) = auto_svd(&original).unwrap();
        assert_eq!((u.height, u.width), (rows, rows));
        assert_eq!((sigma.height, sigma.width), (rows, cols));
        assert_eq!((v.height, v.width), (cols, cols));
    }
}
#[cfg(test)]
mod test_svd_diagonal_parity {
    use super::*;

    const MAX_ITERS: usize = 40;
    const TOLERANCE: f64 = 1e-10;
    const ABSOLUTE: f64 = 1e-4;

    use crate::reduction::matrix::svd::interface::full_svd_decomposition;
    use rand::SeedableRng;
    use rand::prelude::*;
    use rand::rngs::StdRng;
    use rand_distr::StandardNormal;
    pub fn generate_random_vector(n: usize) -> Vec<f64> {
        let mut rng = StdRng::seed_from_u64(42);
        let mut data = vec![0f64; n];
        for d in data.iter_mut() {
            *d = rng.sample(StandardNormal);
        }
        data
    }
    fn create_identity_vector(rows: usize, cols: usize) -> Vec<f64> {
        let mut data = vec![0f64; rows * cols];
        let mut offset = 0;
        for _ in 0..rows.min(cols) {
            data[offset] = 1f64;
            offset += 1 + cols;
        }
        data
    }

    fn diagonal(b: &[f64], card: usize, stride: usize) -> Vec<f64> {
        (0..card).map(|i| b[i * stride + i]).collect()
    }

    fn check_diagonal_parity(rows: usize, cols: usize) -> bool {
        let card = rows.min(cols);
        let stride = cols;
        let maximum = rows.max(cols);

        let original = generate_random_vector(rows * cols);

        // full_svd_decomposition path
        let mut b_full = original.clone();
        let mut u = create_identity_vector(rows, rows);
        let mut v = create_identity_vector(cols, cols);
        let mut w_full = vec![0f64; maximum];
        let mut p_full = vec![0f64; maximum];
        full_svd_decomposition(
            &mut b_full,
            &mut u,
            &mut v,
            &mut p_full,
            &mut w_full,
            rows,
            cols,
            card,
            stride,
            MAX_ITERS,
            TOLERANCE,
            ABSOLUTE,
        );

        // svd_decomposition path (no u/v accumulation)
        let mut b_bare = original.clone();
        let mut w_bare = vec![0f64; maximum];
        let mut p_bare = vec![0f64; maximum];
        svd_decomposition(
            &mut b_bare,
            &mut p_bare,
            &mut w_bare,
            rows,
            cols,
            card,
            stride,
            MAX_ITERS,
            TOLERANCE,
            ABSOLUTE,
        );

        let diag_full = diagonal(&b_full, card, stride);
        let diag_bare = diagonal(&b_bare, card, stride);

        diag_full == diag_bare
    }

    #[test]
    fn test_diagonal_parity_square() {
        for dim in [2, 4, 7] {
            assert!(
                check_diagonal_parity(dim, dim),
                "dim={dim}: diagonals diverged"
            );
        }
    }

    #[test]
    fn test_diagonal_parity_wide() {
        for (rows, cols) in [(2, 4), (4, 6), (4, 8)] {
            assert!(
                check_diagonal_parity(rows, cols),
                "{rows}x{cols}: diagonals diverged"
            );
        }
    }

    #[test]
    fn test_diagonal_parity_tall() {
        for (rows, cols) in [(4, 2), (6, 4), (8, 4)] {
            assert!(
                check_diagonal_parity(rows, cols),
                "{rows}x{cols}: diagonals diverged"
            );
        }
    }

    #[rustfmt::skip]
    #[test]
    fn test_diagonal_parity_trials() {
        let trials = 10_000;
        let mut failures = 0;
        for _ in 0..trials {
            if !check_diagonal_parity(6, 6) { failures += 1; }
        }
        println!("diagonal parity: {failures} failures / {trials}");
        assert!(failures == 0, "exact-path diagonals diverged {failures} times — codepaths are not identical");
    }
}
#[cfg(test)]
mod test_svd_convergence_rate {
    use super::*;
    const MAX_ITERS: usize = 40;
    const TOLERANCE: f64 = 1e-10;
    const ABSOLUTE: f64 = 1e-4;
    const CONVERGE_THRESHOLD: f64 = 1e-6;
    use rand::SeedableRng;
    use rand::prelude::*;
    use rand::rngs::StdRng;
    use rand_distr::StandardNormal;

    pub fn generate_random_vector(n: usize) -> Vec<f64> {
        let mut rng = StdRng::seed_from_u64(42);
        let mut data = vec![0f64; n];
        for d in data.iter_mut() {
            *d = rng.sample(StandardNormal);
        }
        data
    }

    // off-diagonal energy just above the main diagonal (upper bidiagonal band)
    fn sum_upper_bidiagonal(m: &[f64], rows: usize, stride: usize) -> f64 {
        let mut error = 0f64;
        let mut offset = 1;
        for _ in 0..rows.saturating_sub(1) {
            error += m[offset].abs();
            offset += stride + 1;
        }
        error
    }

    // off-diagonal energy just below the main diagonal (lower bidiagonal band)
    fn sum_lower_bidiagonal(m: &[f64], rows: usize, stride: usize) -> f64 {
        let mut error = 0f64;
        let mut offset = stride;
        for _ in 0..rows.saturating_sub(1) {
            error += m[offset].abs();
            offset += stride + 1;
        }
        error
    }

    // total residual off-diagonal mass, whichever band is relevant post-convergence
    fn off_diagonal_residual(m: &[f64], rows: usize, stride: usize) -> f64 {
        sum_upper_bidiagonal(m, rows, stride) + sum_lower_bidiagonal(m, rows, stride)
    }

    fn run_convergence_trial(
        rows: usize,
        cols: usize,
        max_iters: usize,
        tol: f64,
        absolute: f64,
    ) -> f64 {
        let card = rows.min(cols);
        let stride = cols;
        let maximum = rows.max(cols);

        let mut b = generate_random_vector(rows * cols);
        let mut w = vec![0f64; maximum];
        let mut p = vec![0f64; maximum];

        svd_decomposition(
            &mut b, &mut p, &mut w, rows, cols, card, stride, max_iters, tol, absolute,
        );

        off_diagonal_residual(&b, card, stride)
    }
    fn convergence_rate_report(
        rows: usize,
        cols: usize,
        trials: usize,
        max_iters: usize,
        tol: f64,
        converge: f64,
        absolute: f64,
    ) -> Result<(), String> {
        let mut failures = 0usize;
        let mut max_residual = 0f64;
        let mut sum_residual = 0f64;
        let card = rows.min(cols);
        let convergence_threshold = (card as f64) * converge;
        let iterations = card * max_iters;

        for _ in 0..trials {
            let residual = run_convergence_trial(rows, cols, iterations, tol, absolute);
            sum_residual += residual;
            if residual > max_residual {
                max_residual = residual;
            }
            if residual > convergence_threshold {
                failures += 1;
            }
        }

        let mean_residual = sum_residual / trials as f64;
        let rate = 100.0 * (trials - failures) as f64 / trials as f64;

        println!(
            "{rows}x{cols}: converged {rate:.3}% ({failures} failures / {trials}), \
             mean residual = {mean_residual:.3e}, max residual = {max_residual:.3e}"
        );

        if failures > 0 {
            Err(format!(
                "{rows}x{cols}: {failures}/{trials} trials failed to converge below {convergence_threshold:e}"
            ))
        } else {
            Ok(())
        }
    }
    #[test]
    fn test_convergence_rate_square_6x6() {
        convergence_rate_report(
            6,
            6,
            10_000,
            MAX_ITERS,
            TOLERANCE,
            CONVERGE_THRESHOLD,
            ABSOLUTE,
        )
        .unwrap();
    }

    #[test]
    fn test_convergence_rate_square_various() {
        let mut errors = Vec::new();
        for dim in [2, 3, 4, 5, 8] {
            if let Err(e) = convergence_rate_report(
                dim,
                dim,
                2_000,
                MAX_ITERS,
                TOLERANCE,
                CONVERGE_THRESHOLD,
                ABSOLUTE,
            ) {
                errors.push(e);
            }
        }
        assert!(errors.is_empty(), "\n{}", errors.join("\n"));
    }

    #[test]
    fn test_convergence_rate_tall() {
        let mut errors = Vec::new();
        for (rows, cols) in [(4, 2), (6, 4), (8, 4), (10, 6)] {
            if let Err(e) = convergence_rate_report(
                rows,
                cols,
                2_000,
                MAX_ITERS,
                TOLERANCE,
                CONVERGE_THRESHOLD,
                ABSOLUTE,
            ) {
                errors.push(e);
            }
        }
        assert!(errors.is_empty(), "\n{}", errors.join("\n"));
    }

    #[test]
    fn test_convergence_rate_wide() {
        let mut errors = Vec::new();
        for (rows, cols) in [(2, 4), (4, 6), (4, 8), (6, 10)] {
            if let Err(e) = convergence_rate_report(
                rows,
                cols,
                2_000,
                MAX_ITERS,
                TOLERANCE,
                CONVERGE_THRESHOLD,
                ABSOLUTE,
            ) {
                errors.push(e);
            }
        }
        assert!(errors.is_empty(), "\n{}", errors.join("\n"));
    }
}
