pub mod simple_definite;
pub mod simple_indefinite;

pub use simple_definite::analytical_integral;
pub use simple_definite::definite_integral;
pub use simple_definite::romberg_definite;
pub use simple_indefinite::indefinite_integral;

#[derive(Debug)]
pub enum IntegralError {
    MaxIterationsReached,
}
