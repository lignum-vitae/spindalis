pub mod simple_definite;
pub mod simple_indefinite;

#[derive(Debug)]
pub enum IntegralError {
    MaxIterationsReached,
}
