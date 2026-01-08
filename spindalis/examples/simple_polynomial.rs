use spindalis::polynomials::parse_simple_polynomial;
use spindalis::polynomials::{PolynomialTraits, SimplePolynomial};

fn main() {
    let polynomial = "5a^3 + 4a^4 - 5a^2 + 1";
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
    let value = struct_parsed.eval_univariate(2.0).unwrap();
    println!("Polynomial evaluated at x=2: {value}\n");

    // Derivative of parsed polynomial
    println!("Finding derivatives of polynomials");
    let dx = struct_parsed.derivate_univariate().unwrap();
    println!("Derivative of simple polynomial: {dx}");

    // Indefinite integral of parsed polynomial
    println!("Finding indefinite integral of polynomials");
    let anti_dx = struct_parsed.indefinite_integral_univariate().unwrap();
    println!("Indefinite integral: {anti_dx:.2}");
}
