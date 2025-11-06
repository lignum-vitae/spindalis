use spindalis::integrals::analytical_integral;

#[test]
fn test_analytical_integral_cubic() {
    // ∫₀¹ x³ dx = 1/4
    let poly = [0.0, 0.0, 0.0, 1.0];
    let result = analytical_integral(&poly, 0.0, 1.0);
    assert!((result - (1.0 / 4.0)).abs() < 1e-6);
}

#[test]
fn test_analytical_integral_quadratic() {
    // ∫₀¹ x² dx = 1/3
    let poly = [0.0, 0.0, 1.0];
    let result = analytical_integral(&poly, 0.0, 1.0);
    assert!((result - (1.0 / 3.0)).abs() < 1e-6);
}

#[test]
fn test_analytical_integral_linear() {
    // ∫₀¹ (2x + 1) dx = 2
    let poly = [1.0, 2.0];
    let result = analytical_integral(&poly, 0.0, 1.0);
    assert!((result - 2.0).abs() < 1e-6);
}

#[test]
fn test_analytical_integral_constant() {
    // ∫₀¹ 3 dx = 3
    let poly = [3.0];
    let result = analytical_integral(&poly, 0.0, 1.0);
    assert!((result - 3.0).abs() < 1e-6);
}
