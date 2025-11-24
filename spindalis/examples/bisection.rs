use spindalis::polynomials::{PolynomialTraits, SimplePolynomial};
use spindalis::solvers::{Bounds, SolveMode, bisection};

fn main() {
    let polynomial = "-2x^6 - 1.6x^4 + 12x + 1";
    let parsed = SimplePolynomial::parse(polynomial).unwrap();
    let error_tol = 1e-5;
    let itermax = 10000;
    println!("The polynomial being evaluated is {polynomial}");

    let result = bisection(
        &parsed,
        Bounds {
            lower: 0.0,
            init: 0.6,
            upper: 1.0,
        },
        error_tol,
        itermax,
        SolveMode::Extrema,
    );

    match result {
        Ok(x) => {
            println!(
                "Approximate maximum coords between x=0 and x=1: ({x}, {:.5})",
                &parsed.eval_univariate(x).unwrap()
            );

            println!(
                "True maximum coords within that range: (0.90449, {:.5})\n",
                &parsed.eval_univariate(0.90449).unwrap()
            );
        }
        Err(e) => eprintln!("{e:?}"),
    }

    let params = [
        (
            Bounds {
                lower: -0.2,
                init: -0.05,
                upper: 0.0,
            },
            -0.08333,
        ),
        (
            Bounds {
                lower: 0.0,
                init: 0.6,
                upper: 2.0,
            },
            1.34612,
        ),
    ];
    for (bound, true_root) in params {
        let result = bisection(&parsed, bound, error_tol, itermax, SolveMode::Root);

        match result {
            Ok(x) => {
                println!(
                    "Approximate root coords between x=-0.2 and x=0: ({x}, {:.5})",
                    &parsed.eval_univariate(x).unwrap()
                );

                println!(
                    "True root coords within that range: ({true_root}, {:.3})\n",
                    &parsed.eval_univariate(true_root).unwrap()
                );
            }
            Err(e) => eprintln!("{e:?}\n"),
        }
    }
}
