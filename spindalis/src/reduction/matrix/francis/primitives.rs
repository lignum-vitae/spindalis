use crate::reduction::matrix::francis::constants::{EPSILON, MAX_ITERS};
/// params
/// takes in data forom a matrix slice
/// zeros the incoming data and creates the householder vec
///
/// if the vector is less than the tolerance the workspace vec
/// will return nonsense
///
/// * v: matrix slice data
/// * w: sized workspace vector
pub fn params(v: &mut [f64], workspace: &mut [f64]) -> f64 {
    debug_assert_eq!(v.len(), workspace.len());
    let mut max_element = 0f64;
    for val in v.iter() {
        let v = val.abs();
        if v > max_element {
            max_element = v
        };
    }
    if max_element.abs() < EPSILON {
        workspace[0] = 1f64;
        return 0f64;
    }
    let mut magnitude_squared = 0f64;
    let inv_max_element = 1f64 / max_element;
    for (val, gbg) in v.iter_mut().zip(workspace.iter_mut()) {
        *val *= inv_max_element;
        magnitude_squared += *val * *val;
        *gbg = *val;
        *val = 0f64;
    }
    let g = workspace[0].signum() * magnitude_squared.sqrt();
    let scale = workspace[0] + g;
    let inv_scale = 1f64 / scale;
    for val in workspace[1..].iter_mut() {
        *val *= inv_scale;
    }
    v[0] = -g * max_element;
    workspace[0] = 1f64;
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
    hess_lin_matrix: &mut [f64],
    projection: &mut [f64],
    workspace: &mut [f64],
    tau: f64,
    rows: usize,
    cols: usize,
    stride: usize,
) {
    debug_assert!(cols <= workspace.len());
    debug_assert_eq!(rows, projection.len());
    // (I - tuu')A;
    // A -= t*uu'A;
    // w := u'A;
    // R -= t*uw';
    let mut roffset = 0;
    for j in 0..cols {
        // let scalar = p[0];
        // scalar implicitly 1
        workspace[j] = hess_lin_matrix[j];
    }
    for i in 1..rows {
        roffset += stride;
        let scalar = projection[i];
        for j in 0..cols {
            workspace[j] += scalar * hess_lin_matrix[roffset + j];
        }
    }
    for j in 0..cols {
        workspace[j] *= tau;
        hess_lin_matrix[j] -= workspace[j];
    }
    roffset = 0;
    for i in 1..rows {
        roffset += stride;
        for j in 0..cols {
            hess_lin_matrix[roffset + j] -= projection[i] * workspace[j];
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
    hess_lin_matrix: &mut [f64],
    projection: &mut [f64],
    workspace: &mut [f64],
    tau: f64,
    rows: usize,
    cols: usize,
    stride: usize,
) {
    debug_assert!(rows <= workspace.len());
    debug_assert_eq!(cols, projection.len());
    // A(I - tuu');
    // A - t*Auu';
    // w := Au;
    // R -= t*wu;
    let mut roffset = 0;
    for i in 0..rows {
        workspace[i] = hess_lin_matrix[roffset];
        for k in 1..cols {
            workspace[i] += hess_lin_matrix[roffset + k] * projection[k];
        }
        workspace[i] *= tau;
        roffset += stride;
    }
    roffset = 0;
    for i in 0..rows {
        hess_lin_matrix[roffset] -= workspace[i];
        for j in 1..cols {
            hess_lin_matrix[roffset + j] -= workspace[i] * projection[j];
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
    hess_lin_matrix: &mut [f64],
    projection: &mut [f64],
    workspace: &mut [f64],
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
        let (slice, t) = hess_lin_matrix.split_at_mut(offset + stride);
        let slice = &mut slice[offset + o..offset + cols];
        let proj = &mut projection[..split_range];
        let tau = params(slice, proj);
        offset += stride;
        if tau == 0f64 {
            continue;
        }
        rapply_householder(
            &mut t[o..],
            proj,
            workspace,
            tau,
            rows - o,
            split_range,
            stride,
        );
        lapply_householder(
            &mut hess_lin_matrix[offset..],
            proj,
            workspace,
            tau,
            active_range,
            cols,
            stride,
        );
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
    curriter: &mut usize,
) {
    let shift = amount * stride + amount;
    *range -= amount;
    *e1 = e1.saturating_sub(shift);
    *e2 = e2.saturating_sub(shift);
    *tl = tl.saturating_sub(shift);
    *bl = bl.saturating_sub(shift);
    *curriter = curriter.saturating_sub(MAX_ITERS >> 1);
}
pub fn complex_eig_pair(hess_lin_matrix: &mut [f64], tl: usize, bl: usize) -> bool {
    let d = (hess_lin_matrix[tl] - hess_lin_matrix[bl + 1]) / 2f64;
    d * d + hess_lin_matrix[tl + 1] * hess_lin_matrix[bl] < EPSILON
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
    hess_lin_matrix: &mut [f64],
    workspace: &mut [f64],
    stride: usize,
    top_left: usize,
    bottom_left: usize,
) {
    // u1 = a + bi;
    // u2 = a - bi;
    // M = H^2 - H(u1 + u2) +Iu1 *u2;
    // M = H^2 - H *trace +I * det;
    let (m00, m01) = (hess_lin_matrix[top_left], hess_lin_matrix[top_left + 1]);
    let (m10, m11) = (
        hess_lin_matrix[bottom_left],
        hess_lin_matrix[bottom_left + 1],
    );

    let (h00, h01) = (hess_lin_matrix[0], hess_lin_matrix[1]);
    let (h10, h11) = (hess_lin_matrix[stride], hess_lin_matrix[stride + 1]);
    let h12 = hess_lin_matrix[stride + 2];

    let trace = m00 + m11;
    let deter = m00 * m11 - m01 * m10;

    workspace[0] = h00 * h00 + h01 * h10 - trace * h00 + deter;
    workspace[1] = h01 * (h00 + h11 - trace);
    workspace[2] = h01 * h12;
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
    hess_lin_matrix: &mut [f64],
    workspace: &mut [f64],
    stride: usize,
    tl: usize,
) {
    // u1 = a + bi;
    // u2 = a - bi;
    // M = H^2 - H(u1 + u2) +Iu1 *u2;
    // M = H^2 - H *trace + I * det;
    let m01 = hess_lin_matrix[tl + 1];

    let (h00, h01) = (hess_lin_matrix[0], hess_lin_matrix[1]);
    let (h10, h11) = (hess_lin_matrix[stride], hess_lin_matrix[stride + 1]);
    let h12 = hess_lin_matrix[stride + 2];

    let s = m01.abs() + h01.abs();
    let trace = 2.0 * s;
    let deter = s * s;

    workspace[0] = h00 * h00 + h01 * h10 - trace * h00 + deter;
    workspace[1] = h01 * (h00 + h11 - trace);
    workspace[2] = h01 * h12;
}
pub fn eigen(m00: f64, m01: f64, m10: f64, m11: f64) -> f64 {
    let d = (m00 - m11) / 2f64;
    let discriminate = d * d + m10 * m01;
    if discriminate >= -EPSILON {
        m11 + d - d.signum() * discriminate.max(0f64).sqrt()
    } else {
        m11 + d
    }
}
