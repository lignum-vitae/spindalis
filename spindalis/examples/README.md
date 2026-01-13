# Spindalis Examples

## Polynomials

### PolynomialTraits

The two polynomial structs (simple, and intermediate) implement the `PolynomialTraits`
trait.

```rust
pub trait PolynomialTraits {
    fn parse(input: &str) -> Result<Self, PolynomialError>
    where
        Self: std::marker::Sized;
    fn eval_univariate<F>(&self, point: F) -> Result<f64, PolynomialError>
    where
        F: Into<f64> + std::clone::Clone + std::fmt::Debug;
    fn eval_multivariate<V, S, F>(&self, vars: &V) -> Result<f64, PolynomialError>
    where
        V: IntoIterator<Item = (S, F)> + std::fmt::Debug + Clone,
        S: AsRef<str>,
        F: Into<f64>;
    fn derivate_univariate(&self) -> Result<Self, PolynomialError>
    where
        Self: std::marker::Sized;
    fn derivate_multivariate<S>(&self, var: S) -> Self
    where
        S: AsRef<str>;
    fn indefinite_integral_univariate(&self) -> Result<Self, PolynomialError>
    where
        Self: std::marker::Sized;
    fn indefinite_integral_multivariate<S>(&self, var: S) -> Self
    where
        S: AsRef<str>;
}
```

The Polynomial struct will have this trait implemented in the future.

### Parse and evaluate Simple Polynomials

```rust
#[derive(Debug, PartialEq)]
pub struct SimplePolynomial {
    pub coefficients: Vec<f64>,
}
```

Parse a univariate polynomial string with positive integer exponents
and evaluate it at a given point:

Vectors for parsed polynomials and derivatives are organised from $x^0$
to the highest power of x present in the polynomial.

The value of each position is the coefficient for the polynomial
raised to the index of the respective position.

This function can handle addition and subtraction.

[1.0, 0.0, -5.0, 5.0, 4.0] -> $1x^0+0x^1-5x^2+5x^3+4x^4$

### Parse and evaluate Intermediate Polynomials

```rust
#[derive(Debug, PartialEq)]
pub struct IntermediatePolynomial {
    pub terms: Vec<Term>,
    pub variables: Vec<String>,
}
```

The basic functionality of the simple polynomial extended.
This intermediate function can additionally handle fractional exponents, decimal
exponents, multivariate polynomials, and negative exponents.

Instead of using a vector of coefficients, each element of the polynomial is a `Term`

```rust
pub struct Term {
    pub coefficient: f64,              // Coefficient value
    pub variables: Vec<(String, f64)>, // Variable name and exponent
}
```

### Parse polynomials with an AST

```rust
#[derive(Debug, PartialEq)]
pub struct Polynomial {
    expr: Expr,
}
```

This method is slowest to parse, but is intended to parse, evaluate, integrate, and
derivate any polynomial given to it. The `Polynomial` struct implements Pratt
parsing to parse polynomials into an abstract syntax tree.

### Find Derivates

- Derivatives
  - Simple Derivative
  - Partial Derivative (for use with intermediate polynomial)

### Find Integrals

- Definite Integrals
  - Univariate Definite Integral
  - Romberg integration
  - Analytical method

- Indefinite Integrals
  - Simple Indefinite Integral
  - Intermediate Indefinite Integral

## Math

Where applicable, functions accept any coefficient matrix that can be coerced
into a `Arr2D<f64>` type. That includes nested vecs of ints or floats,
nested arrays of ints or floats, and Arr2D vectors of types other than
`f64`. The matrix must be borrowed for conversion to work.
The right hand side vector also accepts a vector containing
any numerical values that can be converted into `f64`.

### Linear Regression

Ensure that input vectors are not empty and are of the same length to avoid errors.

- Gradient Descent Regression
  - Finds the line of best fit by iteratively adjusting the model's parameters
  (coefficients) to minimize the cost function, often the mean squared error.
- Least Squares Regression
  - Analytically determines the line of best fit by directly minimizing the
  sum of the squares of the vertical distances (residuals) from the data points
  to the line.
- Polynomial Regression
  - Models the relationship between the independent variable and the dependent
  variable as an n-th degree polynomial to fit non-linear data patterns.

### System of Linear Equations

- Gaussian Elimination
  - A direct method for solving a system of linear equations ($\mathbf{Ax} = \mathbf{b}$)
  by performing a series of row operations to transform the augmented matrix into
  an upper triangular matrix. This allows the solution to be found easily using
  back substitution.
- LU Decomposition (Lower-Upper)
  - A factorization method to decompose a matrix into a lower triangular matrix
  and an upper triangular matrix. This significantly speeds up solving
  $\mathbf{Ax} = \mathbf{b}$ for multiple right-hand side vectors $\mathbf{b}$.

- PLU Decomposition (Permutation, Lower-Upper)
  - An extension of LU decomposition that includes a permutation matrix to keep
  track of pivoting to handle singular matrices and improve numerical stability.
  The factorization is $\mathbf{PA} = \mathbf{LU}$.

### Eigenvalue Problems

- Power Method (Eigenvalue and Associated Eigenvector)
  - An iterative algorithm used to find the largest eigenvalue (the dominant eigenvalue)
  of a given matrix and its corresponding eigenvector of a matrix. The smallest
  eigenvalue can be found by performing the algorithm on the inverse of the matrix,
  with the smallest eigenvalue being the reciprocal of the result.

### Root and Extrema Finders

- Bisection method
  - A bracketed root-finding method that repeatedly bisects an interval and
  selects the subinterval where the function changes sign, ensuring convergence
  to a root.
- Newton-Raphson method
  - An open root-finding method that uses a polynomial and its derivative
  at an initial guess to iteratively find better approximations of a root.
