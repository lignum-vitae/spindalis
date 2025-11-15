use crate::solvers::SolverError;
use crate::utils::{Arr2DError, arr2D::Arr2D, back_substitution};

pub fn gaussian_elimination<M, W>(
    matrix: M,
    rhs: &[W],
    tolerance: f64,
) -> Result<Vec<f64>, SolverError>
where
    M: TryInto<Arr2D<f64>, Error = Arr2DError>,
    W: Into<f64> + Copy,
{
    let mut coeff_matrix: Arr2D<f64> = matrix.try_into()?;
    if coeff_matrix.height != coeff_matrix.width {
        return Err(SolverError::NonSquareMatrix);
    }
    if coeff_matrix.height != rhs.len() {
        return Err(SolverError::NumArgumentsMismatch {
            num_rows: coeff_matrix.height,
            rhs_len: rhs.len(),
        });
    }
    let mut rhs_vector: Vec<f64> = rhs.iter().map(|x| (*x).into()).collect();
    let size = coeff_matrix.height;
    let mut solution = vec![0.0; size];
    let mut error_flag = 0;

    // Scaling vector
    let mut scale_factor = vec![0.0; size];
    for i in 0..size {
        scale_factor[i] = coeff_matrix[i][0].abs();
        for j in 1..size {
            if coeff_matrix[i][j].abs() > scale_factor[i] {
                scale_factor[i] = coeff_matrix[i][j].abs();
            }
        }
    }
    forward_elimination(
        &mut coeff_matrix,
        &mut scale_factor,
        size,
        &mut rhs_vector,
        tolerance,
        &mut error_flag,
    );

    if error_flag != -1 {
        back_substitution(&coeff_matrix, size, &rhs_vector, &mut solution);
    }
    Ok(solution)
}

fn forward_elimination(
    coeff_matrix: &mut Arr2D<f64>,
    scale_factor: &mut [f64],
    size: usize,
    rhs_vector: &mut [f64],
    tol: f64,
    error_flag: &mut i32,
) {
    for k in 0..(size - 1) {
        partial_pivot(coeff_matrix, rhs_vector, scale_factor, size, k);
        if (coeff_matrix[k][k] / scale_factor[k]).abs() < tol {
            *error_flag = -1;
            return;
        }
        for i in (k + 1)..size {
            let factor = coeff_matrix[i][k] / coeff_matrix[k][k];
            for j in (k + 1)..size {
                coeff_matrix[i][j] -= factor * coeff_matrix[k][j];
            }
            rhs_vector[i] -= factor * rhs_vector[k]
        }
    }
    if (coeff_matrix[size - 1][size - 1] / scale_factor[size - 1]).abs() < tol {
        *error_flag = -1;
    }
}

fn partial_pivot(
    coeff_matrix: &mut Arr2D<f64>,
    rhs_vector: &mut [f64],
    scale_factor: &mut [f64],
    size: usize,
    k: usize,
) {
    let mut p = k;
    let mut big = (coeff_matrix[k][k] / scale_factor[k]).abs();
    for ii in (k + 1)..size {
        let temp = (coeff_matrix[ii][k] / scale_factor[ii]).abs();
        if temp > big {
            big = temp;
            p = ii;
        }
    }
    if p != k {
        // Swap rows in A
        coeff_matrix.swap_rows(p, k);

        // Swap entries in rhs_vector
        rhs_vector.swap(p, k);

        // Swap entries in scale_factor
        scale_factor.swap(p, k);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_elim() {
        let coeff_matrix = vec![
            vec![8.0, 2.0, -2.0],
            vec![10.0, 2.0, 4.0],
            vec![12.0, 2.0, 2.0],
        ];

        let rhs_vector = vec![8.0, 16.0, 16.0];
        let tol = 1e-12;
        let expected: Vec<f64> = vec![1.0; 3];

        let res = gaussian_elimination(&coeff_matrix, &rhs_vector, tol).unwrap();

        assert_eq!(res, expected)
    }

    #[test]
    fn test_known_elim_2() {
        let matrix = vec![vec![3, 6], vec![5, -8]];

        let coeff_matrix = Arr2D::try_from(matrix).unwrap();
        let rhs_vector = vec![12, 2];
        let tol = 1e-12;
        let expected: Vec<f64> = vec![2.0, 1.0];

        let res = gaussian_elimination(&coeff_matrix, &rhs_vector, tol).unwrap();

        assert_eq!(res, expected)
    }

    #[test]
    fn test_non_square() {
        let matrix = vec![
            vec![1.0, -2.0, 1.0, -1.0],
            vec![2.0, -3.0, 4.0, -3.0],
            vec![3.0, -5.0, 5.0, -4.0],
        ];

        let coeff_matrix: Arr2D<f64> = Arr2D::try_from(matrix).unwrap();
        let rhs_vector = vec![0.0; 4];
        let tol = 1e-12;

        let result = gaussian_elimination(&coeff_matrix, &rhs_vector, tol);

        assert!(matches!(result, Err(SolverError::NonSquareMatrix)));
    }
}
