use spindalis::derivatives::simple_derivative;
use spindalis::polynomials::{eval_simple_polynomial, parse_simple_polynomial};
use spindalis::solvers::{SolveMode, newton_raphson_method};

fn main() {
    let polynomial = "0.5x^3 - 3.9x^2 + 6x - 1.5";
    let guesses = [0.0, 1.0, 2.0];
    let parsed = parse_simple_polynomial(polynomial).unwrap();

    println!("The polynomial being evaluated is {polynomial}");
    for guess in guesses {
        let result = newton_raphson_method(
            &parsed,
            simple_derivative,
            eval_simple_polynomial,
            guess,
            1000,
            1e-5,
            SolveMode::Root,
        );
        match result {
            Ok(x) => println!(
                "Starting at {guess}, root found: ({x:.5}, {:.5})",
                eval_simple_polynomial(x, &parsed).abs()
            ),
            Err(e) => eprintln!("{e:?}"),
        }
    }

    let guesses = [0.0, 5.0];
    for guess in guesses {
        let result = newton_raphson_method(
            &parsed,
            simple_derivative,
            eval_simple_polynomial,
            guess,
            10000,
            1e-5,
            SolveMode::Extrema,
        );
        match result {
            Ok(x) => println!(
                "Starting at {guess}, extrema found: ({x:.5}, {:.5})",
                eval_simple_polynomial(x, &parsed)
            ),
            Err(e) => eprintln!("{e:?}"),
        }
    }
}
