use spindalis::derivatives::partial_derivative;
use spindalis::polynomials::{eval_polynomial_extended, parse_polynomial_extended};

#[allow(clippy::unnecessary_to_owned)]
fn main() {
    // Parsing function
    let polynomial = "4x^2y^3 + 4x - 2y + z^1.0/2.0";
    let _parsed = parse_polynomial_extended(polynomial).unwrap();

    // Parsing macro
    let parsed = parse_polynomial_extended!(4x ^ 2y ^ 3 + 4x - 2y + z ^ 1.0 / 2.0);

    println!("Polynomial Parsing");
    println!("Parsed polynomial: {parsed:?}\n");

    // Evaluating polynomials
    println!("Evaluating Polynomials");
    let vars = vec![("x", 2), ("y", 1), ("z", 4)];
    let value = eval_polynomial_extended(&parsed, &vars);
    println!("Polynomial evaluated at x=2, y=1, z=4: {value:?}");

    let vars = [("x", 4), ("y", 5), ("z", 2)];
    let value = eval_polynomial_extended(&parsed, &vars);
    println!("Polynomial evaluated at x=4, y=5, z=2: {value:?}");

    let vars = [("x", 0.5), ("y", 5.3), ("z", 2.1)];
    let value = eval_polynomial_extended(&parsed, &vars);
    println!("Polynomial evaluated at x=0.5, y=5.3, z=2.1: {value:?}\n");

    // Calculative partial derivative
    println!("Partial derivatives of polynomials");
    let dx = partial_derivative(&parsed, "x").terms;
    println!("Derivative with respect to x: {dx:?}");

    let dy = partial_derivative(&parsed, "y").terms;
    println!("Derivative with respect to y: {dy:?}");

    let dz = partial_derivative(&parsed, "z".to_string()).terms;
    println!("Derivative with respect to z: {dz:?}");
}
