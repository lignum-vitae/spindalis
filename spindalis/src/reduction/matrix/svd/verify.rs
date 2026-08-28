use crate::reduction::matrix::francis::givens::{
    apply_g_left, apply_gt_right, implicit_givens_rotation,
};
use crate::reduction::matrix::francis::primitives::{
    lapply_householder, params, rapply_householder,
};
use crate::reduction::matrix::svd::primitives::{deflate, singular};

#[rustfmt::skip]
fn full_zero_col(
    b: &mut [f64],
    u: &mut [f64],
    p: &mut [f64],
    w: &mut [f64],
    rows: usize,
    ract: usize,
    cact: usize,
    stride: usize,
) {
    let mut roffset = 0;
    for k in 0..ract {
        w[k] = b[roffset];
        b[roffset] = 0f64;
        roffset += stride;
    }
    let proj = &mut p[..ract];
    let tau = params(&mut w[..ract], proj);
    b[0] = w[0];
    if tau == 0f64 { return; }
    if cact != 0 {
        lapply_householder(&mut b[1..], proj, w, tau, ract, cact, stride);
    }
    rapply_householder(u, proj, w, tau, rows, ract, rows);
}
fn full_zero_row(
    b: &mut [f64],
    v: &mut [f64],
    p: &mut [f64],
    w: &mut [f64],
    cols: usize,
    ract: usize,
    cact: usize,
    stride: usize,
) {
    let slice = &mut b[..cact];
    let proj = &mut p[..cact];
    let tau = params(slice, proj);
    if tau == 0f64 {
        return;
    }
    if ract != 0 {
        rapply_householder(&mut b[stride..], proj, w, tau, ract, cact, stride);
    }
    rapply_householder(v, proj, w, tau, cols, cact, cols);
}

