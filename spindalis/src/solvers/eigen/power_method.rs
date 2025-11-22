use crate::utils::{Arr2D, Arr2DError};

pub fn power_method<M>(matrix: M, es: f64) -> Result<(f64, Arr2D<f64>), Arr2DError>
where
    M: TryInto<Arr2D<f64>, Error = Arr2DError>,
{
    let matrix: Arr2D<f64> = matrix.try_into()?;
    if matrix.height != matrix.width || matrix.height == 0 || matrix.width == 0 {
        return Err(Arr2DError::NonSquareMatrix);
    }
    let initial_eigenvector = Arr2D::from(&[[1.0], [1.0], [1.0]]);
    let mut eigenvector = &matrix * initial_eigenvector;
    // Arr2D.max() only returns None if the matrix is empty
    let mut eigenvalue = eigenvector.max().unwrap(); // Matrix won't be empty here
    eigenvector = eigenvector / eigenvalue; // Normalised Eigenvector
    loop {
        eigenvector = &matrix * eigenvector;
        let normalisation_value = eigenvector.max().unwrap(); // Matrix also won't be empty here
        let normalised_eigenvector = &eigenvector / normalisation_value;

        // Rayleigh quotient for faster convergence
        let numerator = &normalised_eigenvector.transpose() * (&matrix * &normalised_eigenvector); // x_k^T * (A * x_k)
        let denominator = &normalised_eigenvector.transpose() * &normalised_eigenvector; // x_k^T * x_k
        let next_eigenvalue = numerator.as_scalar_unchecked() / denominator.as_scalar_unchecked(); // convert to f64

        let ea = ((next_eigenvalue - eigenvalue) / next_eigenvalue).abs();

        eigenvalue = next_eigenvalue;
        eigenvector = normalised_eigenvector;
        if ea < es {
            break;
        }
    }
    Ok((eigenvalue, eigenvector))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::arr2D::Rounding;

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
    }

    #[test]
    fn test_known_inverse_solution() {
        // Smallest eigenvalue and eigenvector
        let matrix = Arr2D::from(&[
            [3.556, -1.778, 0.0],
            [-1.778, 3.556, -1.778],
            [0.0, -1.778, 3.556],
        ]);
        let inverse_res = matrix.inverse().unwrap();
        let rounded_expected_eigenvalue =
            ((1_f64 / 0.960127) * 10_f64.powi(5)).round() / 10_f64.powi(5);
        let expected = (
            rounded_expected_eigenvalue,
            Arr2D::from(&[[0.707], [1.0], [0.707]]),
        );

        let (converged_value, mut eigenvector) = power_method(&inverse_res, 1e-10).unwrap();
        let mut eigenvalue = 1_f64 / converged_value;
        eigenvalue = (eigenvalue * 10_f64.powi(5)).round() / 10_f64.powi(5);
        eigenvector = eigenvector.round_to_decimal(3);
        let result = (eigenvalue, eigenvector);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_known_solution_2() {
        // Largest eigenvalue and eigenvector
        let matrix = Arr2D::from(&[[2, 8, 10], [8, 4, 5], [10, 5, 7]]);
        let expected = (19.88, Arr2D::from(&[[0.9035], [0.7698], [1.0]]));

        let (mut eigenvalue, mut eigenvector) = power_method(&matrix, 1e-10).unwrap();
        eigenvalue = (eigenvalue * 10_f64.powi(2)).round() / 10_f64.powi(2);
        eigenvector = eigenvector.round_to_decimal(4);
        let result = (eigenvalue, eigenvector);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_known_inverse_solution_2() {
        // Smallest eigenvalue and eigenvector
        let matrix = Arr2D::from(&[[2, 8, 10], [8, 4, 5], [10, 5, 7]]);
        let expected = (0.29, Arr2D::from(&[[0.04117], [1.0], [-0.80702]]));

        let inverse_res = matrix.inverse().unwrap();
        let (converged_value, mut eigenvector) = power_method(&inverse_res, 1e-10).unwrap();
        let mut eigenvalue = 1_f64 / converged_value;
        eigenvalue = (eigenvalue * 10_f64.powi(2)).round() / 10_f64.powi(2);
        eigenvector = eigenvector.round_to_decimal(5);
        let result = (eigenvalue, eigenvector);
        assert_eq!(result, expected);
    }
}
