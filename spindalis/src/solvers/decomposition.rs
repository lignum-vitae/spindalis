use crate::solvers::SolverError;
use crate::utils::{Arr2D, Arr2DError};

// Doolittle algorithm for LU Decomposition
pub fn lu_decomposition<M>(matrix: M) -> Result<(Arr2D<f64>, Arr2D<f64>), SolverError>
where
    M: TryInto<Arr2D<f64>, Error = Arr2DError>,
{
    let matrix: Arr2D<f64> = matrix.try_into()?;
    if matrix.height != matrix.width {
        return Err(SolverError::NonSquareMatrix);
    }

    let mut lower: Arr2D<f64> = Arr2D::full(0.0, matrix.height, matrix.width);
    let mut upper: Arr2D<f64> = Arr2D::full(0.0, matrix.height, matrix.width);

    for i in 0..matrix.height {
        for k in i..matrix.height {
            let mut total = 0.0;
            for j in 0..i {
                total += lower[i][j] * upper[j][k];
            }
            upper[i][k] = matrix[i][k] - total;
        }
        for k in i..matrix.height {
            if i == k {
                lower[i][i] = 1_f64;
            } else {
                let mut total = 0.0;
                for j in 0..i {
                    total += lower[k][j] * upper[j][i];
                }
                lower[k][i] = (matrix[k][i] - total) / upper[i][i];
            }
        }
    }

    Ok((lower, upper))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::SolverError;
    use crate::utils::Arr2D;

    #[test]
    fn test_known_solution() {
        let mat = Arr2D::from(&[[2, -1, -2], [-4, 6, 3], [-4, -2, 8]]);
        let result = lu_decomposition(&mat).unwrap();
        let lower_exp = Arr2D::from(&[[1.0, 0.0, 0.0], [-2.0, 1.0, 0.0], [-2.0, -1.0, 1.0]]);
        let upper_exp = Arr2D::from(&[[2.0, -1.0, -2.0], [0.0, 4.0, -1.0], [0.0, 0.0, 3.0]]);
        let expected = (lower_exp, upper_exp);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_known_solution_nested_vectors() {
        let mat = vec![
            vec![2.0, -1.0, -2.0],
            vec![-4.0, 6.0, 3.0],
            vec![-4.0, -2.0, 8.0],
        ];
        let result = lu_decomposition(mat).unwrap();
        let lower_exp = Arr2D::from(&[[1.0, 0.0, 0.0], [-2.0, 1.0, 0.0], [-2.0, -1.0, 1.0]]);
        let upper_exp = Arr2D::from(&[[2.0, -1.0, -2.0], [0.0, 4.0, -1.0], [0.0, 0.0, 3.0]]);
        let expected = (lower_exp, upper_exp);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_known_solution_nested_borrowed_vectors() {
        let mat = vec![vec![2, -1, -2], vec![-4, 6, 3], vec![-4, -2, 8]];
        let result = lu_decomposition(&mat).unwrap();
        let lower_exp = Arr2D::from(&[[1.0, 0.0, 0.0], [-2.0, 1.0, 0.0], [-2.0, -1.0, 1.0]]);
        let upper_exp = Arr2D::from(&[[2.0, -1.0, -2.0], [0.0, 4.0, -1.0], [0.0, 0.0, 3.0]]);
        let expected = (lower_exp, upper_exp);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_non_square_matrix() {
        let mat = Arr2D::from(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]); // 2×3
        let result = lu_decomposition(&mat);

        assert!(matches!(result, Err(SolverError::NonSquareMatrix)));
    }
}
