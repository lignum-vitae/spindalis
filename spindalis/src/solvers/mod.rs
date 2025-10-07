pub mod bisection;
pub mod nrm;

pub use bisection::bisection;
pub use nrm::newton_raphson_method;

pub enum SolveMode {
    Root,
    Extrema,
}
