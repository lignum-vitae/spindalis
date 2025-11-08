use crate::regressors::linear::{LinearModel, LinearRegressor};

pub struct LeastSquaresRegression;

impl LinearRegressor for LeastSquaresRegression {
    fn fit(&self, x: &[f64], y: &[f64]) -> LinearModel {
        let mut coefficients = Vec::new();
        let length = x.len() as f64;
        let sumx = x.iter().sum::<f64>();
        let sumy = y.iter().sum::<f64>();
        let sumxy = x
            .iter()
            .zip(y.iter())
            .map(|(x_i, y_i)| x_i * y_i)
            .sum::<f64>();
        let sumx2 = x.iter().map(|x_i| x_i.powi(2)).sum::<f64>();
        let x_mean = sumx / length;
        let y_mean = sumy / length;
        let slope = (length * sumxy - sumx * sumy) / (length * sumx2 - sumx * sumx);
        let intercept = y_mean - slope * x_mean;

        let sq_total = y.iter().map(|y_i| (y_i - y_mean).powi(2)).sum::<f64>();
        let sq_residual = x
            .iter()
            .zip(y.iter())
            .map(|(&x_i, &y_i)| (y_i - (intercept + slope * x_i)).powi(2))
            .sum::<f64>();

        let std_err = (sq_residual / (length - 2_f64)).sqrt();
        let r2 = (sq_total - sq_residual) / sq_total;

        coefficients.push(intercept);
        coefficients.push(slope);

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

        let least_squares = LeastSquaresRegression;
        let model = least_squares.fit(&x, &y);

        assert!(approx_eq(model.slope().unwrap(), 1.458, ERROR_TOL));
        assert!(approx_eq(model.intercept(), -2.014, ERROR_TOL));
        assert!(approx_eq(model.std_err, 1.3067, ERROR_TOL));
        assert!(approx_eq(model.r2, 0.9144, ERROR_TOL));
        assert_eq!(model.to_polynomial_string(), "-2.01389 + 1.45833x")
    }

    #[test]
    fn perfect_line_recovery() {
        // y = 3x + 2
        let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let y: Vec<f64> = x.iter().map(|&x| 3.0 * x + 2.0).collect();

        let least_squares = LeastSquaresRegression;
        let model = least_squares.fit(&x, &y);

        assert!(approx_eq(model.slope().unwrap(), 3.0, ERROR_TOL));
        assert!(approx_eq(model.intercept(), 2.0, ERROR_TOL));
        assert!(approx_eq(model.std_err, 0.0, ERROR_TOL));
        assert!(approx_eq(model.r2, 1.0, 1e-6));
    }

    #[test]
    fn low_r2_score() {
        // No relationship: x varies, y is random noise
        let x: Vec<f64> = (0..20).map(|i| i as f64).collect();
        let y: Vec<f64> = vec![
            1.0, -1.0, 2.0, -2.0, 3.0, -3.0, 1.5, -0.5, 0.3, -0.3, 1.2, -1.1, 2.1, -2.1, 3.1, -3.1,
            0.2, 0.1, -0.2, 0.0,
        ];

        let least_squares = LeastSquaresRegression;
        let model = least_squares.fit(&x, &y);

        assert!(approx_eq(model.r2, 0.00642, 1e-5));
    }
}
