pub fn implicit_givens_rotation(a: f64, b: f64) -> (f64, f64, f64) {
    let ratio: f64;
    let norm_scale: f64;
    let sine: f64;
    let cosine: f64;
    let radius: f64;

    if a == 0f64 {
        cosine = 0f64;
        sine = 1f64;
        radius = b;
    } else if b.abs() > a.abs() {
        ratio = a / b;
        norm_scale = (1f64 + ratio * ratio).sqrt();
        sine = 1f64 / norm_scale;
        cosine = sine * ratio;
        radius = b * norm_scale;
    } else {
        ratio = b / a;
        norm_scale = (1f64 + ratio * ratio).sqrt();
        cosine = 1f64 / norm_scale;
        sine = cosine * ratio;
        radius = a * norm_scale;
    }
    (radius, cosine, sine)
}
pub fn apply_g_left(
    a: &mut [f64],
    i: usize,
    j: usize,
    stride: usize,
    range: usize,
    cosine: f64,
    sine: f64,
) {
    // G * A
    // alpha, beta, gamma, delta,
    // c, s, -s, c
    let r1 = i * stride;
    let r2 = j * stride;
    for k in 0..range {
        // alpha a[i*,k] + beta a[j*, k];
        let i_replace = cosine * a[r1 + k] + sine * a[r2 + k];
        // gamma a[i*,k] + delta a[j*, k];
        let j_replace = -sine * a[r1 + k] + cosine * a[r2 + k];
        a[r1 + k] = i_replace;
        a[r2 + k] = j_replace;
    }
}
pub fn apply_g_right(
    a: &mut [f64],
    i: usize,
    j: usize,
    stride: usize,
    range: usize,
    cosine: f64,
    sine: f64,
) {
    // A * G
    // alpha, beta, gamma, delta,
    // c, s, -s, c
    let mut r = 0;
    for _ in 0..range {
        // alpha a[l,i*] + gamma a[l, j*];
        let i_replace = cosine * a[r + i] - sine * a[r + j];
        // beta a[l,i*] + delta a[l, j*];
        let j_replace = sine * a[r + i] + cosine * a[r + j];
        a[r + i] = i_replace;
        a[r + j] = j_replace;
        r += stride;
    }
}
pub fn apply_gt_left(
    a: &mut [f64],
    i: usize,
    j: usize,
    stride: usize,
    range: usize,
    cosine: f64,
    sine: f64,
) {
    // G' * A
    // transpose the negative sine
    // alpha, beta, gamma, delta,
    // c, -s, s, c
    apply_g_left(a, i, j, stride, range, cosine, -sine);
}
pub fn apply_gt_right(
    a: &mut [f64],
    i: usize,
    j: usize,
    stride: usize,
    range: usize,
    cosine: f64,
    sine: f64,
) {
    // A * G'
    // alpha, beta, gamma, delta,
    // c, -s, s, c
    apply_g_right(a, i, j, stride, range, cosine, -sine);
}
