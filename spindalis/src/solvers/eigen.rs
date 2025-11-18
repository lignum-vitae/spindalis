use crate::utils::{Arr2D, Arr2DError};

pub fn power_method<M>(matrix: M, es: f64) -> Result<(f64, Arr2D<f64>), Arr2DError>
where
    M: TryInto<Arr2D<f64>, Error = Arr2DError>,
{
    let matrix: Arr2D<f64> = matrix.try_into()?;
    let initial_eigen_vector = Arr2D::from(&[[1.0], [1.0], [1.0]]);
    let mut eigen_vector = &matrix * initial_eigen_vector;
    let mut eigen_value = eigen_vector.max().unwrap();
    eigen_vector = eigen_vector / eigen_value;
    loop {
        eigen_vector = &matrix * eigen_vector;
        let next_eigen_value = eigen_vector.max().unwrap();

        let ea = ((next_eigen_value - eigen_value) / next_eigen_value).abs();

        eigen_value = next_eigen_value;
        eigen_vector = eigen_vector / eigen_value;
        if ea < es {
            break;
        }
    }
    Ok((eigen_value, eigen_vector))
}

#[cfg(test)]
mod tests {
    use crate::utils::arr2D::Rounding;

    use super::*;

    #[test]
    fn test_known_solution() {
        // Largest eigenvalue and eigenvector
        let matrix = Arr2D::from(&[
            [3.556, -1.778, 0.0],
            [-1.778, 3.556, -1.778],
            [0.0, -1.778, 3.556],
        ]);
        let expected = (6.070, Arr2D::from(&[[1.0], [-1.414], [1.0]]));

        let (mut eigenvalue, mut eigenvector) = power_method(&matrix, 1e-10).unwrap();
        eigenvalue = (eigenvalue * 10_f64.powi(2)).round() / 10_f64.powi(2);
        eigenvector = eigenvector.round_to_decimal(3);
        let result = (eigenvalue, eigenvector);
        assert_eq!(result, expected);

        // Smallest eigenvalue and eigenvector
        let inverse_res = matrix.inverse().unwrap();
        let expected = (0.96, Arr2D::from(&[[0.707], [1.0], [0.707]]));

        let (mut eigenvalue, mut eigenvector) = power_method(&inverse_res, 1e-10).unwrap();
        eigenvalue = (eigenvalue * 10_f64.powi(2)).round() / 10_f64.powi(2);
        eigenvector = eigenvector.round_to_decimal(3);
        let result = (eigenvalue, eigenvector);
        assert_eq!(result, expected);
    }
}
