use spindalis::integrals::{analytical_integral, definite_integral, romberg_definite};
use spindalis::polynomials::{PolynomialExtended, PolynomialTraits, SimplePolynomial};

fn main() {
    // SIMPLE POLYNOMIALS
    println!("Simple Polynomial parsing:");
    // Calculate the definite integral from a to b
    // This method uses Simpon's 1/3 and 3/8 rules for multiple segments
    // and the trapezoid method for a single segment
    let parsed = SimplePolynomial::parse("64x ^ 3 - 144x ^ 2 + 108x - 27").unwrap();
    let result = definite_integral(&parsed, -3.0, 5.0, 5).unwrap();
    println!("The definite integral of 64x^3 - 144x^2 + 108x - 27 is {result:.2}");

    // Calculate the definite integral from a to b with the romberg method
    let parsed = SimplePolynomial::parse("3x ^ 2").unwrap();
    let result = romberg_definite(&parsed, 0.0, 1.0, 8, 1e-6).unwrap();
    println!("The romberg definite integral of '3x^2' from 0 to 1 is {result}");

    // Calculate the definite integral from a to b using the analytical method
    // This method is ∫ₐᵇ x dx = F(b) − F(a)
    // where F(a) is the indefinite integral evaluated at point a
    // and F(b) is the indefinite integral evaluated at point b
    let poly = SimplePolynomial::parse("3x ^ 2 - 1").unwrap();
    let result = analytical_integral(&poly, 1.0, 5.0).unwrap();
    println!("The analytical integral of '3x^2 - 1' from 1 to 5 is {result}");

    // Calculate the indefinite integral of a parsed simple polynomial
    let parsed = SimplePolynomial::parse("x ^ 3 - x").unwrap();
    let result = parsed.indefinite_integral_univariate().unwrap();
    let eval = result.eval_univariate(2.0).unwrap();
    println!("The indefinite integral for 'x^3 - x' [{parsed}] is '1/4*x^4 - 1/2x^2' [{result}].");
    println!("The value of the integral evaluated at 2 is {eval}\n");

    // Extended POLYNOMIALS
    println!("Extended Polynomial parsing:");
    let parsed = PolynomialExtended::parse("64x ^ 3 - 144x ^ 2 + 108x - 27").unwrap();
    let result = definite_integral(&parsed, -3.0, 5.0, 5).unwrap();
    println!("The definite integral of 64x^3 - 144x^2 + 108x - 27 is {result:.2}");

    let parsed = PolynomialExtended::parse("3x ^ 2").unwrap();
    let result = romberg_definite(&parsed, 0.0, 1.0, 8, 1e-6).unwrap();
    println!("The romberg definite integral of '3x^2' from 0 to 1 is {result}");

    let poly = PolynomialExtended::parse("3x ^ 2 - 1").unwrap();
    let result = analytical_integral(&poly, 1.0, 5.0).unwrap();
    println!("The analytical integral of '3x^2 - 1' from 1 to 5 is {result}");

    let parsed = PolynomialExtended::parse("x ^ 3 - x").unwrap();
    let result = parsed.indefinite_integral_univariate().unwrap();
    let eval = result.eval_univariate(2.0).unwrap();
    println!("The indefinite integral for 'x^3 - x' [{parsed}] is '1/4*x^4 - 1/2x^2' [{result}].");
    println!("The value of the integral evaluated at 2 is {eval}\n");
}
