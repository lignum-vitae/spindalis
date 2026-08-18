use crate::reduction::matrix::francis::primitives::{
    lapply_householder, params, rapply_householder,
};
fn zero_col(b: &mut [f64], p: &mut [f64], w: &mut [f64], ract: usize, cact: usize, stride: usize) {
    let mut roffset = 0;
    for k in 0..ract {
        w[k] = b[roffset];
        b[roffset] = 0f64;
        roffset += stride;
    }
    let proj = &mut p[..ract];
    let tau = params(&mut w[..ract], proj);
    b[0] = w[0];
    if cact != 0 && tau != 0f64 {
        lapply_householder(&mut b[1..], proj, w, tau, ract, cact, stride);
    }
}
fn zero_row(b: &mut [f64], p: &mut [f64], w: &mut [f64], ract: usize, cact: usize, stride: usize) {
    let slice = &mut b[..cact];
    let proj = &mut p[..cact];
    let tau = params(slice, proj);
    if ract != 0 && tau != 0f64 {
        rapply_householder(&mut b[stride..], proj, w, tau, ract, cact, stride);
    }
}
/// # ubidiagonal :: upper bidiagonal
///
/// * b: matrix to create the bidiagonal
/// * p: projection vector
/// * w: workspace vector
/// * rows: number of rows
/// * cols: number of cols
/// * stride: stride of the data
#[rustfmt::skip]
pub fn ubidiagonal(
    b: &mut [f64],
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
        zero_col(&mut b[offset + k..], p, w, ract, cact - 1, stride);
        zero_row(&mut b[offset + k + 1..], p, w, ract - 1, cact - 1, stride);
        ract -= 1;
        cact -= 1;
        offset += stride;
    }
    if rows > cols {
        zero_col(&mut b[offset + pivot..], p, w, ract, 0, stride);
    }
}
/// # lbidiagonal :: lower bidiagonal
///
/// * b: matrix to create the bidiagonal
/// * p: projection vector
/// * w: workspace vector
/// * rows: number of rows
/// * cols: number of cols
/// * stride: stride of the data
#[rustfmt::skip]
pub fn lbidiagonal(
    b: &mut [f64],
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
    let pivot = card.saturating_sub(1);
    let mut offset = 0;
    for k in 0..pivot {
        zero_row(&mut b[offset + k..], p, w, ract - 1, cact, stride);
        zero_col(&mut b[offset + k + stride..], p, w, ract - 1, cact - 1, stride);
        ract -= 1;
        cact -= 1;
        offset += stride;
    }
    if cols > rows {
        zero_row(&mut b[offset + pivot..], p, w, 0, cact, stride);
    }
}
