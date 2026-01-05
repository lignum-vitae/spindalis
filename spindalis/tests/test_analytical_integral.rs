use spindalis::integrals::analytical_integral;
use spindalis::polynomials::{PolynomialExtended, PolynomialTraits, SimplePolynomial};

#[test]
fn test_analytical_integral_cubic() {
    // ∫₀¹ x³ dx = 1/4
    let poly = SimplePolynomial::parse("x^3").unwrap();
    let result = analytical_integral(&poly, 0.0, 1.0).unwrap();
    assert!((result - (1.0 / 4.0)).abs() < 1e-6);
}

#[test]
fn test_analytical_integral_quadratic() {
    // ∫₀¹ x² dx = 1/3
    let poly = PolynomialExtended::parse("x^2").unwrap();
    let result = analytical_integral(&poly, 0.0, 1.0).unwrap();
    assert!((result - (1.0 / 3.0)).abs() < 1e-6);
}

#[test]
fn test_analytical_integral_linear() {
    // ∫₀¹ (2x + 1) dx = 2
    let poly = SimplePolynomial::parse("2x+1").unwrap();
    let result = analytical_integral(&poly, 0.0, 1.0).unwrap();
    assert!((result - 2.0).abs() < 1e-6);
}

#[test]
fn test_analytical_integral_constant() {
    // ∫₀¹ 3 dx = 3
    let poly = PolynomialExtended::parse("3").unwrap();
    let result = analytical_integral(&poly, 0.0, 1.0).unwrap();
    assert!((result - 3.0).abs() < 1e-6);
}

#[test]
fn test_parsed_polynomial() {
    let poly = SimplePolynomial::parse("3x ^ 2 - 1").unwrap();
    let result = analytical_integral(&poly, 1.0, 5.0).unwrap();
    assert!((result - 120_f64).abs() < 1e-6);
}
