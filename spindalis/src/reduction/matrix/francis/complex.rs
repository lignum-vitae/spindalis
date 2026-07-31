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
    h: &mut [f64],
    p: &mut [f64],
    w: &mut [f64],
    mut range: usize,
    size: usize,
    stride: usize,
    max_iters: usize,
    tolerance: f64,
) {
    p.fill(0f64);
    let s = range * stride;
    // error 1 supra-diagonal above the first real eigen
    // error 2 supra-diagonal above the second complex real eigen
    let mut e1 = s.saturating_sub(stride + 1);
    let mut e2 = s.saturating_sub(stride + stride + 2);
    let mut tl = s.saturating_sub(stride + 2);
    let mut bl = s.saturating_sub(2);
    let mut curriter = 0;
    let mut stall = 0;
    while range > 0 && curriter < max_iters {
        curriter += 1;
        if h[e1].abs() < tolerance {
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
        } else if h[e2].abs() < tolerance {
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
                francis_iteration_cpx_2x2(h, size, stride, tl, bl);
            } else if (stall + EXCEPTION_SHIFT_OFFSET).is_multiple_of(EXCEPTION_SHIFT_PERIOD) {
                exception_shift(h, w, stride, range, tl, bl);
                francis_iteration_cpx(h, p, w, size, range, stride);
            } else {
                double_shift(h, w, stride, range, tl, bl);
                francis_iteration_cpx(h, p, w, size, range, stride);
            }
            stall += 1;
        }
    }
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
pub fn francis_iteration_cpx(
    h: &mut [f64],
    p: &mut [f64],
    w: &mut [f64],
    size: usize,
    range: usize,
    stride: usize,
) {
    let bound = range.min(3);
    let p = &mut p[..bound];
    let tau = params(&mut w[..bound], p);
    if tau != 0f64 {
        rapply_householder(h, p, w, tau, size, bound, stride);
        lapply_householder(h, p, w, tau, bound, range, stride);
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
    }
}
/// francis_iteration_cpx_2x2
///
/// * h: hessenberg linearized matrix
/// * size: static number of rows for rotations
/// * range: number of rows in active window
/// * stride: stride of the data format
/// * tl : top-left of the eigen-pair
/// * bl : bottom-left of the eigen-pair
pub fn francis_iteration_cpx_2x2(h: &mut [f64], size: usize, stride: usize, tl: usize, bl: usize) {
    let eig = eigen(h[tl], h[tl + 1], h[bl], h[bl + 1]);
    let (_, cosine, sine) = implicit_givens_rotation(h[0] - eig, h[1]);
    apply_gt_right(h, 0, 1, stride, size, cosine, sine);
    apply_g_left(h, 0, 1, stride, 2, cosine, sine);
}
