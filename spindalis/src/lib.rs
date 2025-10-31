pub mod polynomials;
pub mod reduction;
pub mod regressors;
pub mod solvers;
pub mod utils;

pub mod derivatives {
    pub use crate::polynomials::derivatives::extended::partial_derivative;
    pub use crate::polynomials::derivatives::simple::simple_derivative;
}

/*
pub use regressors::linear::linear_regression;
*/

/*
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
