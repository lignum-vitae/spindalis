pub fn round_f64(value: f64, decimals: i32) -> f64 {
    (value * 10_f64.powi(decimals)).round() / 10_f64.powi(decimals)
}
