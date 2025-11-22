pub mod linear;

pub use linear::gradient_descent::GradientDescentRegression;
pub use linear::least_squares::LeastSquaresRegression;
pub use linear::polynomial::PolynomialRegression;
pub use linear::{LinearModel, LinearRegressor, LinearRegressorError};
