# Spindalis

A bioinformatics library for numerical modeling, optimisation, data analysis,
and simulation written in Rust.

Spindalis provides a collection of numerical methods, polynomial parsing
and evaluation tools, derivative computation, and optimisation algorithms for
scientific computing and bioinformatics applications.

## Features

- Polynomial parsing and evaluation
- Derivative computation
- Root finding with Bisection and Newton–Raphson methods
- Extensible modules for numerical modelling and optimisation

## Installation

Add spindalis as a dependency in your Cargo.toml:

```toml
[dependencies]
spindalis = { git = "https://github.com/lignum-vitae/spindalis.git" }
```

Then run:

`cargo build`

## Usage

### Derivative

Compute the derivative of a polynomial:

```rust
use spindalis::derivative;
use spindalis::parse_polynomial;

let polynomial = "5x^3 + 4x^4 - 5x^2 + 1";
let parsed = parse_polynomial(polynomial);
let dx = derivative(&parsed);

println!("Parsed polynomial: {:?}", parsed)
println!("Derivative coefficients: {:?}", dx);

# Parsed polynomial: [1.0, 0.0, -5.0, 5.0, 4.0]
# Derivative coefficients: [0.0, -10.0, 15.0, 16.0]
```

### eval_polynomial and parse_polynomial

Parse a polynomial string and evaluate it at a given point:

```rust
use spindalis::{eval_polynomial, parse_polynomial};

let polynomial = "5x^3 + 4x^4 - 5x^2 + 1";
let parsed = parse_polynomial(polynomial);

println!("Parsed polynomial: {:?}", parsed);

let value = eval_polynomial(2.0, &parsed);
println!("Polynomial evaluated at x=2: {:?}", value);

# Polynomial evaluated at x=2: 85.0
```

### Bisection

Find a root or extremum of a polynomial using the bisection method:

```rust
use spindalis::solvers::bisection;
use spindalis::{eval_polynomial, parse_polynomial};

let polynomial = "-2x^6 - 1.6x^4 + 12x + 1";
let parsed = parse_polynomial(polynomial);

let res = bisection(polynomial, 0.0, 1.0, 5.0, 1000, 0.6);

println!(
    "Approximate maximum coords: ({res}, {:.5})",
    eval_polynomial(res, &parsed)
);

println!(
    "True maximum coords: (0.90449, {:.5})",
    eval_polynomial(0.90449, &parsed)
);

# Approximate maximum coords: (0.90625, 9.68783)
# True maximum coords: (0.90449, 9.68792)
```

### Newton–Raphson Method

Find roots of a polynomial starting from initial guesses:

```rust
use spindalis::solvers::newton_raphson_method;
use spindalis::{eval_polynomial, parse_polynomial};

let polynomial = "0.5x^3 - 3.9x^2 + 6x - 1.5";
let parsed = parse_polynomial(polynomial);
let guesses = [0.0, 1.0, 2.0];

for guess in guesses {
    let res = newton_raphson_method(polynomial, guess, 100, 0.01);
    match res {
        Some(x) => println!(
            "Starting at {guess}, root found: ({x:.5}, {:.5})",
            eval_polynomial(x, &parsed).abs()
        ),
        None => println!("Starting at {guess}, no root was found within the given iterations"),
    }
}

# Starting at 0, root found: (0.30997, 0.00000)
# Starting at 1, root found: (5.82992, 0.00000)
# Starting at 2, root found: (1.66011, 0.00000)

```
