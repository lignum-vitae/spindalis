# Spindalis

[![crates.io](https://img.shields.io/crates/v/spindalis.svg)](https://crates.io/crates/spindalis)
[![docs.rs](https://docs.rs/spindalis/badge.svg)](https://docs.rs/spindalis)
[![Build Status](https://github.com/lignum-vitae/spindalis/workflows/Build%20and%20test%20Rust/badge.svg)](https://github.com/lignum-vitae/spindalis/actions?workflow=Build%20and%20test%20Rust)
[![Build Status](https://github.com/lignum-vitae/spindalis/workflows/Clippy%20check%20-%20lint/badge.svg)](https://github.com/lignum-vitae/spindalis/actions?workflow=Clippy%20check%20-%20lint)

A bioinformatics library for numerical modeling, optimisation, data analysis,
and simulation written in Rust.

Spindalis provides a collection of numerical methods, polynomial parsing
and evaluation tools, derivative computation, and optimisation algorithms for
scientific computing and bioinformatics applications.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Project Layout](#project-layout)
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

### Via Crates.io (Recommended)

To use the latest stable release, add spindalis as a dependency:

```toml
[dependencies]
spindalis = "0.X.X" # Always use the latest version available on crates.io
```

Or, use Cargo:

```nginx
cargo add spindalis
```

### Via Git Repository (Latest Development Build)

```toml
[dependencies]
spindalis = { git = "https://github.com/lignum-vitae/spindalis.git" }
```

Then run:

`cargo build`

## Project layout

| Module                 | Description                                                                                            |
| ---------------------- | ------------------------------------------------------------------------------------------------------ |
| `utils`                | Utility functions such as `Arr2D`, `Arr2DError`, forward substitution, and back substitution           |
| `polynomials`          | Parsing and evaluating simple and extended polynomials                                                 |
| `derivatives`          | Differentiating simple and extended polynomials                                                        |
| `integrals`            | Integrating simple and extended polynomials                                                            |
| `solvers`              | Solving equations and differential equations, including root-finding, extrema-finding, and ODE solvers |
| `solvers/eigen`        | Algorithms to solve eigenvalue and eigenvector problems                                                 |
| `solvers/decomposition`| Decomposition algorithms including LU decomposition and LU decomposition with partial pivoting         |
| `regressors`           | Linear and non-linear regression, including least-squares, Gaussian, and polynomial regression         |
| `reduction`            | Linear and non-linear dimensionality reduction algorithms, including PCA                               |

### Running Examples

Working examples of the available algorithms as well as a full list of available algorithms can be found in the
[`examples/`](https://github.com/lignum-vitae/spindalis/tree/main/spindalis/examples) directory.

Run any example with the following command:

`cargo run --example <example_name>`

Do not include `.rs` when running examples.

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
