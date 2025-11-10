

## Polynomials

### Parse and evaluate Simple Polynomials

Parse a univariate polynomial string with positive integer exponents
and evaluate it at a given point:

Vectors for parsed polynomials and derivatives are organised from $x^0$
to the highest power of x present in the polynomial.

The value of each position is the coefficient for the polynomial
raised to the index of the respective position.

This function can handle addition and subtraction.

[1.0, 0.0, -5.0, 5.0, 4.0] -> $1x^0+0x^1-5x^2+5x^3+4x^4$

### Parse and evaluate Polynomials Extended

The basic functionality of the simple polynomial extended.
This extended function can additionally handle fractional exponents, decimal
exponents, multivariate polynomials, and negative exponents.

Instead of using a vector of coefficients, each element of the polynomial is a `Term`

```rust
pub struct Term {
    pub coefficient: f64,              // Coefficient value
    pub variables: Vec<(String, f64)>, // Variable name and exponent
}
```

### Find Derivates

- Derivatives
    - Simple Derivative
    - Partial Derivative (for use with polynomial extended)

### Find Integrals

- Definite Integrals
    - Simple Definite Integral
    - Romberg integration
    - Analytical method

- Indefinite Integrals
    - Simple Indefinite Integral

## Math

### Linear Regression

- Gradient Descent Regression
- Least Squares Regression
- Polynomial Regression

### System of Linear Equations

- Gaussian Elimination
    - This function accepts any coefficient matrix that can be coerced
into a `Arr2D<f64>` type. That includes nested vecs of ints or floats,
nested arrays of ints or floats, and Arr2D vectors of types other than
`f64`. The right hand side vector also accepts a vector containing
any numerical values that can be converted into `f64`.

### Root and Extrema Finders

- Bisection method
- Newton-Raphson method