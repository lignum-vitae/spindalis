use std::f64;

// Lanczos approximation of the gamma function (g = 7, 9 coefficients)
// Source: https://grokipedia.com/page/Lanczos_approximation
pub fn gamma_f64(z: f64) -> f64 {
    const G: f64 = 7.0;
    const COEFFICIENTS: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];

    let z = z - 1.;
    let series = COEFFICIENTS
        .iter()
        .enumerate()
        .skip(1)
        .fold(COEFFICIENTS[0], |acc, (i, c)| acc + c / (z + i as f64));
    let t = z + G + 0.5;
    let log_gamma = ((2.0 * f64::consts::PI).sqrt()).ln() + (z + 0.5) * t.ln() - t + series.ln();

    log_gamma.exp()
}

pub fn factorial_f64(n: f64) -> f64 {
    if n < 0.0 {
        return f64::NAN;
    }
    if n.fract() != 0.0 {
        return gamma_f64(n + 1.); // Gamma(n+1) = n!
    }
    (1..=n as u64).fold(1.0, |acc, x| acc * x as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::rounding::round_f64;

    #[test]
    fn test_integer_factorial_1() {
        assert_eq!(factorial_f64(0.0), 1.0);
    }

    #[test]
    fn test_integer_factorial_2() {
        assert_eq!(factorial_f64(5.0), 120.0);
    }

    #[test]
    fn test_non_integer_factorial_1() {
        assert_eq!(round_f64(factorial_f64(0.5), 4), 0.8862);
    }

    #[test]
    fn test_non_integer_factorial_2() {
        assert_eq!(round_f64(factorial_f64(4.5), 4), 52.3428);
    }

    #[test]
    fn test_non_integer_factorial_3() {
        assert_eq!(round_f64(factorial_f64(1.5), 4), 1.3293);
    }

    #[test]
    fn test_negative_factorial_1() {
        assert!(factorial_f64(-1.0).is_nan());
    }

    #[test]
    fn test_negative_factorial_2() {
        assert!(factorial_f64(-0.5).is_nan());
    }
}
