use crate::reduction::matrix::svd::constants::MAX_ITERS;

pub fn singular(m00: f64, m01: f64, m10: f64, m11: f64) -> f64 {
    let off_diag = m00 * m01 + m10 * m11;
    let m00 = m00 * m00 + m10 * m10;
    let m11 = m01 * m01 + m11 * m11;
    let d = (m00 - m11) / 2f64;
    let discriminate = d * d + off_diag * off_diag;
    m11 + d - d.signum() * discriminate.max(0f64).sqrt()
}

pub fn deflate(
    amount: usize,
    stride: usize,
    range: &mut usize,
    inter: &mut usize,
    e1: &mut usize,
    tl: &mut usize,
    bl: &mut usize,
    // stall: &mut usize,
    curriter: &mut usize,
) {
    let shift = amount * stride + amount;
    *range -= amount;
    *inter = inter.saturating_sub(amount);
    *e1 = e1.saturating_sub(shift);
    *tl = tl.saturating_sub(shift);
    *bl = bl.saturating_sub(shift);
    *curriter = curriter.saturating_sub(MAX_ITERS >> 1);
}
