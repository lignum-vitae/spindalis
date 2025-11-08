use crate::utils::Arr2DError;
use std::{
    any::type_name,
    fmt::{self, Display},
    ops::{Index, IndexMut, Mul},
};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Default)]
pub struct Arr2D<T> {
    inner: Vec<T>,
    pub height: usize,
    pub width: usize,
}

impl<T> Arr2D<T> {
    /// Return a tuple of (height, width)
    pub fn shape(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    /// Return the total number of items (i.e. height * width)
    pub fn size(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.height == 0 || self.width == 0
    }

    /// Change the height and width
    ///
    /// # Errors
    ///
    /// This function will return an error if the size isn't divislbe by the new height
    pub fn reshape(&mut self, height: usize) -> Result<(), Arr2DError> {
        let size = self.height * self.width;
        if !size.is_multiple_of(height) {
            return Err(Arr2DError::InvalidReshape {
                size,
                new_height: height,
            });
        }

        self.height = height;
        self.width = size / height;
        Ok(())
    }

    /// Element-wise map operatiorn
    pub fn map<F, U>(&self, f: F) -> Arr2D<U>
    where
        F: Fn(&T) -> U,
    {
        let inner: Vec<_> = self.inner.iter().map(f).collect();

        Arr2D {
            inner,
            height: self.height,
            width: self.width,
        }
    }

    /// Create an iterator of refs to rows
    pub fn rows(&self) -> Arr2DRows<'_, T> {
        Arr2DRows {
            data: &self.inner,
            width: self.width,
            remaining: self.height,
        }
    }

    /// Create an iterator of mut refs to rows
    pub fn rows_mut(&mut self) -> Arr2DRowsMut<'_, T> {
        Arr2DRowsMut {
            data: self.inner.as_mut_slice(),
            width: self.width,
            remaining: self.height,
        }
    }

    //  Create 2D Array from flat vector
    pub fn from_flat<D>(
        inner: D,
        default_val: T,
        height: usize,
        width: usize,
    ) -> Result<Self, Arr2DError>
    where
        D: AsRef<[T]>,
        T: Clone,
    {
        let vec_len = inner.as_ref().len();
        let Arr2D_size = height * width;
        if vec_len > Arr2D_size || Arr2D_size == 0 {
            return Err(Arr2DError::InvalidShape {
                input_size: (vec_len),
                output_size: (Arr2D_size),
            });
        }

        let inner = inner.as_ref().to_vec();
        if vec_len < Arr2D_size {
            let mut new_inner = inner.clone();
            new_inner.resize(Arr2D_size, default_val);
            return Ok(Self {
                inner: new_inner,
                height,
                width,
            });
        }

        Ok(Self {
            inner,
            height,
            width,
        })
    }

    // Dot product for scalar x matrix, vector x matrix, and matrix x matrix
    pub fn dot(&self, rhs: &Self) -> Result<Self, Arr2DError>
    where
        T: Copy + std::default::Default + std::ops::AddAssign + std::ops::Mul<Output = T>,
    {
        if self.height == 1 && self.width == 1 {
            let scalar = self[0][0];
            let mut result = Arr2D::full(T::default(), rhs.height, rhs.width);
            for i in 0..rhs.height {
                for j in 0..rhs.width {
                    result[i][j] = scalar * rhs[i][j];
                }
            }
            return Ok(result);
        }
        if self.width != rhs.height {
            return Err(Arr2DError::InvalidDotShape {
                lhs: self.width,
                rhs: rhs.height,
            });
        }
        let mut result = Arr2D::full(T::default(), self.height, rhs.width);
        for i in 0..self.height {
            for j in 0..rhs.width {
                let mut sum = T::default();
                for k in 0..self.width {
                    sum += self[i][k] * rhs[k][j]
                }
                result[i][j] = sum;
            }
        }
        Ok(result)
    }
}

