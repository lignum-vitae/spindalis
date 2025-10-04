use spindalis::solvers::{SolveMode, bisection, newton_raphson_method};
use spindalis::{derivative, eval_polynomial, parse_polynomial};

fn main() {
    let polynomial = "-2x^6 - 1.6x^4 + 12x + 1";
    let parsed = parse_polynomial(&polynomial);

    let res = bisection(polynomial, 0.0, 1.0, 5.0, 1000, 0.6, SolveMode::Extrema);

    match res {
        Some(x) => {
            println!(
                "Approximate maximum coords: ({x}, {:.5})",
                eval_polynomial(x, &parsed)
            );

            println!(
                "True maximum coords: (0.90449, {:.5})",
                eval_polynomial(0.90449, &parsed)
            );
        }
        None => println!("No extrema was found within the given iterations"),
    }

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
