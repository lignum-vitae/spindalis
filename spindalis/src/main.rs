use spindalis::derivatives::partial::partial_derivative;
use spindalis::polynomials::{ascii_letters, parse_polynomial_extended};
use spindalis::solvers::{SolveMode, bisection, newton_raphson_method};
use spindalis::utils::arr2D::Arr2D;
use spindalis::{derivative, eval_simple_polynomial, parse_simple_polynomial};

fn main() {
    let polynomial = "-2x^6 - 1.6x^4 + 12x + 1";
    let parsed = parse_simple_polynomial(polynomial).unwrap();

    let res = bisection(polynomial, 0.0, 1.0, 5.0, 1000, 0.6, SolveMode::Extrema);

    match res {
        Ok(x) => {
            println!(
                "Approximate maximum coords: ({x}, {:.5})",
                eval_simple_polynomial(x, &parsed)
            );

            println!(
                "True maximum coords: (0.90449, {:.5})",
                eval_simple_polynomial(0.90449, &parsed)
            );
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }

    let parsed = parse_simple_polynomial("5x^3 + 4x^4 - 5x^2 +1").unwrap();
    println!("{parsed:?}");
    let eval = eval_simple_polynomial(2.0, &parsed);
    println!("{eval:?}");
    let dx = derivative(&parsed);
    println!("{dx:?}");

    let guesses = [0.0, 1.0, 2.0];
    let polynomial = "0.5x^3-3.9x^2+6x-1.5";
    let parsed = parse_simple_polynomial(polynomial).unwrap();

    for guess in guesses {
        let res = newton_raphson_method(polynomial, guess, 100, 0.01, SolveMode::Root);
        match res {
            Ok(x) => println!(
                "Starting at {guess}, root found: ({x:.5}, {:.5})",
                eval_simple_polynomial(x, &parsed).abs()
            ),
            Err(e) => {
                println!("Error: {e:?}");
            }
        }
    }

    let guesses = [0.0, 5.0];
    for guess in guesses {
        let res = newton_raphson_method(polynomial, guess, 100, 0.01, SolveMode::Extrema);
        match res {
            Ok(x) => println!(
                "Starting at {guess}, extrema found: ({x:.5}, {:.5})",
                eval_simple_polynomial(x, &parsed)
            ),
            Err(e) => {
                println!("Error: {e:?}");
            }
        }
    }

    let letters = ascii_letters();
    let polynomial = parse_polynomial_extended("3x^2y + 3x - 4y + 3", &letters).unwrap();
    println!("{polynomial:?}");
    let derived_x = partial_derivative(&polynomial, "x");
    let derived_y = partial_derivative(&polynomial, "y");
    println!("{derived_x:?}");
    println!("{derived_y:?}");

    let result = newton_raphson_method("x^2 - 4", 1.0, 1000, 0.0001, SolveMode::Root);
    match result {
        Ok(x) => println!("{x}"),
        Err(e) => println!("Error: {e:?}"),
    }

    let result = newton_raphson_method("x^2 + 4", 1.0, 1000, 0.0001, SolveMode::Root);
    match result {
        Ok(x) => println!("{x}"),
        Err(e) => println!("Error: {e:?}"),
    }

    let arr1 = Arr2D::from_flat(vec![1, 2, 3, 4, 5, 6], 0, 2, 3).unwrap();
    let arr2 = Arr2D::from_flat(vec![7, 8, 9, 10, 11, 12], 0, 3, 2).unwrap();
    println!("{arr1}");
    println!("{arr2}");

    let res = arr1.dot(&arr2).unwrap();
    println!("{res}");

    let this = parse_polynomial_extended("5x^-0.5", &letters);
    println!("{this:?}");
}