// Generic Mul implementation for Arr2D * Arr2D (matrix * matrix)
impl<T> Mul<Arr2D<T>> for Arr2D<T>
where
    T: Mul<Output = T> + Clone + std::default::Default + std::marker::Copy + std::ops::AddAssign,
{
    type Output = Arr2D<T>;

    fn mul(self, rhs: Arr2D<T>) -> Self {
        self.dot(&rhs).unwrap_or_default()
    }
}

// Generic Mul implementation for Arr2D * scalar (matrix * scalar)
impl<T> Mul<T> for Arr2D<T>
where
    T: Mul<Output = T> + Clone + std::default::Default + std::marker::Copy,
{
    type Output = Arr2D<T>;

    fn mul(self, rhs: T) -> Self {
        let mut result = Arr2D::full(T::default(), self.height, self.width);
        for i in 0..self.height {
            for j in 0..self.width {
                result[i][j] = self[i][j] * rhs;
            }
        }
        result
    }
}

impl<T: Copy> Arr2D<T> {
    pub fn transpose(&mut self) {
        let mut new_inner = Vec::with_capacity(self.inner.len());
        for col in 0..self.width {
            for row in 0..self.height {
                new_inner.push(self[(row, col)]);
            }
        }

        self.inner = new_inner;
        std::mem::swap(&mut self.width, &mut self.height);
    }

    pub fn swap_rows(&mut self, mut a: usize, mut b: usize) {
        if a == b {
            return;
        }
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        let w = self.width;

        let (left, right) = self.inner.split_at_mut(b * w);
        let row_b = &mut right[..w];
        let row_a = &mut left[a * w..(a + 1) * w];

        row_a.swap_with_slice(row_b);
    }

    /// Create Arr2D with all elements being the specified value
    pub fn full(val: T, height: usize, width: usize) -> Self {
        Arr2D {
            inner: vec![val; height * width],
            height,
            width,
        }
    }
}

/// Iterator for Arr2D
pub struct Arr2DRows<'a, T> {
    data: &'a [T],
    width: usize,
    /// remaining row count
    remaining: usize,
}

impl<'a, T> Iterator for Arr2DRows<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        self.remaining -= 1;

        if self.width == 0 {
            Some(&self.data[..0])
        } else {
            let (row, rest) = self.data.split_at(self.width);
            self.data = rest;
            Some(row)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

/// Mut iterator for Arr2D
pub struct Arr2DRowsMut<'a, T> {
    data: &'a mut [T],
    width: usize,
    remaining: usize,
}

impl<'a, T> Iterator for Arr2DRowsMut<'a, T> {
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        // it's important to take a temporary ownership over the mut slice here
        // because `self.data` would be alive only during an invocation of this method
        // (which would lead to borrow checker error).
        // `std::mem::take` allows you to own the slice, leaving `self.data` with an empty slice.
        let (row, rest) = std::mem::take(&mut self.data).split_at_mut(self.width);

        self.data = rest;
        self.remaining -= 1;

        Some(row)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

// Convert a nested Vec to Arr2D
impl<T> TryFrom<Vec<Vec<T>>> for Arr2D<T> {
    type Error = Arr2DError;

    fn try_from(values: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        if values.is_empty() {
            return Ok(Self {
                inner: vec![],
                height: 0,
                width: 0,
            });
        }
        let width = values[0].len();
        let height = values.len();
        let mut inner: Vec<T> = Vec::with_capacity(height * width);
        for row in values {
            if row.len() != width {
                return Err(Arr2DError::InconsistentRowLengths);
            }
            inner.extend(row);
        }
        Ok(Self {
            inner,
            height,
            width,
        })
    }
}

// Vec<Vec<T>> -> Arr2D<U>
impl<T: Clone, U: TryFrom<T>> TryFrom<&Vec<Vec<T>>> for Arr2D<U> {
    type Error = Arr2DError;

