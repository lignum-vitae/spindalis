use crate::reduction::matrix::francis::constants::{ABSOLUTE_CAP, MAX_ITERS, TOLERANCE};
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
pub fn full_decomp_sym(
    h: &mut [f64],
    r: &mut [f64],
    mut range: usize,
    size: usize,
    stride: usize
) -> bool {
    let s = range * stride;
    let mut e1 = s.saturating_sub(stride + 1);
    let mut e2 = s.saturating_sub(stride + stride + 2);
    let mut tl = s.saturating_sub(stride + 2);
    let mut bl = s.saturating_sub(2);
    let mut curriter = 0;
    let mut error = vec![];
    while range > 1 && curriter < MAX_ITERS {
        let scale = h[tl].abs() + h[bl+1].abs();
        curriter += 1;
        if h[e1].abs() < (scale * TOLERANCE).min(ABSOLUTE_CAP) {
            deflate(
                1,
                stride,
                &mut range,
                &mut e1,
                &mut e2,
                &mut tl,
                &mut bl,
                &mut curriter,
            );
            error = vec![];
        } else if h[e2].abs() < TOLERANCE && curriter == MAX_ITERS {
            deflate(
                2,
                stride,
                &mut range,
                &mut e1,
                &mut e2,
                &mut tl,
                &mut bl,
                &mut curriter,
            );
            error = vec![];
        } else {
            full_francis_iteration_sym(h, r, size, range, stride, tl, bl);
        }
        error.push(h[e1]);
    }
    range <= 1
}
pub fn full_decomp_cpx(
    h: &mut [f64],
    r: &mut [f64],
    w: &mut [f64],
    mut range: usize,
    size: usize,
    stride: usize,
) -> bool {
    let s = range * stride;
    let mut e1 = s.saturating_sub(stride + 1);
    let mut e2 = s.saturating_sub(stride + stride + 2);
    let mut tl = s.saturating_sub(stride + 2);
    let mut bl = s.saturating_sub(2);
    let mut curriter = 0;
    let _he1 = h[e1];
    let _he2 = h[e2];
    let p = &mut [0f64; 3];
    let mut stall = 0;
    while range > 0 && curriter < MAX_ITERS {
        curriter += 1;
        if h[e1].abs() < TOLERANCE {
            stall = 0;
            deflate(
                1,
                stride,
                &mut range,
                &mut e1,
                &mut e2,
                &mut tl,
                &mut bl,
                &mut curriter,
            );
        } else if h[e2].abs() < TOLERANCE {
            // if e2 == 0 then we are hitting eigen which should be greater than tolerance
            deflate(
                2,
                stride,
                &mut range,
                &mut e1,
                &mut e2,
                &mut tl,
                &mut bl,
                &mut curriter,
            );
            stall = 0;
        } else if range == 2 && complex_eig_pair(h, tl, bl) {
            deflate(
                2,
                stride,
                &mut range,
                &mut e1,
                &mut e2,
                &mut tl,
                &mut bl,
                &mut curriter,
            );
            stall = 0;
        } else {
            if range == 2 {
                full_francis_iteration_cpx_2x2(h, r, size, stride, tl, bl);
            // } else if stall > 0 && (stall + 4) % 10 == 0 {
            } else if (stall + 8) % 12 == 0 {
                // } else if (stall + 4) % 10 == 0 {
                // } else if stall == 6 {
                exception_shift(h, w, stride, range, tl, bl);
                full_francis_iteration_cpx(h, r, p, w, size, range, stride, tl, bl);
            } else {
                double_shift(h, w, stride, range, tl, bl);
                full_francis_iteration_cpx(h, r, p, w, size, range, stride, tl, bl);
            }
            stall += 1;
        }
    }
    range <= 1
}
/// francis_iteration_cpx
///
/// * h: hessenberg linearized matrix
/// * r: rotaiton linearized matrix
/// * p: projection slice
/// * w: workspace slice
/// * size: static number of rows for rotations
/// * range: number of rows in active window
/// * stride: stride of the data format
pub fn full_francis_iteration_cpx(
    h: &mut [f64],
    r: &mut [f64],
    p: &mut [f64],
    w: &mut [f64],
    size: usize,
    range: usize,
    stride: usize,
    _tl: usize,
    _bl: usize,
) {
    let bound = range.min(3);
    let p = &mut p[..bound];
    let tau = params(&mut w[..bound], p);
    if tau != 0f64 {
        rapply_householder(h, p, w, tau, size, bound, stride);
        lapply_householder(h, p, w, tau, bound, range, stride);
        // ----------------- tracking the rotation matrix
        lapply_householder(r, p, w, tau, bound, size, stride);
    }
    let mut offset = 0;
    for o in 1..range.saturating_sub(1) {
        let bound = bound.min(stride - o);
        let (slice, t) = h.split_at_mut(offset + stride);
        let slice = &mut slice[offset + o..offset + o + bound];
        let proj = &mut p[..bound];
        let tau = params(slice, proj);
        offset += stride;
        if tau == 0f64 {
            continue;
        }
        rapply_householder(&mut t[o..], proj, w, tau, size - o, bound, stride);
        lapply_householder(&mut h[offset..], proj, w, tau, bound, range, stride);
        // ----------------- tracking the rotation matrix
        lapply_householder(&mut r[offset..], proj, w, tau, bound, size, stride);
    }
}
pub fn full_francis_iteration_cpx_2x2(
    h: &mut [f64],
    r: &mut [f64],
    size: usize,
    stride: usize,
    tl: usize,
    bl: usize,
) {
    let eig = eigen(h[tl], h[tl + 1], h[bl], h[bl + 1]);
    let (_, cosine, sine) = implicit_givens_rotation(h[0] - eig, h[1]);
    apply_gt_right(h, 0, 1, stride, size, cosine, sine);
    apply_g_left(h, 0, 1, stride, 2, cosine, sine);
    apply_g_left(r, 0, 1, stride, size, cosine, sine);
}
/// full_francis_iteration_sym
///
/// * h: hessenberg linearized matrix
/// * r: rotation accumulated linearized matrix
/// * size: static number of rows for rotations
/// * range: number of rows in active window
/// * stride: stride of the data format
/// * tl: top left of the window for the eigens
/// * bl: bottom left of the window for the eigens
pub fn full_francis_iteration_sym(
    h: &mut [f64],
    r: &mut [f64],
    size: usize,
    range: usize,
    stride: usize,
    tl: usize,
    bl: usize,
) {
    let eig = eigen(h[tl], h[tl + 1], h[bl], h[bl + 1]);
    let (_, cosine, sine) = implicit_givens_rotation(h[0] - eig, h[1]);
    apply_gt_right(h, 0, 1, stride, size, cosine, sine);
    apply_g_left(h, 0, 1, stride, size, cosine, sine);
    apply_g_left(r, 0, 1, stride, size, cosine, sine);
    for o in 0..range.saturating_sub(2) {
        let row = o * stride;
        let s1 = o + 1;
        let s2 = o + 2;
        let (_, cosine, sine) = implicit_givens_rotation(h[row + s1], h[row + s2]);
        apply_gt_right(&mut h[row..], s1, s2, stride, range - o, cosine, sine);
        apply_g_left(h, s1, s2, stride, range, cosine, sine);
        apply_g_left(r, s1, s2, stride, size, cosine, sine);
    }
}
/// full_hessenberg
/// * h: matrix to create the hessenberg
/// * r: rotation matrix should be identity on coldstart
/// * p: projection vector
/// * w: workspace vector
/// * rows: number of rows
/// * cols: number of cols
/// * stride: stride of the data
pub fn full_hessenberg(
    h: &mut [f64],
    r: &mut [f64],
    p: &mut [f64],
    w: &mut [f64],
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
        let (slice, t) = h.split_at_mut(offset + stride);
        let slice = &mut slice[offset + o..offset + cols];
        let proj = &mut p[..split_range];
        let tau = params(slice, proj);
        offset += stride;
        if tau == 0f64 {
            continue;
        }
        rapply_householder(&mut t[o..], proj, w, tau, rows - o, split_range, stride);
        lapply_householder(&mut h[offset..], proj, w, tau, active_range, cols, stride);
        lapply_householder(&mut r[offset..], proj, w, tau, active_range, cols, stride);
    }
}
#[cfg(test)]
mod test_hessenberg_reconstructions {
    use super::*;
    use rand::prelude::*;

    use jedvek::Matrix2D;
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
        Matrix2D::from_flat(data.to_vec(), 0.0, rows, cols).unwrap()
    }
    fn flat(m: &Matrix2D<f64>) -> Vec<f64> {
        m.rows().flatten().copied().collect()
    }
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

            full_hessenberg(&mut h, &mut r, &mut p, &mut w, rows, cols, stride);

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

            full_hessenberg(&mut h, &mut r, &mut p, &mut w, rows, cols, stride);

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

        full_hessenberg(&mut h, &mut r, &mut p, &mut w, rows, cols, stride);
        let converged = full_decomp_sym(&mut h, &mut r, c, c, c);

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

        full_hessenberg(&mut h, &mut r, &mut p, &mut w, rows, cols, stride);
        let converged = full_decomp_cpx(&mut h, &mut r, &mut w, c, c, c);

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
