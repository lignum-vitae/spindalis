use crate::reduction::matrix::francis::givens::{
    apply_g_left, apply_gt_right, implicit_givens_rotation,
};
#[rustfmt::skip]
use crate::reduction::matrix::francis::primitives::{
    deflate,
    eigen,
};

#[rustfmt::skip]
pub fn decomp_sym(
    hess_lin_matrix: &mut [f64],
    mut range: usize,
    size: usize,
    stride: usize,
    max_iters:usize,
    tolerance: f64,
    absolute: f64,
) {
    let s = range * stride;
    // error 1 supra-diagonal above the first real eigen
    // error 2 supra-diagonal above the second complex real eigen
    let mut error1 = s.saturating_sub(stride + 1);
    let mut error2 = s.saturating_sub(stride + stride + 2);
    let mut top_left = s.saturating_sub(stride + 2);
    let mut bottom_left = s.saturating_sub(2);
    let mut curriter = 0;
    while range > 1 && curriter < max_iters {
        let scale = hess_lin_matrix[top_left].abs() + hess_lin_matrix[bottom_left+1].abs();
        curriter += 1;
        if hess_lin_matrix[error1].abs() < (scale * tolerance).min(absolute) {
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
        } else if hess_lin_matrix[error2].abs() < tolerance && curriter == max_iters {
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
            francis_iteration_sym(hess_lin_matrix, size, range, stride, top_left, bottom_left);
        }
    }
}
/// francis_iteration_sym
///
/// * hess_lin_matrix: hessenberg linearized matrix
/// * size: static number of rows for rotations
/// * range: number of rows in active window
/// * stride: stride of the data format
/// * top_left: top left of the window for the eigens
/// * bottom_left: bottom left of the window for the eigens
pub fn francis_iteration_sym(
    hess_lin_matrix: &mut [f64],
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
    }
}
#[cfg(test)]
mod test_verify_correspondance_symmetric {
    use super::*;
    use crate::reduction::matrix::francis::constants::{ABSOLUTE_CAP, MAX_ITERS, TOLERANCE};
    use crate::reduction::matrix::francis::primitives::hessenberg_lq;
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