    fn try_from(values: &Vec<Vec<T>>) -> Result<Self, Self::Error> {
        if values.is_empty() {
            return Ok(Self {
                inner: vec![],
                height: 0,
                width: 0,
            });
        }

        let width = values[0].len();
        let mut inner = Vec::with_capacity(values.len() * width);

        for row in values {
            if row.len() != width {
                return Err(Arr2DError::InconsistentRowLengths);
            }

            for x in row {
                inner.push(
                    U::try_from(x.clone()).map_err(|_| Arr2DError::ConversionFailed {
                        from: type_name::<T>(),
                        to: type_name::<U>(),
                    })?,
                );
            }
        }

        Ok(Self {
            inner,
            height: values.len(),
            width,
        })
    }
}

// Arr2D<T> -> Arr2D<U>
impl<T: Clone, U: TryFrom<T>> TryFrom<&Arr2D<T>> for Arr2D<U> {
    type Error = Arr2DError;

    fn try_from(arr: &Arr2D<T>) -> Result<Self, Self::Error> {
        if arr.is_empty() {
            return Ok(Self {
                inner: vec![],
                height: 0,
                width: 0,
            });
        }

        let mut inner = Vec::with_capacity(arr.inner.len());
        for x in &arr.inner {
            inner.push(
                U::try_from(x.clone()).map_err(|_| Arr2DError::ConversionFailed {
                    from: type_name::<T>(),
                    to: type_name::<U>(),
                })?,
            );
        }

        Ok(Self {
            inner,
            height: arr.height,
            width: arr.width,
        })
    }
}

impl<'a, T> IntoIterator for &'a Arr2D<T> {
    type Item = &'a [T];
    type IntoIter = Arr2DRows<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows()
    }
}

impl<'a, T> IntoIterator for &'a mut Arr2D<T> {
    type Item = &'a mut [T];
    type IntoIter = Arr2DRowsMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows_mut()
    }
}

// Convert a nested array like `&[[1, 2], [3, 4]]` to Arr2D
impl<T, const M: usize, const N: usize> From<&[[T; N]; M]> for Arr2D<T>
where
    T: Clone,
{
    fn from(values: &[[T; N]; M]) -> Self {
        let mut inner = Vec::with_capacity(M * N);

        for row in values.iter() {
            inner.extend_from_slice(row);
        }

        Self {
            inner,
            height: M,
            width: N,
        }
    }
}

// Allow indexing Arr2D items like `arr[(0, 1)]`
impl<T> Index<(usize, usize)> for Arr2D<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        let (row, col) = idx;
        if row >= self.height || col >= self.width {
            panic!(
                "Out of bound index ({row},{col}) into Arr2D of shape ({},{})",
                self.height, self.width
            )
        }
        &self.inner[row * self.width + col]
    }
}
impl<T> IndexMut<(usize, usize)> for Arr2D<T> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        let (row, col) = idx;
        if row >= self.height || col >= self.width {
            panic!(
                "Out of bound index ({row},{col}) into Arr2D of shape ({},{})",
                self.height, self.width
            )
        }
        &mut self.inner[row * self.width + col]
    }
}

// Allow indexing Arr2D rows like `arr[1]`
impl<T> Index<usize> for Arr2D<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        if row >= self.height {
            panic!(
                "Out of bound row index {row} into Arr2D of shape ({},{})",
                self.height, self.width
            )
        }
        &self.inner[row * self.width..(row + 1) * self.width]
    }
}
impl<T> IndexMut<usize> for Arr2D<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        if row >= self.height {
            panic!(
                "Out of bound row index {row} into Arr2D of shape ({},{})",
                self.height, self.width
            )
        }
        &mut self.inner[row * self.width..(row + 1) * self.width]
    }
}

