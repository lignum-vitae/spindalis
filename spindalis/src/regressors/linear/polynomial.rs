use crate::regressors::linear::{LinearModel, LinearRegressor};
use crate::solvers::gaussian_elimination;

pub struct PolynomialRegression {
    order: usize,
}

impl LinearRegressor for PolynomialRegression {
    #[allow(clippy::needless_range_loop)]
    fn fit(&self, x: &[f64], y: &[f64]) -> LinearModel {
        let order = self.order;

        let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; order + 1]; order + 1];
        let mut rhs: Vec<f64> = vec![0.0; order + 1];
        for i in 0..=order {
            for j in 0..=i {
                let k = i + j;
                let poly_sum = x.iter().map(|n| n.powi(k as i32)).sum::<f64>();
                matrix[i][j] = poly_sum;
                matrix[j][i] = poly_sum;
            }
            let poly_sum = y
                .iter()
                .zip(x.iter())
                .map(|(n, m)| n * m.powi(i as i32))
                .sum::<f64>();
            rhs[i] = poly_sum;
        }
        let coefficients = gaussian_elimination(&matrix, &rhs, 1e-5).unwrap();

        let length = y.len() as f64;

        let y_mean = y.iter().sum::<f64>() / length;
        let sq_total: f64 = y.iter().map(|y_i| (y_i - y_mean).powi(2)).sum();
        let sq_residual: f64 = x
            .iter()
            .zip(y.iter())
            .map(|(&x_i, &y_i)| {
                let y_pred: f64 = coefficients
                    .iter()
                    .enumerate()
                    .map(|(pow, &c)| c * x_i.powi(pow as i32))
                    .sum();

                (y_i - y_pred).powi(2)
            })
            .sum();

        let std_err = (sq_residual / (length - 2.0)).sqrt();
        let r2 = (sq_total - sq_residual) / sq_total;

        LinearModel {
            coefficients,
            std_err,
            r2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const ERROR_TOL: f64 = 1e-3;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    #[test]
    fn known_regression() {
        let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let y: Vec<f64> = vec![1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 8.0, 10.0, 13.0];

        let poly_regression = PolynomialRegression { order: 2 };
        let model = poly_regression.fit(&x, &y);

        let expected_slopes = vec![-0.4518, 0.1910];
        for (&res, &exp) in model.slopes().unwrap().iter().zip(&expected_slopes) {
            assert!(approx_eq(res, exp, ERROR_TOL))
        }
        assert!(approx_eq(model.intercept(), 1.48809, ERROR_TOL));
        assert!(approx_eq(model.std_err, 0.31919, ERROR_TOL));
        assert!(approx_eq(model.r2, 0.99488, ERROR_TOL));
        assert_eq!(
            model.to_polynomial_string(),
            "1.48810 - 0.45184x + 0.19102x^2"
        )
    }
}
