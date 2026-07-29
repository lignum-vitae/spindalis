use crate::decomposition::sgivens::{apply_g_left, apply_gt_right, implicit_givens_rotation};
#[rustfmt::skip]
use crate::decomposition::francis::primitives::{
    deflate,
    eigen,
};

#[rustfmt::skip]
pub fn decomp_sym(
    h: &mut [f32],
    mut range: usize,
    size: usize,
    stride: usize,
    max_iters:usize,
    tolerance: f32,
    absolute: f32,
) {
    let s = range * stride;
    // error 1 supra-diagonal above the first real eigen
    // error 2 supra-diagonal above the second complex real eigen
    let mut e1 = s.saturating_sub(stride + 1);
    let mut e2 = s.saturating_sub(stride + stride + 2);
    let mut tl = s.saturating_sub(stride + 2);
    let mut bl = s.saturating_sub(2);
    let mut curriter = 0;
    while range > 1 && curriter < max_iters {
        let scale = h[tl].abs() + h[bl+1].abs();
        curriter += 1;
        if h[e1].abs() < (scale * tolerance).min(absolute) {
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
        } else if h[e2].abs() < tolerance && curriter == max_iters {
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
        } else {
            francis_iteration_sym(h, size, range, stride, tl, bl);
        }
    }
}
/// francis_iteration_sym
///
/// * h: hessenberg linearized matrix
/// * size: static number of rows for rotations
/// * range: number of rows in active window
/// * stride: stride of the data format
/// * tl: top left of the window for the eigens
/// * bl: bottom left of the window for the eigens
pub fn francis_iteration_sym(
    h: &mut [f32],
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
    for o in 0..range.saturating_sub(2) {
        let row = o * stride;
        let s1 = o + 1;
        let s2 = o + 2;
        let (_, cosine, sine) = implicit_givens_rotation(h[row + s1], h[row + s2]);
        apply_gt_right(&mut h[row..], s1, s2, stride, range - o, cosine, sine);
        apply_g_left(h, s1, s2, stride, range, cosine, sine);
    }
}