impl<T: Display> Display for Arr2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.height == 0 || self.width == 0 {
            writeln!(f, "[]")?;
            return Ok(());
        }

        // Get max width per column
        let mut col_widths = vec![0; self.width];
        for c in 0..self.width {
            col_widths[c] = (0..self.height)
                .map(|r| format!("{}", self[(r, c)]).len())
                .max()
                .unwrap_or(0);
        }

        // Print
        for r in 0..self.height {
            if r == 0 {
                write!(f, "[[ ")?;
            } else {
                write!(f, " [ ")?;
            }
            for c in 0..self.width {
                let item = &self[(r, c)];
                write!(f, "{:>width$}", *item, width = col_widths[c])?;
                if c + 1 != self.width {
                    write!(f, ", ")?;
                }
            }
            if r + 1 == self.height {
                write!(f, " ]]")?;
            } else {
                writeln!(f, " ]")?;
            }
        }

        Ok(())
    }
}

impl<T: PartialEq> PartialEq<Vec<Vec<T>>> for Arr2D<T> {
    fn eq(&self, other: &Vec<Vec<T>>) -> bool {
        if self.height != other.len() {
            return false;
        }
        if self.height == 0 {
            return true; // both empty
        }

        let width = self.width;

        if other.iter().any(|row| row.len() != width) {
            return false;
        }

        for r in 0..self.height {
            for c in 0..self.width {
                if self[r][c] != other[r][c] {
                    return false;
                }
            }
        }
        true
    }
}

