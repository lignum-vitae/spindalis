pub fn pca() {}

fn _center_data(data: &Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
    let new_data: Vec<Vec<f64>> = data
        .iter()
        .map(|dimension| {
            let mean = dimension.iter().sum::<f64>() / dimension.len() as f64;
            dimension.iter().map(|&x| x - mean).collect()
        })
        .collect();
    Some(new_data)
}

fn _variance(data: &Vec<f64>) -> Result<f64, String> {
    let length = data.len();
    if length <= 0 {
        return Err("Input vector cannot be empty".to_string());
    }
    let length = length as f64;
    let mean: f64 = data.iter().sum::<f64>() / length;
    let var_sum: f64 = data.iter().map(|x| (x - mean) * (x - mean)).sum();
    Ok(var_sum / (length - 1.0))
}

fn _covariance(x_data: &Vec<f64>, y_data: &Vec<f64>) -> Result<f64, String> {
    let x_length = x_data.len();
    let y_length = y_data.len();
    if x_length <= 0 || y_length <= 0 {
        return Err("Input vector cannot be empty".to_string());
    }
    if x_length != y_length {
        return Err("Length of input vectors must be the same".to_string());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_data() {
        let data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];

        let centered = _center_data(&data).unwrap();
        let expected = vec![vec![-1.0, 0.0, 1.0], vec![-1.0, 0.0, 1.0]];

        for (row_c, row_e) in centered.iter().zip(expected.iter()) {
            for (c, e) in row_c.iter().zip(row_e.iter()) {
                assert!((c - e).abs() < 1e-8);
            }
        }
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
