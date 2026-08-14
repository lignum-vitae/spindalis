use crate::reduction::dimension::{DimensionError, ReductionError};
use crate::utils::{StdDevType, arith_mean, std_dev};
use jedvek::Matrix2D;

// ┌─────────────┬────────┬────────┬────────┬────────┬────────┐
// │             │ Feat 1 │ Feat 2 │ Feat 3 │ Feat 4 │ Feat 5 │
// ├─────────────┼────────┼────────┼────────┼────────┼────────┤
// │ Sample1     │   .    │   .    │   .    │   .    │    .   │
// │ Sample2     │   .    │   .    │   .    │   .    │    .   │
// │ Sample3     │   .    │   .    │   .    │   .    │    .   │
// │ Sample4     │   .    │   .    │   .    │   .    │    .   │
// └─────────────┴────────┴────────┴────────┴────────┴────────┘

// Rows/data.height → samples
// Columns/data.width → features

// Reference https://cs357.cs.illinois.edu/textbook/notes/pca.html

pub fn pca() {}

#[allow(dead_code)]
pub(crate) fn center_data(
    data: &mut Matrix2D<f64>,
    std_type: Option<StdDevType>,
) -> Result<&Matrix2D<f64>, ReductionError> {
    if data.height == 0 || data.width == 0 {
        return Err(ReductionError::ShapeError(DimensionError::EmptyVector));
    }
    let mut std = 1_f64;
    let mut column_data_vec = vec![0.0; data.height];
    for j in 0..data.width {
        let column_data: &mut [f64] = &mut column_data_vec;
        for i in 0..data.height {
            column_data[i] = data[i][j];
        }
        if let Some(std_kind) = std_type {
            std = std_dev(column_data, std_kind);
        }
        let mean = arith_mean(column_data);
        for item in &mut *column_data {
            *item = (*item - mean) / std;
        }
        for i in 0..data.height {
            data[i][j] = column_data[i];
        }
    }
    Ok(data)
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn testcenter_data_3x3_1() {
        let mut data = Matrix2D::from(&[[1., 4., 10.],
                                        [2., 5., 20.],
                                        [3., 9., 30.]]);
        let centered = center_data(&mut data, None).unwrap();
        let expected = Matrix2D::from(&[[-1., -2., -10.],
                                        [ 0., -1.,   0.],
                                        [ 1.,  3.,  10.]]);
        assert_eq!(*centered, expected);
    }

    #[test]
    fn testcenter_data_3x3_2() {
        let mut data = Matrix2D::from(&[[5., 12., 3.],
                                        [1., 8., 9.],
                                        [0., 4., 0.]]);
        let centered = center_data(&mut data, None).unwrap();
        let expected = Matrix2D::from(&[[ 3.,  4., -1.],
                                        [-1.,  0.,  5.],
                                        [-2., -4., -4.]]);
        assert_eq!(*centered, expected);
    }

    #[test]
    fn testcenter_data_4x4() {
        let mut data = Matrix2D::from(&[[10., 20., 30., 40.],
                                        [15., 25., 35., 45.],
                                        [ 5., 10., 15., 20.],
                                        [10., 25., 40., 55.]]);
        let centered = center_data(&mut data, None).unwrap();
        let expected = Matrix2D::from(&[[ 0.,   0.,   0.,   0.],
                                        [ 5.,   5.,   5.,   5.],
                                        [-5., -10., -15., -20.],
                                        [ 0.,   5.,  10.,  15.]]);
        assert_eq!(*centered, expected);
    }

    #[test]
    fn testcenter_data_5x5() {
        let mut data = Matrix2D::from(&[[3., 10.,  2., 8.,  6.],
                                        [4.,  4., 11., 3., 11.],
                                        [5.,  7.,  7., 9.,  3.],
                                        [6.,  3.,  8., 6.,  8.],
                                        [7.,  6., 12., 9.,  2.]]);
        let centered = center_data(&mut data, None).unwrap();
        let expected = Matrix2D::from(&[[-2.,  4., -6.,  1.,  0.],
                                        [-1., -2.,  3., -4.,  5.],
                                        [ 0.,  1., -1.,  2., -3.],
                                        [ 1., -3.,  0., -1.,  2.],
                                        [ 2.,  0.,  4.,  2., -4.]]);
        assert_eq!(*centered, expected);
    }

    #[test]
    fn testcenter_data_8x8() {
        let mut data = Matrix2D::from(&[[11.,  8., 13.,  6., 12.,  7., 14.,  5.],
                                        [12.,  7., 14.,  5., 11.,  8., 13.,  6.],
                                        [13.,  6., 11.,  8., 14.,  5., 12.,  7.],
                                        [14.,  5., 12.,  7., 13.,  6., 11.,  8.],
                                        [ 6., 13.,  8., 11.,  7., 14.,  5., 12.],
                                        [ 7., 14.,  5., 12.,  8., 13.,  6., 11.],
                                        [ 8., 11.,  7., 14.,  5., 12.,  7., 14.],
                                        [ 9., 16., 10., 17., 10., 15., 12., 17.]]);
        let centered = center_data(&mut data, None).unwrap();
        let expected = Matrix2D::from(&[[ 1., -2.,  3., -4.,  2., -3.,  4., -5.],
                                        [ 2., -3.,  4., -5.,  1., -2.,  3., -4.],
                                        [ 3., -4.,  1., -2.,  4., -5.,  2., -3.],
                                        [ 4., -5.,  2., -3.,  3., -4.,  1., -2.],
                                        [-4.,  3., -2.,  1., -3.,  4., -5.,  2.],
                                        [-3.,  4., -5.,  2., -2.,  3., -4.,  1.],
                                        [-2.,  1., -3.,  4., -5.,  2., -3.,  4.],
                                        [-1.,  6.,  0.,  7.,  0.,  5.,  2.,  7.]]);
        assert_eq!(*centered, expected);
    }

    #[test]
    fn testcenter_data_10x10_1() {
        let mut data = Matrix2D::from(&[[ 1.,  2.,  3.,  4.,  5.,  6.,  7.,  8.,  9., 10.],
                                        [ 2.,  3.,  4.,  5.,  6.,  7.,  8.,  9., 10.,  1.],
                                        [ 3.,  4.,  5.,  6.,  7.,  8.,  9., 10.,  1.,  2.],
                                        [ 4.,  5.,  6.,  7.,  8.,  9., 10.,  1.,  2.,  3.],
                                        [ 5.,  6.,  7.,  8.,  9., 10.,  1.,  2.,  3.,  4.],
                                        [ 6.,  7.,  8.,  9., 10.,  1.,  2.,  3.,  4.,  5.],
                                        [ 7.,  8.,  9., 10.,  1.,  2.,  3.,  4.,  5.,  6.],
                                        [ 8.,  9., 10.,  1.,  2.,  3.,  4.,  5.,  6.,  7.],
                                        [ 9., 10.,  1.,  2.,  3.,  4.,  5.,  6.,  7.,  8.],
                                        [ 0.,  1.,  2.,  3.,  4.,  5.,  6.,  7.,  8.,  9.]]);
        let centered = center_data(&mut data, None).unwrap();
        let expected = Matrix2D::from(&[[-3.5, -3.5, -2.5, -1.5, -0.5,  0.5,  1.5,  2.5,  3.5,  4.5],
                                        [-2.5, -2.5, -1.5, -0.5,  0.5,  1.5,  2.5,  3.5,  4.5, -4.5],
                                        [-1.5, -1.5, -0.5,  0.5,  1.5,  2.5,  3.5,  4.5, -4.5, -3.5],
                                        [-0.5, -0.5,  0.5,  1.5,  2.5,  3.5,  4.5, -4.5, -3.5, -2.5],
                                        [ 0.5,  0.5,  1.5,  2.5,  3.5,  4.5, -4.5, -3.5, -2.5, -1.5],
                                        [ 1.5,  1.5,  2.5,  3.5,  4.5, -4.5, -3.5, -2.5, -1.5, -0.5],
                                        [ 2.5,  2.5,  3.5,  4.5, -4.5, -3.5, -2.5, -1.5, -0.5,  0.5],
                                        [ 3.5,  3.5,  4.5, -4.5, -3.5, -2.5, -1.5, -0.5,  0.5,  1.5],
                                        [ 4.5,  4.5, -4.5, -3.5, -2.5, -1.5, -0.5,  0.5,  1.5,  2.5],
                                        [-4.5, -4.5, -3.5, -2.5, -1.5, -0.5,  0.5,  1.5,  2.5,  3.5]]);
        assert_eq!(*centered, expected);
    }

    #[test]
    fn testcenter_data_10x10_2() {
        let mut data = Matrix2D::from(&[[ -3.25,  -2.00,  -0.75,   0.25,   1.50,   2.75,   3.25,   4.50,   5.75,   6.25],
                                        [ -2.25,  -1.00,   0.25,   1.25,   2.50,   3.75,   4.25,   5.50,   6.75,   7.25],
                                        [ -1.25,   0.00,   1.25,   2.25,   3.50,   4.75,   5.25,   6.50,   7.75,   8.25],
                                        [ -0.25,   1.00,   2.25,   3.25,   4.50,   5.75,   6.25,   7.50,   8.75,   9.25],
                                        [  0.75,   2.00,   3.25,   4.25,   5.50,   6.75,   7.25,   8.50,   9.75,  10.25],
                                        [  1.75,   3.00,   4.25,   5.25,   6.50,   7.75,   8.25,   9.50,  10.75,  11.25],
                                        [  2.75,   4.00,   5.25,   6.25,   7.50,   8.75,   9.25,  10.50,  11.75,  12.25],
                                        [  3.75,   5.00,   6.25,   7.25,   8.50,   9.75,  10.25,  11.50,  12.75,  13.25],
                                        [  4.75,   6.00,   7.25,   8.25,   9.50,  10.75,  11.25,  12.50,  13.75,  14.25],
                                        [  5.75,   7.00,   8.25,   9.25,  10.50,  11.75,  12.25,  13.50,  14.75,  15.25]]);
        let centered = center_data(&mut data, None).unwrap();
        let expected = Matrix2D::from(&[[-4.5, -4.5, -4.5, -4.5, -4.5, -4.5, -4.5, -4.5, -4.5, -4.5],
                                        [-3.5, -3.5, -3.5, -3.5, -3.5, -3.5, -3.5, -3.5, -3.5, -3.5],
                                        [-2.5, -2.5, -2.5, -2.5, -2.5, -2.5, -2.5, -2.5, -2.5, -2.5],
                                        [-1.5, -1.5, -1.5, -1.5, -1.5, -1.5, -1.5, -1.5, -1.5, -1.5],
                                        [-0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5],
                                        [ 0.5,  0.5,  0.5,  0.5,  0.5,  0.5,  0.5,  0.5,  0.5,  0.5],
                                        [ 1.5,  1.5,  1.5,  1.5,  1.5,  1.5,  1.5,  1.5,  1.5,  1.5],
                                        [ 2.5,  2.5,  2.5,  2.5,  2.5,  2.5,  2.5,  2.5,  2.5,  2.5],
                                        [ 3.5,  3.5,  3.5,  3.5,  3.5,  3.5,  3.5,  3.5,  3.5,  3.5],
                                        [ 4.5,  4.5,  4.5,  4.5,  4.5,  4.5,  4.5,  4.5,  4.5,  4.5]]);
        assert_eq!(*centered, expected);
    }
}