/// # full ubidiagonal :: upper bidiagonal
///
/// * b: matrix to create the bidiagonal
/// * u: eigenvectors of AA' ie rowspace
/// * v: eigenvectors of A'A ie colspace
/// * p: projection vector
/// * w: workspace vector
/// * rows: number of rows
/// * cols: number of cols
/// * stride: stride of the data
#[rustfmt::skip]
pub fn full_ubidiagonal(
    b: &mut [f64],
    u: &mut [f64],
    v: &mut [f64],
    p: &mut [f64],
    w: &mut [f64],
    rows: usize,
    cols: usize,
    card: usize,
    stride: usize,
) {
    debug_assert!(rows >= cols, "givens rotations do not handle");
    let mut ract = rows;
    let mut cact = cols;
    let mut offset = 0;
    let pivot = card.saturating_sub(1);
    for k in 0..pivot {
        full_zero_col(&mut b[offset + k..], &mut u[k..], p, w, rows, ract, cact - 1, stride);
        full_zero_row(&mut b[offset + k + 1..], &mut v[k + 1 ..], p, w, cols, ract - 1, cact - 1, stride);
        ract -= 1;
        cact -= 1;
        offset += stride;
    }
    if rows > cols {
        full_zero_col(&mut b[offset + pivot..], &mut u[pivot..], p, w, rows, ract, 0, stride);
    }
}
/// # full_lbidiagonal :: lower bidiagonal
///
/// * b: matrix to create the bidiagonal
/// * p: projection vector
/// * w: workspace vector
/// * rows: number of rows
/// * cols: number of cols
/// * stride: stride of the data
#[rustfmt::skip]
pub fn full_lbidiagonal(
    b: &mut [f64],
    u: &mut [f64],
    v: &mut [f64],
    p: &mut [f64],
    w: &mut [f64],
    rows: usize,
    cols: usize,
    card: usize,
    stride: usize,
) {
    debug_assert!(cols >= rows, "givens rotations do not handle");
    let mut ract = rows;
    let mut cact = cols;
    let mut offset = 0;
    let pivot = card.saturating_sub(1);
    for k in 0..pivot {
        full_zero_row( &mut b[offset + k..], &mut v[k..], p, w, cols, ract - 1, cact, stride);
        full_zero_col( &mut b[offset + k + stride..], &mut u[k + 1..], p, w, rows, ract - 1, cact - 1, stride);
        ract -= 1;
        cact -= 1;
        offset += stride;
    }
    if cols > rows {
        full_zero_row( &mut b[offset + pivot..], &mut v[pivot..], p, w, cols, 0, cact, stride);
    }
}
#[rustfmt::skip]
pub fn full_decomp_ugivens(
    b: &mut [f64],
    u: &mut [f64],
    v: &mut [f64],
    rows: usize,
    cols: usize,
    card: usize,
    stride: usize,
    max_iters:usize,
    tolerance: f64,
    absolute: f64,
) {
    let mut range = card;
    let mut inter = card.saturating_sub(2);
    let s = card * stride;
    // error 1 supra-diagonal above the first real eigen
    let mut e1 = s.saturating_sub(stride + 1);
    let mut tl = s.saturating_sub(stride + 2);
    let mut bl = s.saturating_sub(2);
    let mut curriter = 0;
    while range > 1 && curriter < max_iters {
        curriter += 1;
        let scale = b[tl].abs() + b[bl+1].abs();
        if b[e1].abs() < (scale * tolerance).min(absolute) {
            deflate(
                1,
                stride,
                &mut range,
                &mut inter,
                &mut e1,
                &mut tl,
                &mut bl,
                &mut curriter,
            );
        } else {
            full_ugivens_iteration(b, u, v, inter, rows, cols, stride, tl, bl);
        }
    }
}
#[rustfmt::skip]
pub fn full_decomp_lgivens(
    b: &mut [f64],
    u: &mut [f64],
    v: &mut [f64],
    rows: usize,
    cols: usize,
    card: usize,
    stride: usize,
    max_iters:usize,
    tolerance: f64,
    absolute: f64,
) {
    let mut range = card;
    let mut inter = card.saturating_sub(2);
    let s = card * stride;
    // error 1 supra-diagonal above the first real eigen
    let mut tl = (s + card).saturating_sub(2 + stride * 2);
    let mut bl = (s + card).saturating_sub(stride + 2);
    let mut e1 = (s + card).saturating_sub(stride + 2);
    let mut curriter = 0;
    while range > 1 && curriter < max_iters {
        curriter += 1;
        let scale = b[tl].abs() + b[bl+1].abs();
        if b[e1].abs() < (scale * tolerance).min(absolute) {
            deflate(
                1,
                stride,
                &mut range,
                &mut inter,
                &mut e1,
                &mut tl,
                &mut bl,
                &mut curriter,
            );
        } else {
            full_lgivens_iteration(b, u, v, inter, rows, cols, stride, tl, bl);
        }
    }
}
fn full_ugivens_iteration(
    h: &mut [f64],
    u: &mut [f64],
    v: &mut [f64],
    interior: usize,
    rows: usize,
    cols: usize,
    stride: usize,
    tl: usize,
    bl: usize,
) {
    let mut offset = 0;
    let mut uoffset = 0;
    let mut voffset = 0;
    // push zero into col
    let sing = singular(h[tl], h[tl + 1], h[bl], h[bl + 1]);
    let sq_0 = h[0] * h[0];
    let sq_1 = h[0] * h[1];
    let (_, cos, sin) = implicit_givens_rotation(sq_0 - sing, sq_1);
    apply_gt_right(h, 0, 1, stride, 2, cos, sin);
    apply_gt_right(v, 0, 1, cols, cols, cos, sin);
    for _ in 0..interior {
        // push zero into row
        let (_, cos, sin) = implicit_givens_rotation(h[offset], h[offset + stride]);
        apply_g_left(&mut h[offset..], 0, 1, stride, 3, cos, sin);
        apply_gt_right(&mut u[uoffset..], 0, 1, rows, rows, cos, sin);
        // push zero into col
        offset += 1;
        voffset += 1;
        let (_, cos, sin) = implicit_givens_rotation(h[offset], h[offset + 1]);
        apply_gt_right(&mut h[offset..], 0, 1, stride, 3, cos, sin);
        apply_gt_right(&mut v[voffset..], 0, 1, cols, cols, cos, sin);
        uoffset += 1;
        offset += stride;
    }
    // push zero into row
    let (_, cos, sin) = implicit_givens_rotation(h[offset], h[offset + stride]);
    apply_g_left(&mut h[offset..], 0, 1, stride, 2, cos, sin);
    apply_gt_right(&mut u[uoffset..], 0, 1, rows, rows, cos, sin);
}
fn full_lgivens_iteration(
    h: &mut [f64],
    u: &mut [f64],
    v: &mut [f64],
    interior: usize,
    rows: usize,
    cols: usize,
    stride: usize,
    tl: usize,
    bl: usize,
) {
    let mut offset = 0;
    let mut uoffset = 0;
    let mut voffset = 0;
    // push zero into col
    let sing = singular(h[tl], h[tl + 1], h[bl], h[bl + 1]);
    let sq_00 = h[0] * h[0];
    let sq_10 = h[0] * h[stride];
    let (_, cos, sin) = implicit_givens_rotation(sq_00 - sing, sq_10);
    apply_g_left(h, 0, 1, stride, 2, cos, sin);
    apply_gt_right(u, 0, 1, rows, rows, cos, sin);
    for _ in 0..interior {
        // push zero into col
        let (_, cos, sin) = implicit_givens_rotation(h[offset], h[offset + 1]);
        apply_gt_right(&mut h[offset..], 0, 1, stride, 3, cos, sin);
        apply_gt_right(&mut v[voffset..], 0, 1, cols, cols, cos, sin);
        // push zero into row
        offset += stride;
        voffset += 1;
        uoffset += 1;

        let (_, cos, sin) = implicit_givens_rotation(h[offset], h[offset + stride]);
        apply_g_left(&mut h[offset..], 0, 1, stride, 3, cos, sin);
        apply_gt_right(&mut u[uoffset..], 0, 1, rows, rows, cos, sin);
        offset += 1;
    }
    // // push zero into col
    let (_, cos, sin) = implicit_givens_rotation(h[offset], h[offset + 1]);
    apply_gt_right(&mut h[offset..], 0, 1, stride, 2, cos, sin);
    apply_gt_right(&mut v[voffset..], 0, 1, cols, cols, cos, sin);
}

