pub fn implicit_givens_rotation(a: f64, b: f64) -> (f64, f64, f64) {
    let t: f64;
    let tt: f64;
    let s: f64;
    let c: f64;
    let r: f64;

    if a == 0f64 {
        c = 0f64;
        s = 1f64;
        r = b;
    } else if b.abs() > a.abs() {
        t = a / b;
        tt = (1f64 + t * t).sqrt();
        s = 1f64 / tt;
        c = s * t;
        r = b * tt;
    } else {
        t = b / a;
        tt = (1f64 + t * t).sqrt();
        c = 1f64 / tt;
        s = c * t;
        r = a * tt;
    }
    (r, c, s)
}
pub fn apply_g_left(
    a: &mut [f64],
    i: usize,
    j: usize,
    stride: usize,
    range: usize,
    c: f64,
    s: f64,
) {
    // G * A
    // alpha, beta, gamma, delta,
    // c, s, -s, c
    for k in 0..range {
        let r1 = i * stride;
        let r2 = j * stride;
        // alpha a[i*,k] + beta a[j*, k];
        let i_replace = c * a[r1 + k] + s * a[r2 + k];
        // gamma a[i*,k] + delta a[j*, k];
        let j_replace = -s * a[r1 + k] + c * a[r2 + k];
        a[r1 + k] = i_replace;
        a[r2 + k] = j_replace;
    }
}
pub fn apply_gt_left(
    a: &mut [f64],
    i: usize,
    j: usize,
    stride: usize,
    range: usize,
    c: f64,
    s: f64,
) {
    // G' * A
    // transpose the negative sine
    // alpha, beta, gamma, delta,
    // c, -s, s, c
    let r1 = i * stride;
    let r2 = j * stride;
    for k in 0..range {
        // alpha a[i*,j] + beta a[j*, j];
        let i_replace = c * a[r1 + k] - s * a[r2 + k];
        // gamma a[i*,j] + delta a[j*, j];
        let j_replace = s * a[r1 + k] + c * a[r2 + k];
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
    c: f64,
    s: f64,
) {
    // A * G
    // alpha, beta, gamma, delta,
    // c, s, -s, c
    let mut r = 0;
    for _ in 0..range {
        // alpha a[l,i*] + gamma a[l, j*];
        let i_replace = c * a[r + i] - s * a[r + j];
        // beta a[l,i*] + delta a[l, j*];
        let j_replace = s * a[r + i] + c * a[r + j];
        a[r + i] = i_replace;
        a[r + j] = j_replace;
        r += stride;
    }
}
pub fn apply_gt_right(
    a: &mut [f64],
    i: usize,
    j: usize,
    stride: usize,
    range: usize,
    c: f64,
    s: f64,
) {
    // A * G'
    // alpha, beta, gamma, delta,
    // c, -s, s, c
    for l in 0..range {
        let r = l * stride;
        // alpha a[l,i*] + gamma a[l, j*];
        let i_replace = c * a[r + i] + s * a[r + j];
        // beta a[l,i*] + delta a[l, j*];
        let j_replace = -s * a[r + i] + c * a[r + j];
        a[r + i] = i_replace;
        a[r + j] = j_replace;
    }
}
