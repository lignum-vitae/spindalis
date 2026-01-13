use spindalis::derivatives::partial_derivative;
use spindalis::polynomials::{IntermediatePolynomial, PolynomialTraits};
use spindalis::polynomials::{eval_intermediate_polynomial, parse_intermediate_polynomial};

#[allow(clippy::unnecessary_to_owned)]
fn main() {
    // Parsing struct
    let polynomial = "4x^2y^3 + 4x - 2y + z^1.0/2.0";
    let struct_parsed = IntermediatePolynomial::parse(polynomial).unwrap();

    // Parsing macro
    let parsed = parse_intermediate_polynomial!(4x ^ 2y ^ 3 + 4x - 2y + z ^ 1.0 / 2.0);

    println!("Parsing polynomials");
    println!("Original polynomial: {polynomial}");
    println!("Polynomial parsed with macro: {parsed:?}");
    println!("Polynomial parsed with struct: {struct_parsed}\n");

    // Evaluating polynomials
    println!("Evaluating Polynomials");
    let vars = vec![("x", 2), ("y", 1), ("z", 4)];
    println!("Evaluated with function:");
    let value = eval_intermediate_polynomial(&parsed, &vars).unwrap();
    println!("Polynomial evaluated at x=2, y=1, z=4: {value}");
    println!("Evaluated with struct:");
    let value = struct_parsed.eval_multivariate(&vars).unwrap();
    println!("Polynomial evaluated at x=2, y=1, z=4: {value}");

    let vars = [("x", 4), ("y", 5), ("z", 2)];
    let value = struct_parsed.eval_multivariate(&vars).unwrap();
    println!("Polynomial evaluated at x=4, y=5, z=2: {value:.4}");

    let vars = [("x", 0.5), ("y", 5.3), ("z", 2.1)];
    let value = struct_parsed.eval_multivariate(&vars).unwrap();
    println!("Polynomial evaluated at x=0.5, y=5.3, z=2.1: {value:.4}\n");

    // Calculative partial derivative
    println!("Partial derivatives of polynomials");
    println!("With functions:");
    let dx = partial_derivative(&parsed, "x");
    println!("Derivative with respect to x: {dx}");

    let dy = partial_derivative(&parsed, "y");
    println!("Derivative with respect to y: {dy}");

    let dz = partial_derivative(&parsed, "z".to_string());
    println!("Derivative with respect to z: {dz}\n");

    println!("With struct:");
    let dx = struct_parsed.derivate_multivariate("x");
    println!("Derivative with respect to x: {dx}");

    let dy = struct_parsed.derivate_multivariate("y");
    println!("Derivative with respect to y: {dy}");

    let dz = struct_parsed.derivate_multivariate("z");
    println!("Derivative with respect to z: {dz}\n");

    // Calculate indefinite integral
    println!("Multivariate indefinite integral with struct:");
    let anti_dx = struct_parsed.indefinite_integral_multivariate("x");
    println!("Original polynomial: {polynomial}\nX integration of multivariate: {anti_dx:.2}\n");

    println!("Univariate indefinite integral with struct:");
    let polynomial = "5x^3 + 4x^4 - 5x^2 + 1";
    let struct_parsed = IntermediatePolynomial::parse(polynomial).unwrap();
    let anti_dx = struct_parsed.indefinite_integral_univariate().unwrap();
    println!("Original polynomial: {polynomial}\nUnivariate integration: {anti_dx:.2}");
}
