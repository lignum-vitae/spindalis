use crate::utils::arr2D::Arr2D;

#[allow(clippy::needless_range_loop)]
pub fn back_substitution(
    coeff_matrix: &Arr2D<f64>,
    size: usize,
    rhs_vector: &[f64],
    solution: &mut [f64],
) {
    solution[size - 1] = rhs_vector[size - 1] / coeff_matrix[size - 1][size - 1];
    for i in (0..(size - 1)).rev() {
        let mut sum = 0.0;
        for j in (i + 1)..size {
            sum += coeff_matrix[i][j] * solution[j]
        }
        solution[i] = (rhs_vector[i] - sum) / coeff_matrix[i][i];
    }
}

#[allow(clippy::needless_range_loop)]
pub fn forward_substitution(
    coeff_matrix: &Arr2D<f64>,
    size: usize,
    rhs_vector: &[f64],
    solution: &mut [f64],
) {
    for i in 0..size {
        let mut sum = 0.0;
        for j in 0..i {
            sum += coeff_matrix[i][j] * solution[j]
        }
        solution[i] = (rhs_vector[i] - sum) / coeff_matrix[i][i];
    }
}