#[cfg(test)]
mod test_svd_reconstructions {
    use crate::reduction::matrix::svd::interface::full_svd_decomposition;
    use rand::SeedableRng;
    use rand::prelude::*;
    use rand::rngs::StdRng;
    use rand_distr::StandardNormal;
    const ABSOLUTE: f64 = 1e-4;
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
    fn check_svd_reconstruct(rows: usize, cols: usize) -> (bool, bool, bool) {
        // returns (u_ortho_ok, v_ortho_ok, reconstruction_ok)
        let card = rows.min(cols);
        let stride = cols;
        let maximum = rows.max(cols);

        let mut u = create_identity_vector(rows, rows);
        let mut v = create_identity_vector(cols, cols);
        let mut w = vec![0f64; maximum];
        let mut p = vec![0f64; maximum];

        let mut b = generate_random_vector(rows * cols);
        let original = b.clone();

        full_svd_decomposition(
            &mut b, &mut u, &mut v, &mut p, &mut w, rows, cols, card, stride, 40, 1e-10, ABSOLUTE,
        );

        let singular = b.clone();

        let u_identity = create_identity_vector(rows, rows);
        let v_identity = create_identity_vector(cols, cols);

        // U U' ~= I and U' U ~= I
        let mut ut = vec![0f64; rows * rows];
        transpose(&u, rows, rows, &mut ut);
        let mut uut = vec![0f64; rows * rows];
        matrix_mult(&u, rows, rows, &ut, rows, rows, &mut uut);
        let mut utu = vec![0f64; rows * rows];
        matrix_mult(&ut, rows, rows, &u, rows, rows, &mut utu);
        let u_ortho_ok = approx_vector_eq(&uut, &u_identity) && approx_vector_eq(&utu, &u_identity);

        // V V' ~= I and V' V ~= I
        let mut vt = vec![0f64; cols * cols];
        transpose(&v, cols, cols, &mut vt);
        let mut vvt = vec![0f64; cols * cols];
        matrix_mult(&v, cols, cols, &vt, cols, cols, &mut vvt);
        let mut vtv = vec![0f64; cols * cols];
        matrix_mult(&vt, cols, cols, &v, cols, cols, &mut vtv);
        let v_ortho_ok = approx_vector_eq(&vvt, &v_identity) && approx_vector_eq(&vtv, &v_identity);

        // U Sigma V' ~= original
        let mut us = vec![0f64; rows * cols];
        matrix_mult(&u, rows, rows, &singular, rows, cols, &mut us);
        let mut reconstruct = vec![0f64; rows * cols];
        matrix_mult(&us, rows, cols, &vt, cols, cols, &mut reconstruct);
        let recon_ok = approx_vector_eq(&reconstruct, &original);

        (u_ortho_ok, v_ortho_ok, recon_ok)
    }

