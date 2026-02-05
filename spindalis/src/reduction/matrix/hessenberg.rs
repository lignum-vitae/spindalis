use crate::solvers::SolverError;
use crate::utils::Arr2D;

/// Computes the Hessenberg reduction of a square matrix A.
/// Returns (H, Q) such that H = Q^T * A * Q, where H is upper Hessenberg and Q is orthogonal.
pub fn hessenberg_reduction(matrix: &Arr2D<f64>) -> Result<(Arr2D<f64>, Arr2D<f64>), SolverError> {
    if matrix.height != matrix.width {
        return Err(SolverError::NonSquareMatrix);
    }
    let n = matrix.height;
    if n <= 2 {
        return Ok((matrix.clone(), Arr2D::identity(n)));
    }

    let mut h = matrix.clone();
    let mut q = Arr2D::identity(n);

    for k in 0..n - 2 {
        // x = h[k+1..n, k]
        let mut x = 0.0;
        for i in k + 1..n {
            x += h[(i, k)] * h[(i, k)];
        }
        let norm_x = x.sqrt();

        if norm_x == 0.0 {
            continue;
        }

        // v = x + sign(x[0]) * ||x|| * e1
        let h_first = h[(k + 1, k)];
        let sign = if h_first >= 0.0 { -1.0 } else { 1.0 };
        let u1 = h_first - sign * norm_x;
        // v[0] is implicitly 1.0, we store the rest of v in a temporary slice or vec
        let mut v = vec![0.0; n - (k + 1)];
        v[0] = 1.0;
        for i in 1..v.len() {
            v[i] = h[(k + 1 + i, k)] / u1;
        }

        let tau = -sign * u1 / norm_x;

        // Apply H_k = I - tau * v * v^T to A from the left: A = H_k * A
        // This affects rows k+1..n
        for col in k..n {
            let mut dot = 0.0;
            for i in 0..v.len() {
                dot += v[i] * h[(k + 1 + i, col)];
            }
            for i in 0..v.len() {
                h[(k + 1 + i, col)] -= tau * v[i] * dot;
            }
        }

        // Apply H_k to A from the right: A = A * H_k
        // This affects columns k+1..n
        for row in 0..n {
            let mut dot = 0.0;
            for j in 0..v.len() {
                dot += v[j] * h[(row, k + 1 + j)];
            }
            for j in 0..v.len() {
                h[(row, k + 1 + j)] -= tau * v[j] * dot;
            }
        }

        // Accumulate Q = Q * H_k
        // This affects columns k+1..n of Q
        for row in 0..n {
            let mut dot = 0.0;
            for j in 0..v.len() {
                dot += v[j] * q[(row, k + 1 + j)];
            }
            for j in 0..v.len() {
                q[(row, k + 1 + j)] -= tau * v[j] * dot;
            }
        }
    }

    Ok((h, q))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::Rounding;

    #[test]
    fn test_hessenberg_2x2() {
        let mat = Arr2D::from(&[[1.0, 2.0], [3.0, 4.0]]);
        let (h, q) = hessenberg_reduction(&mat).unwrap();
        assert_eq!(h.round_to_decimal(10), mat.round_to_decimal(10));
        assert_eq!(
            q.round_to_decimal(10),
            Arr2D::identity(2).round_to_decimal(10)
        );
    }

    #[test]
    fn test_hessenberg_3x3() {
        // Simple 3x3 matrix
        let mat = Arr2D::from(&[[1.0, 5.0, 7.0], [3.0, 0.0, 6.0], [4.0, 3.0, 1.0]]);
        let (h, q) = hessenberg_reduction(&mat).unwrap();

        // Check if H is upper Hessenberg
        assert!(h[(2, 0)].abs() < 1e-10);

        // Check if Q is orthogonal: Q * Q^T = I
        let qt = q.transpose();
        let i = q.dot(&qt).unwrap();
        assert_eq!(
            i.round_to_decimal(10),
            Arr2D::identity(3).round_to_decimal(10)
        );

        // Check if A = Q * H * Q^T
        let reconstructed = q.dot(&h).unwrap().dot(&qt).unwrap();
        assert_eq!(reconstructed.round_to_decimal(10), mat.round_to_decimal(10));
    }

    #[test]
    fn test_hessenberg_4x4() {
        let mat = Arr2D::from(&[
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, 2.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        let (h, q) = hessenberg_reduction(&mat).unwrap();

        // Check upper Hessenberg form (zeros below subdiagonal)
        assert!(h[(2, 0)].abs() < 1e-10);
        assert!(h[(3, 0)].abs() < 1e-10);
        assert!(h[(3, 1)].abs() < 1e-10);

        // Check orthogonality
        let qt = q.transpose();
        let i = q.dot(&qt).unwrap();
        assert_eq!(
            i.round_to_decimal(10),
            Arr2D::identity(4).round_to_decimal(10)
        );

        // Check A = Q H Q^T
        let reconstructed = q.dot(&h).unwrap().dot(&qt).unwrap();
        assert_eq!(reconstructed.round_to_decimal(10), mat.round_to_decimal(10));
    }
}
