use spindalis::polynomials::{PolynomialTraits, SimplePolynomial};
use spindalis::solvers::{SolveMode, newton_raphson_method};

fn main() {
    let polynomial = "0.5x^3 - 3.9x^2 + 6x - 1.5";
    let guesses = [0.0, 1.0, 2.0];
    let parsed = SimplePolynomial::parse(polynomial).unwrap();

    println!("The polynomial being evaluated is {polynomial}");
    for guess in guesses {
        let result = newton_raphson_method(&parsed, guess, 1000, 1e-5, SolveMode::Root);
        match result {
            Ok(x) => println!(
                "Starting at {guess}, root found: ({x:.5}, {:.5})",
                &parsed.eval_univariate(x).unwrap().abs()
            ),
            Err(e) => eprintln!("{e:?}"),
        }
    }

    let guesses = [0.0, 5.0];
    for guess in guesses {
        let result = newton_raphson_method(&parsed, guess, 10000, 1e-5, SolveMode::Extrema);
        match result {
            Ok(x) => println!(
                "Starting at {guess}, extrema found: ({x:.5}, {:.5})",
                &parsed.eval_univariate(x).unwrap()
            ),
            Err(e) => eprintln!("{e:?}"),
        }
    }
}