impl<T: PartialEq> PartialEq<Arr2D<T>> for Vec<Vec<T>> {
    fn eq(&self, other: &Arr2D<T>) -> bool {
        other == self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- basic getters ---

    #[test]
    fn test_getting_shape() {
        let data = Arr2D::from(&[[1, 2, 3], [6, 5, 4]]);
        assert_eq!(data.shape(), (2, 3));

        let data: Arr2D<i32> = Arr2D::from(&[[1, 2]; 0]);
        assert_eq!(data.shape(), (0, 2));

        let data: Arr2D<i32> = Arr2D::from(&[[]; 0]);
        assert_eq!(data.shape(), (0, 0));

        let data: Arr2D<i32> = Arr2D::from(&[[], []]);
        assert_eq!(data.shape(), (2, 0));
    }

    #[test]
    fn test_getting_size() {
        let data = Arr2D::from(&[[1, 2, 3], [6, 5, 4]]);
        assert_eq!(data.size(), 6);

        let data: Arr2D<i32> = Arr2D::from(&[[1, 2]; 0]);
        assert_eq!(data.size(), 0);

        let data: Arr2D<i32> = Arr2D::from(&[[]; 0]);
        assert_eq!(data.size(), 0);

        let data: Arr2D<i32> = Arr2D::from(&[[], []]);
        assert_eq!(data.size(), 0);
    }

    // --- indexing ---

    #[test]
    fn test_indexing_item() {
        let data = Arr2D::from(&[[1, 2, 3], [6, 5, 4]]);
        assert_eq!(data[(0, 0)], 1);
        assert_eq!(data[(0, 1)], 2);
        assert_eq!(data[(0, 2)], 3);
        assert_eq!(data[(1, 0)], 6);
        assert_eq!(data[(1, 1)], 5);
        assert_eq!(data[(1, 2)], 4);
    }

    #[test]
    fn test_2D_indexing_item() {
        let data = Arr2D::from(&[[1, 2, 3], [6, 5, 4]]);
        assert_eq!(data[0][0], 1);
        assert_eq!(data[0][1], 2);
        assert_eq!(data[0][2], 3);
        assert_eq!(data[1][0], 6);
        assert_eq!(data[1][1], 5);
        assert_eq!(data[1][2], 4);
    }

    #[test]
    fn test_mut_indexing_item() {
        let mut data = Arr2D::from(&[[1, 2, 3], [6, 5, 4]]);
        data[(1, 2)] = 10;
        data[(0, 0)] = 11;
        assert_eq!(data[(0, 0)], 11);
        assert_eq!(data[(0, 1)], 2);
        assert_eq!(data[(0, 2)], 3);
        assert_eq!(data[(1, 0)], 6);
        assert_eq!(data[(1, 1)], 5);
        assert_eq!(data[(1, 2)], 10);
    }

    #[test]
    fn test_2D_mut_indexing_item() {
        let mut data = Arr2D::from(&[[1, 2, 3], [6, 5, 4]]);
        data[(1, 2)] = 10;
        data[(0, 0)] = 11;
        assert_eq!(data[0][0], 11);
        assert_eq!(data[0][1], 2);
        assert_eq!(data[0][2], 3);
        assert_eq!(data[1][0], 6);
        assert_eq!(data[1][1], 5);
        assert_eq!(data[1][2], 10);
    }

    #[test]
    fn test_indexing_row() {
        let data = Arr2D::from(&[[1, 2, 3], [6, 5, 4]]);
        assert_eq!(data[0], [1, 2, 3]);
        assert_eq!(data[1], [6, 5, 4]);
    }
    #[test]
    fn test_mut_indexing_row() {
        let mut data = Arr2D::from(&[[1, 2, 3], [6, 5, 4]]);
        data[0][2] = 9;
        data[1][0] = 10;
        assert_eq!(data[0], [1, 2, 9]);
        assert_eq!(data[1], [10, 5, 4]);
    }

    #[test]
    #[should_panic]
    fn test_index_out_of_bounds_panics() {
        let data = Arr2D::from(&[[1, 2, 3], [4, 5, 6]]);

        let _ = data[(2, 0)];
    }
    #[test]
    #[should_panic]
    fn test_mut_index_out_of_bounds_panics() {
        let mut data = Arr2D::from(&[[1, 2, 3], [4, 5, 6]]);

        let _ = &mut data[(2, 0)];
    }

    #[test]
    #[should_panic]
    fn test_row_index_out_of_bounds_panics() {
        let data = Arr2D::from(&[[1, 2, 3], [4, 5, 6]]);

        let _ = &data[2];
    }
    #[test]
    #[should_panic]
    fn test_row_mut_index_out_of_bounds_panics() {
        let mut data = Arr2D::from(&[[1, 2, 3], [4, 5, 6]]);

        let _ = &mut data[2];
    }

    // --- iterating ---
    #[test]
    fn test_rows_iterator_returns_slices() {
        let data = Arr2D::from(&[[1, 2, 3], [4, 5, 6]]);
        let rows: Vec<&[i32]> = data.rows().collect();

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], &[1, 2, 3]);
        assert_eq!(rows[1], &[4, 5, 6]);
    }

    #[test]
    fn test_iterating_through_items() {
        let data = Arr2D::from(&[[1, 2, 3], [6, 5, 4]]);
        let mut expected = [1, 2, 3, 6, 5, 4].iter();

        for row in data.rows() {
            for item in row {
                assert_eq!(item, expected.next().unwrap());
            }
        }
    }

    #[test]
    fn test_rows_mut_iterator_allows_mutation() {
        let mut data = Arr2D::from(&[[1, 2, 3], [4, 5, 6]]);

        for row in data.rows_mut() {
            row.reverse();
        }

        let expected = Arr2D::from(&[[3, 2, 1], [6, 5, 4]]);
        assert_eq!(data, expected);
    }

    // --- transformation ---

    #[test]
    fn test_reshape() {
        let mut data = Arr2D::from(&[[1, 2, 3], [6, 5, 4]]);
        data.reshape(3).unwrap();
        let expected = Arr2D::from(&[[1, 2], [3, 6], [5, 4]]);

        assert_eq!(data, expected);
    }

    #[test]
    fn test_reshape_invalid_height_errors() {
        let mut data = Arr2D::from(&[[1, 2], [3, 4]]);
        let err = data
            .reshape(3)
            .expect_err("reshape should fail when new height mismatches size");

        assert!(matches!(
            err,
            Arr2DError::InvalidReshape {
                size: 4,
                new_height: 3
            }
        ));
    }

    #[test]
    fn test_transpose() {
        let mut data = Arr2D::from(&[[1, 2, 3], [6, 5, 4]]);
        data.transpose();
        let expected = Arr2D::from(&[[1, 6], [2, 5], [3, 4]]);
        assert_eq!(data, expected);
    }

    #[test]
    fn test_arr_from_vec() {
        let data = Arr2D::try_from(vec![vec![1, 2, 3], vec![6, 5, 4]]).unwrap();
        let expected = Arr2D::from(&[[1, 2, 3], [6, 5, 4]]);
        assert_eq!(data, expected);
    }

    #[test]
    fn test_try_from_inconsistent_rows_returns_error() {
        let err = Arr2D::try_from(vec![vec![1, 2, 3], vec![4, 5]])
            .expect_err("rows with different widths should error");

        assert!(matches!(err, Arr2DError::InconsistentRowLengths));
    }

    #[test]
    fn test_try_from_empty_vec_creates_empty_arr() {
        let data = Arr2D::try_from(Vec::<Vec<i32>>::new()).unwrap();

        assert_eq!(data.rows().count(), 0);
    }

    #[test]
    fn test_map_transforms_elements() {
        let data = Arr2D::from(&[[1, 2], [3, 4]]);
        let mapped = data.map(|value| value * 2);
        let expected = Arr2D::from(&[[2, 4], [6, 8]]);

        assert_eq!(mapped, expected);
    }

    #[test]
    fn test_full() {
        let data = Arr2D::full(10, 3, 4);
        for row in data.rows() {
            for item in row {
                assert_eq!(*item, 10);
            }
        }
    }

    // --- from flat vector to Arr2D ---

    #[test]
    fn test_from_flat() {
        let data = Arr2D::from_flat(vec![1, 2, 3, 4, 5, 6], 0, 2, 3).unwrap();
        let out = Arr2D::from(&[[1, 2, 3], [4, 5, 6]]);

        assert_eq!(data, out);
    }

    #[test]
    #[allow(clippy::needless_borrows_for_generic_args)]
    fn test_from_flat_ref() {
        let data = Arr2D::from_flat(&vec![1, 2, 3, 4, 5, 6], 0, 2, 3).unwrap();
        let out = Arr2D::from(&[[1, 2, 3], [4, 5, 6]]);

        assert_eq!(data, out);
    }

    #[test]
    #[allow(clippy::needless_borrows_for_generic_args)]
    fn test_from_flat_slice() {
        let data = Arr2D::from_flat(&[1, 2, 3, 4, 5, 6], 0, 2, 3).unwrap();
        let out = Arr2D::from(&[[1, 2, 3], [4, 5, 6]]);

        assert_eq!(data, out);
    }

    #[test]
    fn test_from_flat_with_default() {
        let data = Arr2D::from_flat(vec![1, 2, 3, 4], 0, 2, 3).unwrap();
        let out = Arr2D::from(&[[1, 2, 3], [4, 0, 0]]);

        assert_eq!(data, out);
    }

    #[test]
    #[allow(clippy::needless_borrows_for_generic_args)]
    fn test_from_flat_with_default_ref() {
        let data = Arr2D::from_flat(&vec![1, 2, 3, 4], 0, 2, 3).unwrap();
        let out = Arr2D::from(&[[1, 2, 3], [4, 0, 0]]);

        assert_eq!(data, out);
    }

    #[test]
    #[allow(clippy::needless_borrows_for_generic_args)]
    fn test_from_flat_full_zeros() {
        let flat_data = Arr2D::from_flat(&vec![], 0, 2, 3).unwrap();
        let full_data = Arr2D::full(0, 2, 3);

        assert_eq!(flat_data, full_data);
    }

    #[test]
    #[allow(clippy::needless_borrows_for_generic_args)]
    fn test_from_flat_slice_full_zeros() {
        let data = Arr2D::from_flat(&[], 0, 2, 3).unwrap();
        let out = Arr2D::from(&[[0, 0, 0], [0, 0, 0]]);

        assert_eq!(data, out);
    }

    #[test]
    fn test_from_flat_slice_full_zeros_no_ref() {
        let data = Arr2D::from_flat([], 0, 2, 3).unwrap();
        let out = Arr2D::from(&[[0, 0, 0], [0, 0, 0]]);

        assert_eq!(data, out);
    }

    // --- dot product ---

    #[test]
    fn test_mat_mul_mat() {
        let arr1 = Arr2D::from_flat(vec![1, 2, 3, 4, 5, 6], 0, 2, 3).unwrap();
        let arr2 = Arr2D::from_flat(vec![7, 8, 9, 10, 11, 12], 0, 3, 2).unwrap();

        let expected = Arr2D::from_flat(vec![58, 64, 139, 154], 0, 2, 2).unwrap();

        let res = arr1.dot(&arr2).unwrap();
        assert_eq!(res, expected);
    }

    #[test]
    fn test_vec_mul_mat() {
        let arr1 = Arr2D::from_flat(vec![3, 4, 2], 0, 1, 3).unwrap();
        let arr2 = Arr2D::from_flat(vec![13, 9, 7, 15, 8, 7, 4, 6, 6, 4, 0, 3], 0, 3, 4).unwrap();

        let expected = Arr2D::from_flat(vec![83, 63, 37, 75], 0, 1, 4).unwrap();

        let res = arr1.dot(&arr2).unwrap();
        assert_eq!(res, expected);
    }

    #[test]
    fn test_scalar_mul_mat() {
        let scal = Arr2D::from_flat(vec![2], 0, 1, 1).unwrap();
        let mat = Arr2D::from_flat(vec![4, 0, 1, -9], 0, 2, 2).unwrap();

        let expected = Arr2D::from_flat(vec![8, 0, 2, -18], 0, 2, 2).unwrap();

        let res = scal.dot(&mat).unwrap();
        assert_eq!(res, expected);
    }

    #[test]
    fn test_Mul_trait_scalar() {
        let scal = 2;
        let mat = Arr2D::from_flat(vec![4, 0, 1, -9], 0, 2, 2).unwrap();

        let expected = Arr2D::from_flat(vec![8, 0, 2, -18], 0, 2, 2).unwrap();

        let res = mat * scal;
        assert_eq!(res, expected)
    }

    #[test]
    fn test_Mul_trait_mat_mul_mat() {
        let arr1 = Arr2D::from_flat(vec![1, 2, 3, 4, 5, 6], 0, 2, 3).unwrap();
        let arr2 = Arr2D::from_flat(vec![7, 8, 9, 10, 11, 12], 0, 3, 2).unwrap();

        let expected = Arr2D::from_flat(vec![58, 64, 139, 154], 0, 2, 2).unwrap();

        let res = arr1 * arr2;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_Mul_trait_vec_mul_mat() {
        let arr1 = Arr2D::from_flat(vec![3, 4, 2], 0, 1, 3).unwrap();
        let arr2 = Arr2D::from_flat(vec![13, 9, 7, 15, 8, 7, 4, 6, 6, 4, 0, 3], 0, 3, 4).unwrap();

        let expected = Arr2D::from_flat(vec![83, 63, 37, 75], 0, 1, 4).unwrap();

        let res = arr1 * arr2;
        assert_eq!(res, expected);
    }

    // --- misc ---

    #[test]
    fn test_display() {
        let data = Arr2D::from(&[[1.2, 34.5678], [789.02, 0.123]]);
        let out = format!("{}", data);
        let expected = r#"
[[    1.2, 34.5678 ]
 [ 789.02,   0.123 ]]"#;
        assert_eq!(&out, &expected[1..]);
    }
}
