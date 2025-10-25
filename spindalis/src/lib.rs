pub mod polynomials;
pub mod reduction;
pub mod regressors;
pub mod solvers;
pub mod utils;

pub use polynomials::core::simple::eval_simple_polynomial;
pub use polynomials::core::simple::parse_simple_polynomial;
pub use polynomials::derivatives::derivative::derivative;

pub mod derivatives {
    pub use crate::polynomials::derivatives::partial;
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
