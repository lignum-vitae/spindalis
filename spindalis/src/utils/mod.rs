pub mod variation;

pub use variation::arith_mean;
pub use variation::geom_mean;
pub use variation::std_dev;

#[derive(Copy, Clone)]
pub enum StdDevType {
    Poulation,
    Sample,
}
