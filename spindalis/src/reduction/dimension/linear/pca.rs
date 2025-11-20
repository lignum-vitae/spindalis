use crate::reduction::dimension::{DimensionError, ReductionError};
use crate::utils::{StdDevType, arith_mean, arr2D::Arr2D, std_dev};

// ┌─────────────┬────────┬────────┬────────┬────────┬──────────┐
// │             │  USA   │ France │ Belgium│   UK   │ Czechia  │
// ├─────────────┼────────┼────────┼────────┼────────┼──────────┤
// │ Variable1   │   .    │   .    │   .    │   .    │   .      │
// │ Variable2   │   .    │   .    │   .    │   .    │   .      │
// │ Variable3   │   .    │   .    │   .    │   .    │   .      │
// │ Variable4   │   .    │   .    │   .    │   .    │   .      │
// └─────────────┴────────┴────────┴────────┴────────┴──────────┘

// Rows/data.height → variables
// Columns/data.width → features

pub fn pca() {}

fn _center_data(
    data: &Arr2D<f64>,
    std_type: Option<StdDevType>,
) -> Result<Arr2D<f64>, ReductionError> {
    let mut result = data.clone();
    let mut std = 1_f64;
    for row in &mut result {
        let len = row.len();
        if len == 0 {
            return Err(ReductionError::ShapeError(DimensionError::EmptyVector));
        }
        if let Some(std_kind) = std_type {
            std = std_dev(row, std_kind);
        }

        if std.is_nan() {
            return Err(ReductionError::ZeroMean);
        }

        let mean = arith_mean(row);
        for item in row {
            *item = (*item - mean) / std;
        }
    }
    Ok(result)
}

fn _variance(data: &[f64]) -> Result<f64, DimensionError> {
    let length = data.len();
    if length == 0 {
        return Err(DimensionError::EmptyVector);
    }
    let length = length as f64;
    let mean: f64 = data.iter().sum::<f64>() / length;
    let var_sum: f64 = data.iter().map(|x| (x - mean) * (x - mean)).sum();
    Ok(var_sum / (length - 1.0))
}

fn _covariance(x_data: &[f64], y_data: &[f64]) -> Result<f64, DimensionError> {
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

fn _cov_mat(data: &Arr2D<f64>) -> Result<Arr2D<f64>, ReductionError> {
    let height = data.height;

    let mut covariance_matrix = Arr2D::full(0.0, height, height);

    for (i, x) in data.into_iter().enumerate() {
        for (j, y) in data.into_iter().enumerate() {
            if i == j {
                covariance_matrix[i][j] = _variance(x).map_err(ReductionError::ShapeError)?;
            } else {
                covariance_matrix[i][j] = _covariance(x, y).map_err(ReductionError::ShapeError)?;
            }
        }
    }
    Ok(covariance_matrix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_data() {
        let data = Arr2D::from(&[[1., 2., 3.], [4., 5., 6.]]);

        let centered = _center_data(&data, None).unwrap();
        let expected = Arr2D::from(&[[-1., 0., 1.], [-1., 0., 1.]]);

        assert_eq!(centered, expected);
    }

    #[test]
    fn test_variance() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let result = _variance(&data).unwrap();
        let expected = 4.57;
        assert!((result - expected).abs() < 1e-2);
    }

    #[test]
    fn test_covariance() {
        let x = vec![2.1, 2.5, 4.0, 3.6];
        let y = vec![8.0, 12.0, 14.0, 10.0];

        let result = _covariance(&x, &y).unwrap();
        let expected = 1.53;
        assert!((result - expected).abs() < 1e-2);
    }

    #[test]
    fn test_variance_empty() {
        let data: Vec<f64> = vec![];
        let result = _variance(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_covariance_length_mismatch() {
        let x = vec![1.0, 2.0];
        let y = vec![1.0];
        let result = _covariance(&x, &y);
        assert!(result.is_err());
    }
}
