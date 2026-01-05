pub mod extended;
pub mod simple;

pub use extended::PolynomialExtended;
pub use simple::SimplePolynomial;

pub use crate::polynomials::PolynomialError; // import Polynomial Error

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
}
