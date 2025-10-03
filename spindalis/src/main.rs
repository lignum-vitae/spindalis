use spindalis::solvers::{SolveMode, newton_raphson_method};
use spindalis::{derivative, eval_polynomial, parse_polynomial};

fn main() {
    let parsed = parse_polynomial("5x^3 + 4x^4 - 5x^2 +1");
    println!("{:?}", parsed);
    let eval = eval_polynomial(2.0, &parsed);
    println!("{:?}", eval);
    let dx = derivative(&parsed);
    println!("{:?}", dx);

    let guesses = [0.0, 1.0, 2.0];
    let polynomial = "0.5x^3-3.9x^2+6x-1.5";
    let parsed = parse_polynomial(&polynomial);

    for guess in guesses {
        let res = newton_raphson_method(polynomial, guess, 100, 0.01, SolveMode::Root);
        match res {
            Some(x) => println!(
                "Starting at {guess}, root found: ({x:.5}, {:.5})",
                eval_polynomial(x, &parsed).abs()
            ),
            None => println!("Starting at {guess}, no root was found within the given iterations"),
        }
    }

    let guesses = [0.0, 5.0];
    for guess in guesses {
        let res = newton_raphson_method(polynomial, guess, 100, 0.01, SolveMode::Extrema);
        match res {
            Some(x) => println!(
                "Starting at {guess}, extrema found: ({x:.5}, {:.5})",
                eval_polynomial(x, &parsed)
            ),
            None => {
                println!("Starting at {guess}, no extrema was found within the given iterations")
            }
        }
    }
}
