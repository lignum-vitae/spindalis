use std::ops::Index;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Arr2D<T> {
    inner: Vec<T>,
    height: usize,
    width: usize,
}

#[derive(Debug)]
pub enum Arr2DError {
    InconsistentRowLengths,
    InvalidReshape { size: usize, new_height: usize },
}

impl<T> Arr2D<T> {
    /// Change the height and width.
    ///
    /// # Errors
    ///
    /// This function will return an error if the size isn't divislbe by the new height.
    pub fn reshape(&mut self, height: usize) -> Result<(), Arr2DError> {
        let size = self.height * self.width;
        if size % height != 0 {
            return Err(Arr2DError::InvalidReshape {
                size,
                new_height: height,
            });
        }

        self.height = height;
        self.width = size / height;
        Ok(())
    }

    /// Element-wise map operatiorn.
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

    /// Create an iterator of refs to rows.
    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        Arr2DRows {
            data: &self.inner,
            width: self.width,
            remaining: self.height,
        }
    }

    /// Create an iterator of mut refs to rows.
    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        Arr2DRowsMut {
            data: self.inner.as_mut_slice(),
            width: self.width,
            remaining: self.height,
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
        // because `self.data` would be alive only during an invokation of this method
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

// Convert a nested Vec to Arr2D.
impl<T> TryFrom<Vec<Vec<T>>> for Arr2D<T> {
    type Error = Arr2DError;

    fn try_from(values: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let mut it = values.into_iter();
        if let Some(first_row) = it.next() {
            let width = first_row.len();
            let mut height = 1;
            let mut inner = first_row;

            for row in it {
                if row.len() != width {
                    return Err(Arr2DError::InconsistentRowLengths);
                }
                inner.extend(row);
                height += 1;
            }

            Ok(Self {
                inner,
                width,
                height,
            })
        } else {
            Ok(Self {
                inner: vec![],
                width: 0,
                height: 0,
            })
        }
    }
}

// Convert a nested array like `&[[1, 2], [3, 4]]` to Arr2D.
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

// Allow indexing Arr2D items like `arr[(0, 1)]`.
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

// Allow indexing Arr2D rows like `arr[1]`.
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_arr_from_vec() {
        let data = Arr2D::try_from(vec![vec![1, 2, 3], vec![6, 5, 4]]).unwrap();
        let expected = Arr2D::from(&[[1, 2, 3], [6, 5, 4]]);
        assert_eq!(data, expected);
    }

    #[test]
    fn test_indexing_row() {
        let data = Arr2D::from(&[[1, 2, 3], [6, 5, 4]]);
        assert_eq!(data[0], [1, 2, 3]);
        assert_eq!(data[1], [6, 5, 4]);
    }

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
    fn test_rows_iterator_returns_slices() {
        let data = Arr2D::from(&[[1, 2, 3], [4, 5, 6]]);
        let rows: Vec<&[i32]> = data.rows().collect();

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], &[1, 2, 3]);
        assert_eq!(rows[1], &[4, 5, 6]);
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

    #[test]
    #[should_panic]
    fn test_indexing_out_of_bounds_panics() {
        let data = Arr2D::from(&[[1, 2, 3], [4, 5, 6]]);

        let _ = data[(2, 0)];
    }

    #[test]
    #[should_panic]
    fn test_row_index_out_of_bounds_panics() {
        let data = Arr2D::from(&[[1, 2, 3], [4, 5, 6]]);

        let _ = &data[2];
    }
}
