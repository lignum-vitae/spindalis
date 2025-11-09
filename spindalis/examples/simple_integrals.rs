use spindalis::integrals::{
    analytical_integral, definite_integral, indefinite_integral, romberg_definite,
};
use spindalis::polynomials::{eval_simple_polynomial, parse_simple_polynomial};

fn main() {
    // Calculate the definite integral from a to b
    // This method uses Simpon's 1/3 and 3/8 rules for multiple segments
    // and the trapezoid method for a single segment
    let parsed = parse_simple_polynomial!(64x ^ 3 - 144x ^ 2 + 108x - 27);
    let result = definite_integral(&parsed, eval_simple_polynomial, -3.0, 5.0, 5);
    println!("The definite integral of 64x^3 - 144x^2 + 108x - 27 is {result}");

    // Calculate the definite integral from a to b with the romberg method
    let parsed = parse_simple_polynomial!(3x ^ 2);
    let result = romberg_definite(&parsed, eval_simple_polynomial, 0.0, 1.0, 8, 1e-6).unwrap();
    println!("The romberg definite integral of '3x^2' is {result}");

    // Calculate the definite integral from a to b using the analytical method
    // This method is ∫ₐᵇ x dx = F(b) − F(a)
    // where F(a) is the indefinite integral evaluated at point a
    // and F(b) is the indefinite integral evaluated at point b
    let poly = parse_simple_polynomial!(3x ^ 2 - 1);
    let result = analytical_integral(poly, 1.0, 5.0);
    println!("The analytical integral of '3x^2 - 1' is {result}");

    // Calculate the indefinite integral of a parsed simple polynomial
    let parsed = parse_simple_polynomial!(x ^ 3 - x);
    let result = indefinite_integral(&parsed);
    let eval = eval_simple_polynomial(2.0, &result);
    println!("The indefinite integral for 'x^3 - x' {parsed:?} is '1/4*x^4 - 1/2' {result:?}.");
    println!("The value of the integral evaluated at 2 is {eval}");
}
