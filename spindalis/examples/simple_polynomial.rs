use spindalis::derivatives::simple_derivative;
use spindalis::polynomials::{PolynomialTraits, SimplePolynomial};
use spindalis::polynomials::{eval_simple_polynomial, parse_simple_polynomial};

fn main() {
    let polynomial = "5x^3 + 4x^4 - 5x^2 + 1";
    // Parsing struct
    let struct_parsed = SimplePolynomial::parse(polynomial).unwrap();

    // Parsing macro
    let parsed = parse_simple_polynomial!(5x ^ 3 + 4x ^ 4 - 5x ^ 2 + 1);

    println!("Parising polynomials");
    println!("Original polynomial: {polynomial}");
    println!("Polynomial parsed with macro: {parsed:?}");
    println!("Polynomial parsed with struct: {struct_parsed}\n");

    // Evaluating polynomials
    println!("Evaluating polynomials");
    println!("Evaluated with function:");
    let value = eval_simple_polynomial(2.0, &parsed);
    println!("Polynomial evaluated at x=2: {value:?}");
    println!("Evaluated with struct:");
    let value = struct_parsed.eval_univariate(2.0).unwrap();
    println!("Polynomial evaluated at x=2: {value}\n");

    // Derivative of parsed polynomial
    println!("Finding derivatives of polynomials");
    println!("Derived with function:");
    let dx = simple_derivative(&parsed);
    println!("Derivative of simple polynomial: {dx}");
    println!("Derived with struct:");
    let dx = struct_parsed.derivate_univariate().unwrap();
    println!("Derivative of simple polynomial: {dx}");

    // Indefinite integral of parsed polynomial
    println!("Finding indefinite integral of polynomials");
    let anti_dx = struct_parsed.indefinite_integral_univariate().unwrap();
    println!("Indefinite integral: {anti_dx}");
}
