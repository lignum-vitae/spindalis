use crate::reduction::dimension::DimensionError;
use crate::utils::StdDevType;
use jedvek::Matrix2D;

pub fn arith_mean(samples: &[f64]) -> f64 {
    let n = samples.len();
    if n == 0 {
        return f64::NAN;
    }
    samples.iter().sum::<f64>() / n as f64
}

pub fn geom_mean(samples: &[f64]) -> f64 {
    let n = samples.len();
    if n == 0 {
        return f64::NAN;
    }
    samples.iter().product::<f64>().powf(1_f64 / n as f64)
}

pub fn std_dev(samples: &[f64], correction: StdDevType) -> f64 {
    let n = samples.len();
    let denomiator = match correction {
        StdDevType::Poulation => n,
        StdDevType::Sample => n.saturating_sub(1),
    };
    if denomiator == 0 {
        return f64::NAN;
    }
    let mean = arith_mean(samples);
    let variance = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / denomiator as f64;

    variance.sqrt()
}

pub fn variance(data: &[f64]) -> Result<f64, DimensionError> {
    let length = data.len();
    if length == 0 {
        return Err(DimensionError::EmptyVector);
    }
    let length = length as f64;
    let mean: f64 = data.iter().sum::<f64>() / length;
    let var_sum: f64 = data.iter().map(|x| (x - mean) * (x - mean)).sum();
    Ok(var_sum / (length - 1.0))
}

pub fn covariance(x_data: &[f64], y_data: &[f64]) -> Result<f64, DimensionError> {
    let x_length = x_data.len();
    let y_length = y_data.len();
    if x_length == 0 || y_length == 0 {
        return Err(DimensionError::EmptyVector);
    }
    if x_length != y_length {
        return Err(DimensionError::DimensionMismatch {
            len_x: x_length,
            len_y: y_length,
        });
    }
    let n = x_length as f64;
    let x_mean: f64 = x_data.iter().sum::<f64>() / n;
    let y_mean: f64 = y_data.iter().sum::<f64>() / n;
    let cov_sum: f64 = x_data
        .iter()
        .zip(y_data.iter())
        .map(|(x, y)| (x - x_mean) * (y - y_mean))
        .sum();
    Ok(cov_sum / (n - 1.0))
}

pub fn compute_cov_mat(data: &Matrix2D<f64>) -> Result<Matrix2D<f64>, DimensionError> {
    if data.height <= 1 {
        return Err(DimensionError::InvalidDivision);
    }
    let m = data.height as f64;
    Ok(data.transpose() * data * (1. / (m - 1.)))
}

pub fn compute_cov_mat_unchecked(data: &Matrix2D<f64>) -> Matrix2D<f64> {
    let m = data.height as f64;
    data.transpose() * data * (1. / (m - 1.))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reduction::dimension::linear::pca::center_data;

    #[test]
    fn test_variance() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let result = variance(&data).unwrap();
        let expected = 4.57;
        assert!((result - expected).abs() < 1e-2);
    }

    #[test]
    fn test_covariance() {
        let x = vec![2.1, 2.5, 4.0, 3.6];
        let y = vec![8.0, 12.0, 14.0, 10.0];

        let result = covariance(&x, &y).unwrap();
        let expected = 1.53;
        assert!((result - expected).abs() < 1e-2);
    }

    #[test]
    fn test_variance_empty() {
        let data: Vec<f64> = vec![];
        let result = variance(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_covariance_length_mismatch() {
        let x = vec![1.0, 2.0];
        let y = vec![1.0];
        let result = covariance(&x, &y);
        assert!(result.is_err());
    }

    #[test]
    fn test_covar_mat_3x2() {
        let mut data = Matrix2D::from(&[[-1., -2.], [0., 1.], [1., 1.]]);
        let centered = center_data(&mut data, None).unwrap();
        let cov_mat = compute_cov_mat_unchecked(centered);
        let expected = Matrix2D::from(&[[1.0, 1.5], [1.5, 3.0]]);
        assert_eq!(cov_mat, expected);
    }

    #[test]
    fn test_covar_mat_5x2() {
        let mut data = Matrix2D::from(&[[-2., 4.], [-1., -2.], [0., 1.], [1., -3.], [2., 0.]]);
        let centered = center_data(&mut data, None).unwrap();
        let cov_mat = compute_cov_mat_unchecked(centered);
        let expected = Matrix2D::from(&[[2.50, -2.25], [-2.25, 7.50]]);
        assert_eq!(cov_mat, expected);
    }

    #[test]
    fn test_covar_mat_5x4() {
        let mut data = Matrix2D::from(&[
            [-3.5, 7.0, 100.0, -3.5],
            [-1.5, 3.0, 102.0, 0.5],
            [0.5, 6.0, 97.0, -4.5],
            [2.5, 8.0, 99.0, -1.5],
            [4.5, 1.0, 102.0, -3.5],
        ]);
        let centered = center_data(&mut data, None).unwrap();
        let cov_mat = compute_cov_mat_unchecked(centered);
        let expected = Matrix2D::from(&[
            [10.00, -3.50, 0.50, -1.00],
            [-3.50, 8.50, -4.50, -0.75],
            [0.50, -4.50, 4.50, 2.25],
            [-1.00, -0.75, 2.25, 4.00],
        ]);
        assert_eq!(cov_mat, expected);
    }
}
