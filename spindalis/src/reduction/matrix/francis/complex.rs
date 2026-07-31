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

pub fn decomp_cpx(
    hess_lin_matrix: &mut [f64],
    projection: &mut [f64],
    workspace: &mut [f64],
    mut range: usize,
    size: usize,
    stride: usize,
    max_iters: usize,
    tolerance: f64,
) {
    let s = range * stride;
    // error 1 supra-diagonal above the first real eigen
    // error 2 supra-diagonal above the second complex real eigen
    let mut error1 = s.saturating_sub(stride + 1);
    let mut error2 = s.saturating_sub(stride + stride + 2);
    let mut top_left = s.saturating_sub(stride + 2);
    let mut bottom_left = s.saturating_sub(2);
    let mut curriter = 0;
    let mut stall = 0;
    while range > 0 && curriter < max_iters {
        curriter += 1;
        if hess_lin_matrix[error1].abs() < tolerance {
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
        } else if hess_lin_matrix[error2].abs() < tolerance {
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
                francis_iteration_cpx_2x2(hess_lin_matrix, size, stride, top_left, bottom_left);
            } else if (stall + EXCEPTION_SHIFT_OFFSET).is_multiple_of(EXCEPTION_SHIFT_PERIOD) {
                exception_shift(hess_lin_matrix, workspace, stride, top_left);
                francis_iteration_cpx(hess_lin_matrix, projection, workspace, size, range, stride);
            } else {
                double_shift(hess_lin_matrix, workspace, stride, top_left, bottom_left);
                francis_iteration_cpx(hess_lin_matrix, projection, workspace, size, range, stride);
            }
            stall += 1;
        }
    }
}
pub fn francis_iteration_cpx(
    hess_lin_matrix: &mut [f64],
    projection: &mut [f64],
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
    }
}
pub fn francis_iteration_cpx_2x2(
    hess_lin_matrix: &mut [f64],
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
}

#[cfg(test)]
mod test_verify_correspondance_complex {
    use super::*;
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
