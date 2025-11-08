# Spindalis

A bioinformatics library for numerical modeling, optimisation, data analysis,
and simulation written in Rust.

Spindalis provides a collection of numerical methods, polynomial parsing
and evaluation tools, derivative computation, and optimisation algorithms for
scientific computing and bioinformatics applications.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Polynomials](#polynomials)
  - [Math](#math)
- [Contributing](#contributing)
- [Stability](#stability)
- [License](#license)

## Features

- Polynomial parsing and evaluation
- Derivative computation
- Root and Extrema finding with Bisection and Newton–Raphson methods
- Extensible modules for numerical modelling and optimisation

## Installation

Add Spindalis as a dependency in your Cargo.toml:

```toml
[dependencies]
spindalis = { git = "https://github.com/lignum-vitae/spindalis.git" }
```

Then run:

`cargo build`

## Usage

### Polynomials

#### Parse and evaluate simple polynomials

Parse a univariate polynomial string with positive integer exponents
and evaluate it at a given point:

Vectors for parsed polynomials and derivatives are organised from $x^0$
to the highest power of x present in the polynomial.

The value of each position is the coefficient for the polynomial
raised to the index of the respective position.

This function can handle addition and subtraction.

[1.0, 0.0, -5.0, 5.0, 4.0] -> $1x^0+0x^1-5x^2+5x^3+4x^4$

```rust
use spindalis::polynomials::{eval_simple_polynomial, parse_simple_polynomial};

// Parsing function
let polynomial = "5x^3 + 4x^4 - 5x^2 + 1";
let parsed = parse_simple_polynomial(polynomial).unwrap();

// Parsing macro
let parsed = parse_simple_polynomial!(5x^3 + 4x^4 - 5x^2 + 1);

println!("Parsed polynomial: {parsed:?}");

let value = eval_simple_polynomial(2.0, &parsed);
println!("Polynomial evaluated at x=2: {:?}", value);

// Parsed polynomial: [1.0, 0.0, -5.0, 5.0, 4.0]
// Polynomial evaluated at x=2: 85.0
```

There is another function that extends the simple polynomial.
This extended function can additionally handle fractional exponents, decimal
exponents, multivariate polynomials, and negative exponents.

```rust
use spindalis::polynomials::{eval_polynomial_extended, parse_polynomial_extended};

// Parsing function
let polynomial = "4x^2y^3 + 4x - 2y + z^1.0/2.0";
let parsed = parse_polynomial_extended(polynomial).unwrap();

// Parsing macro
let parsed = parse_polynomial_extended!(4x^2y^3 + 4x - 2y + z^1.0/2.0);

println!("Parsed polynomial: {parsed:?}");

let mut vars = HashMap::new();
vars.insert("x".to_string(), 2.0);
vars.insert("y".to_string(), 1.0);
vars.insert("z".to_string(), 8.0);

let value = eval_polynomial_extended(&parsed, &vars);
println!("Polynomial evaluated at x=2, y=1, z=4: {:?}", value);

/*
Parsed Polynomial:
 vec![
    Term {
        coefficient: 4.0,
        variables: vec![("x".to_string(), 2.0), ("y".to_string(), 3.0)],
    }, // 4x^2y^3
    Term {
        coefficient: 4.0,
        variables: vec![("x".to_string(), 1.0)],
    }, // 4x
    Term {
        coefficient: 2.0,
        variables: vec![("y".to_string(), 1.0)],
    }, // 2y
    Term {
        coefficient: 1.0,
        variables: vec![("z".to_string(), 0.5)],
    }, // z^1.0/2.0
];

Polynomial evaluated at x=2, y=1, z=4: 24
*/
```

#### Find Derivates

Compute the derivative of a simple polynomial:

```rust
use spindalis::derivatives::simple_derivative;
use spindalis::polynomials::parse_simple_polynomial;

let polynomial = "5x^3 + 4x^4 - 5x^2 + 1";
let parsed = parse_simple_polynomial(polynomial).unwrap();
let dx = simple_derivative(&parsed);

println!("Derivative coefficients: {:?}", dx);

// Derivative coefficients: [0.0, -10.0, 15.0, 16.0]
```

The extended polynomials can be derived using the partial derivative from the
extended file.

```rust
use spindalis::derivatives::partial_derivative;
use spindalis::polynomials::parse_polynomial_extended;

let polynomial = "4x^2y^3 + 4x - 2y + z^1.0/2.0";
let parsed = parse_simple_polynomial(polynomial).unwrap();
let dx = partial_derivative(&parsed, "x")

println!("Partial derivative of x: {dx:?}");

/*
Partial derivative of x:
 vec![
    Term {
        coefficient: 8.0,
        variables: vec![("x".to_string(), 1.0), ("y".to_string(), 3.0)],
    }, // 8xy^3
    Term {
        coefficient: 4.0,
        variables: vec![],
    }, // 4
];
*/
```

#### Find Integrals

Compute the definite integral of a simple polynomial

```rust
use spindalis::polynomials::parse_simple_polynomial;
use spindalis::integrals::definite_integral;

let parsed = parse_simple_polynomial!(64x ^ 3 - 144x ^ 2 + 108x - 27);
let result = definite_integral(&parsed, eval_simple_polynomial, -3.0, 5.0, 5);

println!("The definite integral of 64x^3 - 144x^2 + 108x - 27 is {result}");
// The definite integral of 64x^3 - 144x^2 + 108x - 27 is 2056
```

Use Romberg integration to compute the definite integral

```rust
use spindalis::polynomials::parse_simple_polynomial;
use spindalis::integrals::romberg_definite;

let parsed = parse_simple_polynomial!(3x ^ 2);
let result = romberg_definite(&parsed, eval_simple_polynomial, 0.0, 1.0, 8, 1e-6).unwrap();

println!("The romberg definite integral is {result}");
// The romberg definite integral is 1
```

Use the Analytical method to compute the definite integral

```rust
use spindalis::polynomials::parse_simple_polynomial;
use spindalis::integrals::analytical_integral;
let poly = parse_simple_polynomial!(3x ^ 2 - 1);
let result = analytical_integral(poly, 1.0, 5.0);

println!("The result of the analytical integral is {result}");
// The result of the analytical integral is 120

```

Compute the indefinite integral of a simple polynomial

```rust
use spindalis::integrals::indefinite_integral;
use spindalis::polynomials::{parse_simple_polynomial, eval_simple_polynomial};

let parsed = parse_simple_polynomial!(x ^ 3 - x);
let result = indefinite_integral(&parsed);
let eval = eval_simple_polynomial(2.0, &result);

println!("The indefinite integral for {parsed:?} is {result:?}.");
println!("The value for the integral evaluated at 2 is {eval}");
// The indefinite integral for [0.0, 1.0, 0.0, 1.0] is [0.0, 0.0, -0.5, 0.0, 0.25].
// The value for the integral evaluated at 2 is 2.0.
```

### Math

#### Linear Regression

Gradient Descent Regression

```rust
use spindalis::regressors::{GradientDescentRegression, LinearModel};

let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
let y: Vec<f64> = vec![1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 8.0, 10.0, 13.0];

let grad_descent = GradientDescentRegression{
    steps: 10000,
    step_size: 0.01,
};

let model = grad_descent.fit(&x, &y);

println!("Slope = {:.2}\nIntercept = {:.2}", model.slope().unwrap(), model.intercept());
// Slope = 1.46
// Intercept = -2.01
println!("Standard Error = {:.3}\nR2 Score = {:.3}", model.std_err, model.r2);
// Standard Error = 1.307
// R2 Score = 0.914
println!("{}", model.to_polynomial_string());
// -2.01389 + 1.45833x
```

Least Squares Regression

```rust
use spindalis::regressors::{LeastSquaresRegression, LinearModel};

let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
let y: Vec<f64> = vec![1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 8.0, 10.0, 13.0];

let least_squares = LeastSquaresRegression;

let model = least_squares.fit(&x, &y);

println!("Slope = {:.2}\nIntercept = {:.2}", model.slope().unwrap(), model.intercept());
// Slope = 1.46
// Intercept = -2.01
println!("Standard Error = {:.3}\nR2 Score = {:.3}", model.std_err, model.r2);
// Standard Error = 1.307
// R2 Score = 0.914
println!("{}", model.to_polynomial_string());
// -2.01389 + 1.45833x
```

Polynomial Regression

```rust
use spindalis::regressors::{PolynomialRegression, LinearModel};

let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
let y: Vec<f64> = vec![1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 8.0, 10.0, 13.0];

let poly_regression = PolynomialRegression { order: 2 };

let model = poly_regression.fit(&x, &y);

println!("Slopes = {:?}\nIntercept = {:.2}", model.slopes().unwrap(), model.intercept());
// Slopes = [-0.4518398268398198, 0.19101731601731534]
// Intercept = 1.49
println!("Standard Error = {:.3}\nR2 Score = {:.3}", model.std_err, model.r2);
// Standard Error = 0.319
// R2 Score = 0.995
println!("{}", model.to_polynomial_string());
// 1.48810 - 0.45184x + 0.19102x^2
```

#### System of Linear Equations

Gaussian Elimination

This function accepts any coefficient matrix that can be coerced
into a `Arr2D<f64>` type. That includes nested vecs of ints or floats,
nested arrays of ints or floats, and Arr2D vectors of types other than
`f64`. The right hand side vector also accepts a vector containing
any numerical values that can be converted into `f64`.

```rust
let coeff_matrix = vec![
    vec![8.0, 2.0, -2.0],
    vec![10.0, 2.0, 4.0],
    vec![12.0, 2.0, 2.0],
];

let mut rhs_vector = vec![8.0, 16.0, 16.0];
let tol = 1e-12;
let expected: Vec<f64> = vec![1.0; 3];

let solution = gaussian_elimination(&coeff_matrix, &mut rhs_vector, tol).unwrap();
println!("Solution:");
for (i, sol) in solution.iter().enumerate() {
    print!("x{} = {sol}", i + 1);
    if i != solution.len() - 1 {
        print!(", ")
    }
}
// Solution:
// x1 = 1, x2 = 1, x3 = 1
```

#### Bisection

Locate a root or extremum of a polynomial via the bisection method:

```rust
use spindalis::solvers::{SolveMode, Bounds, bisection};
use crate::derivatives::simple_derivative;
use crate::polynomials::{eval_simple_polynomial, parse_simple_polynomial};

let polynomial = "-2x^6 - 1.6x^4 + 12x + 1";
let parsed = parse_simple_polynomial(&polynomial);

let result = bisection(
    &parsed,
    simple_derivative,
    eval_simple_polynomial,
    Bounds {
        lower: 0.0,
        init: 0.6,
        upper: 1.0,
    },
    1e-5,
    10000,
    SolveMode::Extrema,
);

match result {
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

// Approximate maximum coords: (0.90625, 9.68783)
// True maximum coords: (0.90449, 9.68792)

let result = bisection(
    &parsed,
    simple_derivative,
    eval_simple_polynomial,
    Bounds {
        lower: -0.2,
        init: -0.05,
        upper: 0.0,
    },
    1e-5,
    10000,
    SolveMode::Root,
);

match result {
    Some(x) => {
        println!(
            "Approximate root coords: ({x}, {:.5})",
            eval_polynomial(x, &parsed)
        );

        println!(
            "True root coords: (-0.08333, {:.5})",
            eval_polynomial(-0.08333, &parsed)
        );
    },
    None => println!("No extrema was found within the given iterations"),
}

// Approximate root coords: (-0.1, -0.20016)
// True root coords: (-0.08333, 0.00026)

let result = bisection(
    &parsed,
    simple_derivative,
    eval_simple_polynomial,
    Bounds {
        lower: 0.0,
        init: 0.6,
        upper: 2.0,
    },
    1e-5,
    10000,
    SolveMode::Root,
);

match result {
    Some(x) => {
        println!(
            "Approximate root coords: ({x}, {:.5})",
            eval_polynomial(x, &parsed)
        );

        println!(
            "True root coords: (1.34612, {:.5})",
            eval_polynomial(1.34612, &parsed)
        );
    },
    None => println!("No extrema was found within the given iterations"),
}

// Approximate root coords: (1.3125, 1.77781)
// True root coords: (1.34612, 0.00026)
```

#### Newton–Raphson Method

Locate a root or extremum of a polynomial via the Newton-Raphson method:

```rust
use spindalis::solvers::{SolveMode, newton_raphson_method};
use spindalis::derivatives::simple_derivative;
use spindalis::polynomials::{eval_simple_polynomial, parse_simple_polynomial};

let polynomial = "0.5x^3 - 3.9x^2 + 6x - 1.5";
let guesses = [0.0, 1.0, 2.0];
let parsed = parse_polynomial(&polynomial);

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
        Some(x) => println!(
            "Starting at {guess}, root found: ({x:.5}, {:.5})",
            eval_polynomial(x, &parsed).abs()
        ),
        None => println!("Starting at {guess}, no root was found within the given iterations"),
    }
}

// Starting at 0, root found: (0.30997, 0.00000)
// Starting at 1, root found: (5.82992, 0.00000)
// Starting at 2, root found: (1.66011, 0.00000)

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
        Some(x) => println!(
            "Starting at {guess}, extrema found: ({x:.5}, {:.5})",
            eval_polynomial(x, &parsed)
        ),
        None => {
            println!("Starting at {guess}, no extrema was found within the given iterations")
        }
    }
}

// Starting at 0, extrema found: (0.93868, 1.10926)
// Starting at 5, extrema found: (4.26132, -8.06126)
```

## Contributing

We welcome contributions! Please read our:

- [Code of Conduct](https://github.com/lignum-vitae/spindalis/blob/main/docs/CODE_OF_CONDUCT.md)
- [Contribution Guidelines](https://github.com/lignum-vitae/spindalis/blob/main/docs/CONTRIBUTING.md)

## Stability

This project is in the alpha stage. APIs may change without warning until version
1.0.0.

## License

This project is licensed under the MIT License - see the
[LICENSE](https://github.com/lignum-vitae/spindalis/blob/main/LICENSE) file for details.
