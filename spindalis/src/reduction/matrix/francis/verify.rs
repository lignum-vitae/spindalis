#![allow(unused)]

// NOTE: Contained functions are used for testing purposes to ensure that the
// implementaitons of symmetric eigen decomp and complex eigen decomp maintain
// the same methods

use crate::reduction::matrix::francis::constants::{ABSOLUTE_CAP, MAX_ITERS, TOLERANCE};
use crate::reduction::matrix::francis::constants::{
    EXCEPTION_SHIFT_OFFSET, EXCEPTION_SHIFT_PERIOD,
};
use crate::reduction::matrix::francis::givens::{
    apply_g_left, apply_gt_right, implicit_givens_rotation,
};
#[rustfmt::skip]
use crate::reduction::matrix::francis::primitives::{
    params,
    deflate,
    eigen,
    double_shift,
    exception_shift,
    complex_eig_pair,
    lapply_householder,
    rapply_householder,
};
#[rustfmt::skip]
fn decomp_sym_with_rotation(
    hess_lin_matrix: &mut [f64],
    rotation: &mut [f64],
    mut range: usize,
    size: usize,
    stride: usize
) -> bool {
    let s = range * stride;
    let mut error1 = s.saturating_sub(stride + 1);
    let mut error2 = s.saturating_sub(stride + stride + 2);
    let mut top_left = s.saturating_sub(stride + 2);
    let mut bottom_left = s.saturating_sub(2);
    let mut curriter = 0;
    while range > 1 && curriter < MAX_ITERS {
        let scale = hess_lin_matrix[top_left].abs() + hess_lin_matrix[bottom_left+1].abs();
        curriter += 1;
        if hess_lin_matrix[error1].abs() < (scale * TOLERANCE).min(ABSOLUTE_CAP) {
            deflate(
                1,
                stride,
                &mut range,
                &mut error1,
                &mut error2,
                &mut top_left,
                &mut bottom_left,
                &mut curriter,
            );
        } else if hess_lin_matrix[error2].abs() < TOLERANCE && curriter == MAX_ITERS {
            deflate(
                2,
                stride,
                &mut range,
                &mut error1,
                &mut error2,
                &mut top_left,
                &mut bottom_left,
                &mut curriter,
            );
        } else {
            francis_iteration_sym_with_rotation(hess_lin_matrix, rotation, size, range, stride, top_left, bottom_left);
        }
    }
    range <= 1
}
fn decomp_cpx_with_rotation(
    hess_lin_matrix: &mut [f64],
    projection: &mut [f64],
    rotation: &mut [f64],
    workspace: &mut [f64],
    mut range: usize,
    size: usize,
    stride: usize,
) -> bool {
    let s = range * stride;
    let mut error1 = s.saturating_sub(stride + 1);
    let mut error2 = s.saturating_sub(stride + stride + 2);
    let mut top_left = s.saturating_sub(stride + 2);
    let mut bottom_left = s.saturating_sub(2);
    let mut curriter = 0;
    let mut stall = 0;
    while range > 0 && curriter < MAX_ITERS {
        curriter += 1;
        if hess_lin_matrix[error1].abs() < TOLERANCE {
            stall = 0;
            deflate(
                1,
                stride,
                &mut range,
                &mut error1,
                &mut error2,
                &mut top_left,
                &mut bottom_left,
                &mut curriter,
            );
        } else if hess_lin_matrix[error2].abs() < TOLERANCE {
            // if error2 == 0 then we are hitting eigen which should be greater than tolerance
            deflate(
                2,
                stride,
                &mut range,
                &mut error1,
                &mut error2,
                &mut top_left,
                &mut bottom_left,
                &mut curriter,
            );
            stall = 0;
        } else if range == 2 && complex_eig_pair(hess_lin_matrix, top_left, bottom_left) {
            deflate(
                2,
                stride,
                &mut range,
                &mut error1,
                &mut error2,
                &mut top_left,
                &mut bottom_left,
                &mut curriter,
            );
            stall = 0;
        } else {
            if range == 2 || size <= 2 {
                francis_iteration_cpx_2x2_with_rotation(
                    hess_lin_matrix,
                    rotation,
                    size,
                    stride,
                    top_left,
                    bottom_left,
                );
            } else if (stall + EXCEPTION_SHIFT_OFFSET).is_multiple_of(EXCEPTION_SHIFT_PERIOD) {
                exception_shift(hess_lin_matrix, workspace, stride, top_left);
                francis_iteration_cpx_with_rotation(
                    hess_lin_matrix,
                    projection,
                    rotation,
                    workspace,
                    size,
                    range,
                    stride,
                );
            } else {
                double_shift(hess_lin_matrix, workspace, stride, top_left, bottom_left);
                francis_iteration_cpx_with_rotation(
                    hess_lin_matrix,
                    projection,
                    rotation,
                    workspace,
                    size,
                    range,
                    stride,
                );
            }
            stall += 1;
        }
    }
    range <= 1
}
fn francis_iteration_cpx_with_rotation(
    hess_lin_matrix: &mut [f64],
    projection: &mut [f64],
    rotation: &mut [f64],
    workspace: &mut [f64],
    size: usize,
    range: usize,
    stride: usize,
) {
    let bound = range.min(3);
    let projection = &mut projection[..bound];
    let tau = params(&mut workspace[..bound], projection);
    if tau != 0f64 {
        rapply_householder(
            hess_lin_matrix,
            projection,
            workspace,
            tau,
            size,
            bound,
            stride,
        );
        lapply_householder(
            hess_lin_matrix,
            projection,
            workspace,
            tau,
            bound,
            range,
            stride,
        );
        // ----------------- tracking the rotation matrix
        lapply_householder(rotation, projection, workspace, tau, bound, size, stride);
    }
    let mut offset = 0;
    for o in 1..range.saturating_sub(1) {
        let bound = bound.min(stride - o);
        let (slice, target) = hess_lin_matrix.split_at_mut(offset + stride);
        let slice = &mut slice[offset + o..offset + o + bound];
        let proj = &mut projection[..bound];
        let tau = params(slice, proj);
        offset += stride;
        if tau == 0f64 {
            continue;
        }
        rapply_householder(
            &mut target[o..],
            proj,
            workspace,
            tau,
            size - o,
            bound,
            stride,
        );
        lapply_householder(
            &mut hess_lin_matrix[offset..],
            proj,
            workspace,
            tau,
            bound,
            range,
            stride,
        );
        // ----------------- tracking the rotation matrix
        lapply_householder(
            &mut rotation[offset..],
            proj,
            workspace,
            tau,
            bound,
            size,
            stride,
        );
    }
}
fn francis_iteration_cpx_2x2_with_rotation(
    hess_lin_matrix: &mut [f64],
    rotation: &mut [f64],
    size: usize,
    stride: usize,
    top_left: usize,
    bottom_left: usize,
) {
    let eig = eigen(
        hess_lin_matrix[top_left],
        hess_lin_matrix[top_left + 1],
        hess_lin_matrix[bottom_left],
        hess_lin_matrix[bottom_left + 1],
    );
    let (_, cosine, sine) = implicit_givens_rotation(hess_lin_matrix[0] - eig, hess_lin_matrix[1]);
    apply_gt_right(hess_lin_matrix, 0, 1, stride, size, cosine, sine);
    apply_g_left(hess_lin_matrix, 0, 1, stride, 2, cosine, sine);
    apply_g_left(rotation, 0, 1, stride, size, cosine, sine);
}
fn francis_iteration_sym_with_rotation(
    hess_lin_matrix: &mut [f64],
    rotation: &mut [f64],
    size: usize,
    range: usize,
    stride: usize,
    top_left: usize,
    bottom_left: usize,
) {
    let eig = eigen(
        hess_lin_matrix[top_left],
        hess_lin_matrix[top_left + 1],
        hess_lin_matrix[bottom_left],
        hess_lin_matrix[bottom_left + 1],
    );
    let (_, cosine, sine) = implicit_givens_rotation(hess_lin_matrix[0] - eig, hess_lin_matrix[1]);
    apply_gt_right(hess_lin_matrix, 0, 1, stride, size, cosine, sine);
    apply_g_left(hess_lin_matrix, 0, 1, stride, size, cosine, sine);
    apply_g_left(rotation, 0, 1, stride, size, cosine, sine);
    for o in 0..range.saturating_sub(2) {
        let row = o * stride;
        let s1 = o + 1;
        let s2 = o + 2;
        let (_, cosine, sine) =
            implicit_givens_rotation(hess_lin_matrix[row + s1], hess_lin_matrix[row + s2]);
        apply_gt_right(
            &mut hess_lin_matrix[row..],
            s1,
            s2,
            stride,
            range - o,
            cosine,
            sine,
        );
        apply_g_left(hess_lin_matrix, s1, s2, stride, range, cosine, sine);
        apply_g_left(rotation, s1, s2, stride, size, cosine, sine);
    }
}
fn hessenberg_lq_with_rotation(
    hess_lin_matrix: &mut [f64],
    projection: &mut [f64],
    rotation: &mut [f64],
    workspace: &mut [f64],
    rows: usize,
    cols: usize,
    stride: usize,
) {
    // stores tau
    let mut offset = 0;
    let mut active_range = rows;
    let mut split_range = cols;
    for o in 1..rows {
        active_range -= 1;
        split_range -= 1;
        let (slice, target) = hess_lin_matrix.split_at_mut(offset + stride);
        let slice = &mut slice[offset + o..offset + cols];
        let proj = &mut projection[..split_range];
        let tau = params(slice, proj);
        offset += stride;
        if tau == 0f64 {
            continue;
        }
        rapply_householder(
            &mut target[o..],
            proj,
            workspace,
            tau,
            rows - o,
            split_range,
            stride,
        );
        lapply_householder(
            &mut hess_lin_matrix[offset..],
            proj,
            workspace,
            tau,
            active_range,
            cols,
            stride,
        );
        lapply_householder(
            &mut rotation[offset..],
            proj,
            workspace,
            tau,
            active_range,
            cols,
            stride,
        );
    }
}
#[cfg(test)]
mod test_hessenberg_reconstructions {
    use super::*;
    use jedvek::Matrix2D;
    use rand::SeedableRng;
    use rand::prelude::*;
    use rand_distr::StandardNormal;
    //  NOTE: This should also be weighted towards the size of the dimensionality
    //  of the decomposition ie the condition number not a flat tolerance level
    const TOLERANCE: f64 = 1e-2;
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
    fn to_matrix(data: &[f64], rows: usize, cols: usize) -> Matrix2D<f64> {
        Matrix2D::from_flat(data, 0.0, rows, cols).unwrap()
    }
    fn flat(m: &Matrix2D<f64>) -> Vec<f64> {
        m.rows().flatten().copied().collect()
    }
    fn generate_random_vector(n: usize) -> Vec<f64> {
        let mut rng = StdRng::seed_from_u64(42);
        let mut data = vec![0f64; n];
        for d in data.iter_mut().take(n) {
            *d = rng.sample(StandardNormal);
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
    // fn generate_strict_symmetric_vector(n: usize) -> Vec<f64> {
    //     let mut data = generate_random_vector(n * n);
    //     for i in 0..n {
    //         for j in 0..i {
    //             let val = data[i * n + j];
    //             data[j * n + i] = val;
    //         }
    //     }
    //     data
    // }
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
    #[test]
    fn test_hessenberg_reconstruct_general() {
        for dim in [1, 2, 4, 7] {
            let (rows, cols) = (dim, dim);
            let stride = dim;
            let mut h = generate_random_vector(rows * cols);
            let mut r = generate_identity_vector(rows, cols);
            let mut p = vec![0f64; cols];
            let mut w = vec![0f64; rows];
            let original = to_matrix(&h, rows, cols);

            hessenberg_lq_with_rotation(&mut h, &mut p, &mut r, &mut w, rows, cols, stride);

            let kernel = to_matrix(&h, rows, cols);
            let rotation = to_matrix(&r, rows, cols);
            let identity = Matrix2D::identity(dim);

            // R R' ~= I
            let rrt = rotation.dot(&rotation.transpose()).unwrap();
            assert!(
                approx_vector_eq(&flat(&rrt), &flat(&identity)),
                "dim={dim}: R R' not orthogonal, got {rrt}"
            );
            // R' R ~= I
            let rtr = rotation.transpose().dot(&rotation).unwrap();
            assert!(
                approx_vector_eq(&flat(&rtr), &flat(&identity)),
                "dim={dim}: R' R not orthogonal, got {rtr}"
            );
            // R' H R ~= original
            let reconstruct = rotation
                .transpose()
                .dot(&kernel)
                .unwrap()
                .dot(&rotation)
                .unwrap();
            assert!(
                approx_vector_eq(&flat(&reconstruct), &flat(&original)),
                "dim={dim}: reconstruction mismatch, got {reconstruct} expected {original}"
            );
        }
    }
    #[test]
    fn test_hessenberg_reconstruct_symmetric() {
        for dim in [1, 2, 4, 7] {
            let (rows, cols) = (dim, dim);
            let stride = dim;
            let mut h = generate_approx_symmetric_vector(dim);
            let mut r = generate_identity_vector(rows, cols);
            let mut p = vec![0f64; cols];
            let mut w = vec![0f64; rows];
            let original = to_matrix(&h, rows, cols);

            hessenberg_lq_with_rotation(&mut h, &mut p, &mut r, &mut w, rows, cols, stride);

            let kernel = to_matrix(&h, rows, cols);
            let rotation = to_matrix(&r, rows, cols);
            let identity = Matrix2D::identity(dim);

            let rrt = rotation.dot(&rotation.transpose()).unwrap();
            assert!(
                approx_vector_eq(&flat(&rrt), &flat(&identity)),
                "dim={dim}: R R' not orthogonal"
            );

            // symmetric-specific: hessenberg of a symmetric matrix should be
            // tridiagonal, i.e. zero below the first subdiagonal
            for i in 0..rows {
                for j in 0..cols {
                    if i > j + 1 {
                        assert!(
                            h[i * stride + j].abs() < 1e-2,
                            "dim={dim}: expected tridiagonal, got h[{i}][{j}]={}",
                            h[i * stride + j]
                        );
                    }
                }
            }

            // R' H R ~= original
            let reconstruct = rotation
                .transpose()
                .dot(&kernel)
                .unwrap()
                .dot(&rotation)
                .unwrap();
            assert!(
                approx_vector_eq(&flat(&reconstruct), &flat(&original)),
                "dim={dim}: symmetric reconstruction mismatch"
            );
        }
    }
    fn check_decomp_sym_reconstruct() -> (bool, bool) {
        let c = 6;
        let (rows, cols) = (c, c);
        let stride = c;

        let mut h = generate_approx_symmetric_vector(rows);
        let mut r = generate_identity_vector(rows, cols);
        let mut p = vec![0f64; cols];
        let mut w = vec![0f64; rows];

        let original = to_matrix(&h, rows, cols);

        hessenberg_lq_with_rotation(&mut h, &mut p, &mut r, &mut w, rows, cols, stride);
        let converged = decomp_sym_with_rotation(&mut h, &mut r, c, c, c);

        let kernel = to_matrix(&h, rows, cols);
        let rotation = to_matrix(&r, rows, cols);
        let identity = Matrix2D::identity(c);

        let rrt = rotation.dot(&rotation.transpose()).unwrap();
        let rtr = rotation.transpose().dot(&rotation).unwrap();
        let reconstruct = rotation
            .transpose()
            .dot(&kernel)
            .unwrap()
            .dot(&rotation)
            .unwrap();

        let ortho_ok = approx_vector_eq(&flat(&rrt), &flat(&identity))
            && approx_vector_eq(&flat(&rtr), &flat(&identity));
        let recon_ok = approx_vector_eq(&flat(&reconstruct), &flat(&original));

        (converged, ortho_ok && recon_ok)
    }
    #[rustfmt::skip]
    #[test]
    fn test_symmetric_reconstruct() {
        let trials = 10_000;
        let mut convergence_failures = 0;
        let mut reconstruction_failures = 0;

        for _ in 0..trials {
            let (converged, reconstructed) = check_decomp_sym_reconstruct();
            if !converged { convergence_failures += 1; }
            if !reconstructed { reconstruction_failures += 1; }
        }

        println!("sym: {convergence_failures} convergence failures, {reconstruction_failures} reconstruction failures / {trials}");
        assert!(convergence_failures < 10, "too many convergence failures: {convergence_failures}");
        assert!(reconstruction_failures < 10, "too many reconstruction failures: {reconstruction_failures}");
    }
    fn check_decomp_cpx_reconstruct() -> (bool, bool) {
        let c = 6;
        let (rows, cols) = (c, c);
        let stride = c;

        let mut h = generate_random_vector(rows * cols);
        let mut r = generate_identity_vector(rows, cols);
        let mut p = vec![0f64; cols];
        let mut w = vec![0f64; rows];

        let original = to_matrix(&h, rows, cols);

        hessenberg_lq_with_rotation(&mut h, &mut p, &mut r, &mut w, rows, cols, stride);
        let converged = decomp_cpx_with_rotation(&mut h, &mut p, &mut r, &mut w, c, c, c);

        let kernel = to_matrix(&h, rows, cols);
        let rotation = to_matrix(&r, rows, cols);
        let identity = Matrix2D::identity(c);

        let rrt = rotation.dot(&rotation.transpose()).unwrap();
        let rtr = rotation.transpose().dot(&rotation).unwrap();
        let reconstruct = rotation
            .transpose()
            .dot(&kernel)
            .unwrap()
            .dot(&rotation)
            .unwrap();

        let ortho_ok = approx_vector_eq(&flat(&rrt), &flat(&identity))
            && approx_vector_eq(&flat(&rtr), &flat(&identity));
        let recon_ok = approx_vector_eq(&flat(&reconstruct), &flat(&original));

        (converged, ortho_ok && recon_ok)
    }
    #[rustfmt::skip]
    #[test]
    fn test_complex_reconstruct() {
        let trials = 10_000;
        let mut convergence_failures = 0;
        let mut reconstruction_failures = 0;

        for _ in 0..trials {
            let (converged, reconstructed) = check_decomp_cpx_reconstruct();
            if !converged { convergence_failures += 1; }
            if !reconstructed { reconstruction_failures += 1; }
        }
        println!("cpx: {convergence_failures} convergence failures, {reconstruction_failures} reconstruction failures / {trials}");
        assert!(convergence_failures < 10, "too many convergence failures: {convergence_failures}");
        assert!(reconstruction_failures < 10, "too many reconstruction failures: {reconstruction_failures}");
    }
}

#[cfg(test)]
mod test_verify_correspondance_complex {
    use super::*;
    use crate::reduction::matrix::francis::complex::decomp_cpx;
    use crate::reduction::matrix::francis::constants::{MAX_ITERS, TOLERANCE};
    use crate::reduction::matrix::francis::primitives::hessenberg_lq;
    use crate::reduction::matrix::francis::verify::{
        decomp_cpx_with_rotation, hessenberg_lq_with_rotation,
    };
    use rand::prelude::*;
    use rand_distr::StandardNormal;

    fn generate_random_vector(n: usize) -> Vec<f64> {
        let mut rng = StdRng::seed_from_u64(42);
        let mut data = vec![0f64; n];
        for d in data.iter_mut().take(n) {
            *d = rng.sample(StandardNormal);
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
    // A * A^T so it's complex (with float noise, like the other tests)
    fn generate_approx_complex_vector(n: usize) -> Vec<f64> {
        let a = generate_random_vector(n * n);
        let mut result = vec![0f64; n * n];
        for i in 0..n {
            for j in 0..n {
                let mut sum = 0f64;
                for k in 0..n {
                    sum += a[i * n + k] * a[j * n + k];
                }
                result[i * n + j] = sum;
            }
        }
        result
    }
    fn approx_slice_eq(a: &[f64], b: &[f64], tolerance: f64) -> bool {
        assert_eq!(a.len(), b.len());
        a.iter().zip(b.iter()).all(|(x, y)| {
            if x.is_nan() || y.is_nan() {
                return false;
            }
            (x - y).abs() < tolerance
        })
    }
    #[test]
    fn test_decomp_cpx_matches_decomp_cpx_with_rotation() {
        // 5 random shapes
        for dim in [2, 3, 5, 6, 8] {
            let (rows, cols) = (dim, dim);
            let stride = dim;

            let base = generate_approx_complex_vector(dim);

            // --- plain decomp_cpx path ---
            let mut h_plain = base.clone();
            let mut p_plain = vec![0f64; cols];
            let mut w_plain = vec![0f64; rows.max(3)];
            hessenberg_lq(&mut h_plain, &mut p_plain, &mut w_plain, rows, cols, stride);
            decomp_cpx(
                &mut h_plain,
                &mut p_plain,
                &mut w_plain,
                dim,
                dim,
                stride,
                MAX_ITERS,
                TOLERANCE,
            );

            // --- rotation-tracking decomp_cpx_with_rotation path ---
            let mut h_rot = base.clone();
            let mut r_rot = generate_identity_vector(rows, cols);
            let mut p_rot = vec![0f64; cols];
            let mut w_rot = vec![0f64; rows.max(3)];
            hessenberg_lq_with_rotation(
                &mut h_rot, &mut p_rot, &mut r_rot, &mut w_rot, rows, cols, stride,
            );
            decomp_cpx_with_rotation(
                &mut h_rot, &mut p_rot, &mut r_rot, &mut w_rot, rows, cols, stride,
            );

            assert!(
                approx_slice_eq(&h_plain, &h_rot, 1e-6),
                "dim={dim}: decomp_cpx and decomp_cpx_with_rotation diverged\nplain: {h_plain:?}\nrot:   {h_rot:?}"
            );
        }
    }
}
#[cfg(test)]
mod test_verify_correspondance_symmetric {
    use super::*;
    use crate::reduction::matrix::francis::constants::{ABSOLUTE_CAP, MAX_ITERS, TOLERANCE};
    use crate::reduction::matrix::francis::primitives::hessenberg_lq;
    use crate::reduction::matrix::francis::symmetric::decomp_sym;
    use crate::reduction::matrix::francis::verify::{
        decomp_sym_with_rotation, hessenberg_lq_with_rotation,
    };
    use rand::SeedableRng;
    use rand::prelude::*;
    use rand::rngs::StdRng;
    use rand_distr::StandardNormal;

    fn generate_random_vector(n: usize) -> Vec<f64> {
        let mut rng = StdRng::seed_from_u64(42);
        let mut data = vec![0f64; n];
        for d in data.iter_mut().take(n) {
            *d = rng.sample(StandardNormal);
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

    // A * A^T so it's symmetric (with float noise, like the other tests)
    fn generate_approx_symmetric_vector(n: usize) -> Vec<f64> {
        let a = generate_random_vector(n * n);
        let mut result = vec![0f64; n * n];
        for i in 0..n {
            for j in 0..n {
                let mut sum = 0f64;
                for k in 0..n {
                    sum += a[i * n + k] * a[j * n + k];
                }
                result[i * n + j] = sum;
            }
        }
        result
    }

    fn approx_slice_eq(a: &[f64], b: &[f64], tolerance: f64) -> bool {
        assert_eq!(a.len(), b.len());
        a.iter().zip(b.iter()).all(|(x, y)| {
            if x.is_nan() || y.is_nan() {
                return false;
            }
            (x - y).abs() < tolerance
        })
    }
    #[test]
    fn test_decomp_sym_matches_decomp_sym_with_rotation() {
        // 5 random shapes
        for dim in [2, 3, 5, 6, 8] {
            let (rows, cols) = (dim, dim);
            let stride = dim;

            let base = generate_approx_symmetric_vector(dim);

            // --- plain decomp_sym path ---
            let mut h_plain = base.clone();
            let mut p_plain = vec![0f64; cols];
            let mut w_plain = vec![0f64; rows];
            hessenberg_lq(&mut h_plain, &mut p_plain, &mut w_plain, rows, cols, stride);
            decomp_sym(
                &mut h_plain,
                dim,
                dim,
                stride,
                MAX_ITERS,
                TOLERANCE,
                ABSOLUTE_CAP,
            );
            // --- rotation-tracking decomp_sym_with_rotation path ---
            let mut h_rot = base.clone();
            let mut r_rot = generate_identity_vector(rows, cols);
            let mut p_rot = vec![0f64; cols];
            let mut w_rot = vec![0f64; rows];
            hessenberg_lq_with_rotation(
                &mut h_rot, &mut p_rot, &mut r_rot, &mut w_rot, rows, cols, stride,
            );
            decomp_sym_with_rotation(&mut h_rot, &mut r_rot, dim, dim, stride);
            assert!(
                approx_slice_eq(&h_plain, &h_rot, 1e-6),
                "dim={dim}: decomp_sym and decomp_sym_with_rotation diverged\nplain: {h_plain:?}\nrot:   {h_rot:?}"
            );
        }
    }
}