    #[test]
    fn test_svd_reconstruct_square() {
        for dim in [2, 4, 7] {
            let (u_ok, v_ok, r_ok) = check_svd_reconstruct(dim, dim);
            assert!(u_ok, "dim={dim}: U not orthogonal");
            assert!(v_ok, "dim={dim}: V not orthogonal");
            assert!(r_ok, "dim={dim}: reconstruction mismatch");
        }
    }
    #[test]
    fn test_svd_reconstruct_wide() {
        // rows < cols
        for (rows, cols) in [(1, 2), (2, 4), (4, 6), (4, 8)] {
            let (u_ok, v_ok, r_ok) = check_svd_reconstruct(rows, cols);
            assert!(u_ok, "{rows}x{cols}: U not orthogonal");
            assert!(v_ok, "{rows}x{cols}: V not orthogonal");
            assert!(r_ok, "{rows}x{cols}: reconstruction mismatch");
        }
    }
    #[test]
    fn test_svd_reconstruct_tall() {
        // rows > cols
        for (rows, cols) in [(2, 1), (4, 2), (6, 4), (8, 4)] {
            let (u_ok, v_ok, r_ok) = check_svd_reconstruct(rows, cols);
            assert!(u_ok, "{rows}x{cols}: U not orthogonal");
            assert!(v_ok, "{rows}x{cols}: V not orthogonal");
            assert!(r_ok, "{rows}x{cols}: reconstruction mismatch");
        }
    }
    #[rustfmt::skip]
    #[test]
    fn test_svd_reconstruct_trials() {
        let trials = 10_000;
        let mut u_failures = 0;
        let mut v_failures = 0;
        let mut recon_failures = 0;

        for _ in 0..trials {
            let (u_ok, v_ok, r_ok) = check_svd_reconstruct(6, 6);
            if !u_ok { u_failures += 1; }
            if !v_ok { v_failures += 1; }
            if !r_ok { recon_failures += 1; }
        }

        println!("svd: {u_failures} U failures, {v_failures} V failures, {recon_failures} reconstruction failures / {trials}");
        assert!(u_failures == 0, "too many U orthogonality failures: {u_failures}");
        assert!(v_failures == 0, "too many V orthogonality failures: {v_failures}");
        assert!(recon_failures == 0, "too many reconstruction failures: {recon_failures}");
    }
}
