use crate::reduction::matrix::svd::bidiagonalization::{lbidiagonal, ubidiagonal};
use crate::reduction::matrix::svd::bulge_chasing::{decomp_lgivens, decomp_ugivens};
#[rustfmt::skip]
use crate::reduction::matrix::svd::verify::{
    full_ubidiagonal,
    full_lbidiagonal,
    full_decomp_ugivens,
    full_decomp_lgivens
};

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
mod test_svd_diagonal_parity {
    use super::*;

    const MAX_ITERS: usize = 40;
    const TOLERANCE: f64 = 1e-10;
    const ABSOLUTE: f64 = 1e-4;

    use crate::reduction::matrix::svd::interface::full_svd_decomposition;

    // src/random/generation.rs
    use rand::SeedableRng;
    use rand::prelude::*;
    use rand::rngs::StdRng;
    use rand_distr::StandardNormal;
    // src/algebra/ndmethods.rs
    fn matrix_mult(
        a: &[f64],
        a_rows: usize,
        a_cols: usize,
        b: &[f64],
        b_rows: usize,
        b_cols: usize,
        out: &mut [f64],
    ) {
        assert_eq!(
            a_cols, b_rows,
            "matrix_mult: inner dims mismatch ({a_cols} vs {b_rows})"
        );
        assert_eq!(out.len(), a_rows * b_cols);

        for i in 0..a_rows {
            for k in 0..a_cols {
                let a_ik = a[i * a_cols + k];
                for j in 0..b_cols {
                    out[i * b_cols + j] += a_ik * b[k * b_cols + j];
                }
            }
        }
    }
    pub fn transpose(a: &[f64], rows: usize, cols: usize, out: &mut [f64]) {
        assert_eq!(out.len(), rows * cols);
        for i in 0..rows {
            for j in 0..cols {
                out[j * rows + i] = a[i * cols + j];
            }
        }
    }
    pub fn approx_vector_eq(a: &[f64], b: &[f64]) -> bool {
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
        error / (n as f64).sqrt() < 1e-2
    }
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
            sum_residual += residual as f64;
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
