use crate::regressors::linear::{LinearModel, LinearRegressor};

pub struct GradientDescentRegression {
    pub steps: usize,
    pub step_size: f64,
}

impl LinearRegressor for GradientDescentRegression {
    fn fit(&self, x: &[f64], y: &[f64]) -> LinearModel {
        let mut coefficients = Vec::new();
        let mut wx = 0.0_f64;
        let length = y.len() as f64;
        let y_mean = y.iter().sum::<f64>() / length;
        let mut wy = y_mean;
        let alpha = self.step_size;

        for _ in 0..self.steps {
            let y_pred: Vec<f64> = x.iter().map(|&x_i| wy + wx * x_i).collect();

            // partial derivative of Loss function with respect to weight y
            let gradient_wy = y_pred
                .iter()
                .zip(y)
                .map(|(y_pred_i, y_i)| y_pred_i - y_i)
                .sum::<f64>()
                / (y.len() as f64);

            // partial derivative of Loss function with respect to weight x
            let gradient_wx = y_pred
                .iter()
                .zip(y)
                .zip(x)
                .map(|((y_pred_i, y_i), x_i)| (y_pred_i - y_i) * x_i)
                .sum::<f64>()
                / (y.len() as f64);

            // Update weights
            wx -= alpha * gradient_wx;
            wy -= alpha * gradient_wy;
        }

        let sq_total: f64 = y.iter().map(|y_i| (y_i - y_mean).powi(2)).sum();
        let sq_residual: f64 = x
            .iter()
            .zip(y.iter())
            .map(|(&x_i, &y_i)| (y_i - (wy + wx * x_i)).powi(2))
            .sum();

        let std_err = (sq_residual / (length - 2.0)).sqrt();
        let r2 = (sq_total - sq_residual) / sq_total;

        coefficients.push(wy);
        coefficients.push(wx);

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

        let grad_descent = GradientDescentRegression {
            steps: 10000,
            step_size: 0.01,
        };
        let model = grad_descent.fit(&x, &y);

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

        let grad_descent = GradientDescentRegression {
            steps: 5000,
            step_size: 0.01,
        };
        let model = grad_descent.fit(&x, &y);

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

        let grad_descent = GradientDescentRegression {
            steps: 8000,
            step_size: 0.01,
        };
        let model = grad_descent.fit(&x, &y);

        assert!(approx_eq(model.r2, 0.00642, 1e-5));
    }
}
