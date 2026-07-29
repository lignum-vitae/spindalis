use crate::decomposition::francis::constants::{EPSILON, MAX_ITERS};
/// params
/// takes in data forom a matrix slice
/// zeros the incoming data and creates the householder vec
///
/// if the vector is less than the tolerance the workspace vec
/// will return nonsense
///
/// * v: matrix slice data
/// * w: sized workspace vector
pub fn params(v: &mut [f32], w: &mut [f32]) -> f32 {
    debug_assert_eq!(v.len(), w.len());
    let mut max_element = 0f32;
    for val in v.iter() {
        let v = val.abs();
        if v > max_element {
            max_element = v
        };
    }
    if max_element.abs() < EPSILON {
        w[0] = 1f32;
        return 0f32;
    }
    let mut magnitude_squared = 0f32;
    let inv_max_element = 1f32 / max_element;
    for (val, gbg) in v.iter_mut().zip(w.iter_mut()) {
        *val *= inv_max_element;
        magnitude_squared += *val * *val;
        *gbg = *val;
        *val = 0f32;
    }
    let g = w[0].signum() * magnitude_squared.sqrt();
    let scale = w[0] + g;
    let inv_scale = 1f32 / scale;
    for val in w[1..].iter_mut() {
        *val *= inv_scale;
    }
    v[0] = -g * max_element;
    w[0] = 1f32;
    scale / g
}
/// lapply_householder
///
/// applies the transformation directly starting here to apply
/// to columns 1..cols, simply index into the data and then
/// stride = cols
/// cols = cols - 1;
///
/// * h: matrix linear data slice
/// * p: projection slice
/// * w: workspace slice
/// * rows: number of rows
/// * cols: number of cols
/// * stride: stride of the data
pub fn lapply_householder(
    h: &mut [f32],
    p: &mut [f32],
    w: &mut [f32],
    tau: f32,
    rows: usize,
    cols: usize,
    stride: usize,
) {
    debug_assert!(cols <= w.len());
    debug_assert_eq!(rows, p.len());
    // (I - tuu')A;
    // A -= t*uu'A;
    // w := u'A;
    // R -= t*uw';
    let mut roffset = 0;
    for j in 0..cols {
        // let scalar = p[0];
        // scalar implicitly 1
        w[j] = h[j];
    }
    for i in 1..rows {
        roffset += stride;
        let scalar = p[i];
        for j in 0..cols {
            w[j] += scalar * h[roffset + j];
        }
    }
    for j in 0..cols {
        w[j] *= tau;
        h[j] -= w[j];
    }
    roffset = 0;
    for i in 1..rows {
        roffset += stride;
        for j in 0..cols {
            h[roffset + j] -= p[i] * w[j];
        }
    }
}
/// rapply_householder
///
/// applies the transformation directly starting here to apply
/// to columns 1..cols, simply index into the data and then
/// stride = cols
/// cols = cols - 1;
///
/// * h: hessenberg matrix data
/// * p: projection vector
/// * w: workspace vector
/// * rows: number of rows
/// * cols: number of cols
/// * stride: stride of the data
pub fn rapply_householder(
    h: &mut [f32],
    p: &mut [f32],
    w: &mut [f32],
    tau: f32,
    rows: usize,
    cols: usize,
    stride: usize,
) {
    debug_assert!(rows <= w.len());
    debug_assert_eq!(cols, p.len());
    // A(I - tuu');
    // A - t*Auu';
    // w := Au;
    // R -= t*wu;
    let mut roffset = 0;
    for i in 0..rows {
        w[i] = h[roffset];
        for k in 1..cols {
            w[i] += h[roffset + k] * p[k];
        }
        w[i] *= tau;
        roffset += stride;
    }
    roffset = 0;
    for i in 0..rows {
        h[roffset] -= w[i];
        for j in 1..cols {
            h[roffset + j] -= w[i] * p[j];
        }
        roffset += stride;
    }
}
/// hessenberg
/// * h: matrix to create the hessenberg
/// * p: projection vector
/// * w: workspace vector
/// * rows: number of rows
/// * cols: number of cols
/// * stride: stride of the data
pub fn hessenberg(
    h: &mut [f32],
    p: &mut [f32],
    w: &mut [f32],
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
        if tau == 0f32 {
            continue;
        }
        rapply_householder(&mut t[o..], proj, w, tau, rows - o, split_range, stride);
        lapply_householder(&mut h[offset..], proj, w, tau, active_range, cols, stride);
    }
}
pub fn deflate(
    amount: usize,
    stride: usize,
    range: &mut usize,
    e1: &mut usize,
    e2: &mut usize,
    tl: &mut usize,
    bl: &mut usize,
    // stall: &mut usize,
    curriter: &mut usize,
) {
    let shift = amount * stride + amount;
    *range -= amount;
    *e1 = e1.saturating_sub(shift);
    *e2 = e2.saturating_sub(shift);
    *tl = tl.saturating_sub(shift);
    *bl = bl.saturating_sub(shift);
    // *stall = 0;
    *curriter = curriter.saturating_sub(MAX_ITERS >> 1);
}
pub fn complex_eig_pair(h: &mut [f32], tl: usize, bl: usize) -> bool {
    let d = (h[tl] - h[bl + 1]) / 2f32;
    d * d + h[tl + 1] * h[bl] < EPSILON
}
/// double_shift
///   - standard shift for francis iteration
///
/// * h: hessenberg linearized matrix
/// * p: projection slice
/// * w: workspace slice
/// * range: number of rows in active window
/// * stride: stride of the data format
pub fn double_shift(
    h: &mut [f32],
    w: &mut [f32],
    stride: usize,
    _range: usize,
    tl: usize,
    bl: usize,
) {
    // u1 = a + bi;
    // u2 = a - bi;
    // M = H^2 - H(u1 + u2) +Iu1 *u2;
    // M = H^2 - H *trace +I * det;
    let (m00, m01) = (h[tl], h[tl + 1]);
    let (m10, m11) = (h[bl], h[bl + 1]);

    let (h00, h01) = (h[0], h[1]);
    let (h10, h11) = (h[stride], h[stride + 1]);
    let h12 = h[stride + 2];

    let trace = m00 + m11;
    let deter = m00 * m11 - m01 * m10;

    w[0] = h00 * h00 + h01 * h10 - trace * h00 + deter;
    w[1] = h01 * (h00 + h11 - trace);
    w[2] = h01 * h12;
}
/// exception_shift
///   - standard shift for francis iteration
///
/// * h: hessenberg linearized matrix
/// * p: projection slice
/// * w: workspace slice
/// * range: number of rows in active window
/// * stride: stride of the data format
pub fn exception_shift(
    h: &mut [f32],
    w: &mut [f32],
    stride: usize,
    _range: usize,
    tl: usize,
    bl: usize,
) {
    // u1 = a + bi;
    // u2 = a - bi;
    // M = H^2 - H(u1 + u2) +Iu1 *u2;
    // M = H^2 - H *trace + I * det;
    let (_m00, m01) = (h[tl], h[tl + 1]);
    let (_m10, _m11) = (h[bl], h[bl + 1]);

    let (h00, h01) = (h[0], h[1]);
    let (h10, h11) = (h[stride], h[stride + 1]);
    let h12 = h[stride + 2];

    let s = m01.abs() + h01.abs();
    let trace = 2.0 * s;
    let deter = s * s;

    w[0] = h00 * h00 + h01 * h10 - trace * h00 + deter;
    w[1] = h01 * (h00 + h11 - trace);
    w[2] = h01 * h12;
}
pub fn eigen(m00: f32, m01: f32, m10: f32, m11: f32) -> f32 {
    let d = (m00 - m11) / 2f32;
    let discriminate = d * d + m10 * m01;
    if discriminate >= -EPSILON {
        m11 + d - d.signum() * discriminate.max(0f32).sqrt()
    } else {
        m11 + d
    }
}
