// Source for implementation https://jonshiach.github.io/ODEs-book/_pages/6.3_LUP_decomposition.html
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

    let mut lower: Arr2D<f64> = Arr2D::full(0.0, matrix.height, matrix.width);
    let mut upper: Arr2D<f64> = Arr2D::full(0.0, matrix.height, matrix.width);
    let mut permutation: Arr2D<f64> = Arr2D::full(0.0, matrix.height, matrix.width);
    for i in 0..matrix.height {
        permutation[i][i] = 1_f64;
    }
    let size = matrix.height;

    // Scaling vector -> maximum of absolute value of each row
    let mut scale_factor = vec![0.0; size];
    for i in 0..size {
        scale_factor[i] = matrix[i][0].abs();
        for j in 1..size {
            if matrix[i][j].abs() > scale_factor[i] {
                scale_factor[i] = matrix[i][j].abs();
            }
        }
    }

    partial_pivot_permutations(&matrix, &mut permutation, &mut scale_factor, size);

    let pivoted_matrix = &permutation * matrix;

    for i in 0..size {
        for k in i..size {
            let mut total = 0.0;
            for j in 0..i {
                total += lower[i][j] * upper[j][k];
            }
            upper[i][k] = pivoted_matrix[i][k] - total;
        }
        for k in i..size {
            if i == k {
                lower[i][i] = 1_f64;
            } else {
                let mut total = 0.0;
                for j in 0..i {
                    total += lower[k][j] * upper[j][i];
                }
                lower[k][i] = (pivoted_matrix[k][i] - total) / upper[i][i];
            }
        }
    }

    Ok((lower, upper, permutation))
}

fn partial_pivot_permutations(
    matrix: &Arr2D<f64>,
    permutation: &mut Arr2D<f64>,
    scale_factor: &mut [f64],
    size: usize,
) {
    for k in 0..size {
        let mut coeff_matrix = matrix.clone();
        let mut pivot = k;
        let mut big = (coeff_matrix[k][k] / scale_factor[k]).abs();
        for ii in (k + 1)..size {
            let temp = (coeff_matrix[ii][k] / scale_factor[ii]).abs();
            if temp > big {
                big = temp;
                pivot = ii;
            }
        }
        if pivot != k {
            // Swap rows in A
            coeff_matrix.swap_rows(pivot, k);

            // Swap rows in identity matrix
            permutation.swap_rows(pivot, k);

            // Swap entries in scale_factor
            scale_factor.swap(pivot, k);
        }
    }
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
        let upper_exp = Arr2D::from(&[[3.0, -2.0, 2.0], [0.0, 1.0, -2.0], [0.0, 0.0, 8.0 / 3.0]]);

        let expected = (lower_exp, upper_exp, perm_exp);

        let result = lu_pivot_decomposition(&matrix).unwrap();
        assert_eq!(result, expected);
    }
}
