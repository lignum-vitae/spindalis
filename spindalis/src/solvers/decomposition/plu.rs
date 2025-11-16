// Source for implementation https://www.math.hkust.edu.hk/~machas/numerical-methods-for-engineers.pdf
// Companion video on youtube https://www.youtube.com/watch?v=WgaFycuL8z0
// In my implementation, the L and U matrices are computed simultaneously
// instead of calculating the U matrix then the L matrix as in the video
// L matrix is composed of multipliers for elimination (as stated at 6:55)
use crate::solvers::SolverError;
use crate::utils::{Arr2D, Arr2DError};

pub type PLUResult = (Arr2D<f64>, Arr2D<f64>, Arr2D<f64>);

// LU Decomposition with Partial pivoting
pub fn lu_pivot_decomposition<M>(matrix: M) -> Result<PLUResult, SolverError>
where
    M: TryInto<Arr2D<f64>, Error = Arr2DError>,
{
    let matrix: Arr2D<f64> = matrix.try_into()?;
    if matrix.height != matrix.width {
        return Err(SolverError::NonSquareMatrix);
    }

    let size = matrix.height;
    let mut lu = matrix.clone();
    let mut permutation: Arr2D<f64> = Arr2D::identity(size);

    for i in 0..size {
        // Find the best pivot row (p) for the current column (i)
        let mut pivot_row = i;
        let mut max_value = lu[i][i].abs();

        // Select best pivot row based on highest scaled pivot ratio in column
        for k in (i + 1)..size {
            let value = lu[k][i].abs();
            if value > max_value {
                max_value = value;
                pivot_row = k;
            }
        }

        if pivot_row != i {
            lu.swap_rows(pivot_row, i);
            permutation.swap_rows(pivot_row, i);
        }
        // Check for singularity BEFORE division
        if lu[i][i].abs() < f64::EPSILON {
            return Err(SolverError::SingularMatrix);
        }

        for k in (i + 1)..size {
            // Calculate the lower triangular matrix
            // Calculates multiplier needed to eliminate the element lu[k][i]
            lu[k][i] /= lu[i][i]; // division by 0 stopped by singularity check

            // Elimination step
            // lu[k][j] will form part of the final upper triangular matrix
            for j in (i + 1)..size {
                lu[k][j] -= lu[k][i] * lu[i][j];
            }
        }
    }
    let mut lower: Arr2D<f64> = Arr2D::full(0.0, size, size);
    let mut upper: Arr2D<f64> = Arr2D::full(0.0, size, size);

    for i in 0..size {
        for j in 0..size {
            if i == j {
                // U takes the diagonal elements from the LU matrix.
                upper[i][j] = lu[i][j];
                // L is a unit lower triangular matrix, so its diagonal elements are 1.0.
                lower[i][j] = 1.0;
            } else if i > j {
                // Lower triangle (below diagonal)
                lower[i][j] = lu[i][j];
            } else {
                // Upper triangle (above diagonal)
                upper[i][j] = lu[i][j];
            }
        }
    }
    Ok((lower, upper, permutation))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::Arr2D;

    #[test]
    fn test_known_solution() {
        let matrix = Arr2D::from(&[[0, 1, -2], [1, 0, 2], [3, -2, 2]]);
        let perm_exp = Arr2D::from(&[[0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]);
        let lower_exp = Arr2D::from(&[
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0 / 3.0, 2.0 / 3.0, 1.0],
        ]);
        let upper_exp = Arr2D::from(&[
            [3.0, -2.0, 2.0],
            [0.0, 1.0, -2.0],
            [0.0, 0.0, 2.666666666666667],
        ]);

        let expected = (lower_exp, upper_exp, perm_exp);

        let result = lu_pivot_decomposition(&matrix).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_known_solution_2() {
        let matrix = Arr2D::from(&[[-2, 2, -1], [6, -6, 7], [3, -8, 4]]);
        let perm_exp = Arr2D::from(&[[0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]]);
        let lower_exp = Arr2D::from(&[
            [1.0, 0.0, 0.0],
            [1.0 / 2.0, 1.0, 0.0],
            [-1.0 / 3.0, -0.0, 1.0], // result returns -0.0 instead of 0.0
        ]);
        let upper_exp = Arr2D::from(&[
            [6.0, -6.0, 7.0],
            [0.0, -5.0, 1.0 / 2.0],
            [0.0, 0.0, 1.333333333333333], // result is short by one decimal place compared to 4/3
        ]);

        let expected = (lower_exp, upper_exp, perm_exp);

        let result = lu_pivot_decomposition(&matrix).unwrap();
        assert_eq!(result, expected);
    }
}
