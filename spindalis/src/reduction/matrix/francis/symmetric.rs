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
