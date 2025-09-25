pub mod approximators;
pub mod reduction;
pub mod regressors;

pub use approximators::bisection::bisection;
pub use approximators::nrm::newton_raphson_method;
pub use reduction::linear::pca;
pub use regressors::linear::linear_regression;

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
