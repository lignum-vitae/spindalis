use spindalis::derivatives::simple_derivative;
use spindalis::polynomials::{eval_simple_polynomial, parse_simple_polynomial};

fn main() {
    // Parsing function
    let polynomial = "5x^3 + 4x^4 - 5x^2 + 1";
    let _parsed = parse_simple_polynomial(polynomial).unwrap();

    // Parsing macro
    let parsed = parse_simple_polynomial!(5x ^ 3 + 4x ^ 4 - 5x ^ 2 + 1);

    println!("Parising polynomials");
    println!("Original polynomial: {polynomial}");
    println!("Parsed polynomial: {parsed:?}\n");

    // Evaluating polynomials
    println!("Evaluating polynomials");
    let value = eval_simple_polynomial(2.0, &parsed);
    println!("Polynomial evaluated at x=2: {value:?}\n");

    // Derivative of parsed polynomial
    println!("Finding derivatives of polynomials");
    let dx = simple_derivative(&parsed);
    println!("Derivative coefficients: {dx}");
}
