pub mod derivatives;
pub mod polynomials;
pub mod reduction;
pub mod regressors;
pub mod solvers;

pub use derivatives::derivative::derivative;
pub use polynomials::polynomial::eval_polynomial;
pub use polynomials::polynomial::parse_polynomial;

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
