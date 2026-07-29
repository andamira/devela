// devela/src/num/alg/matrix/methods.rs
//
//! Construction, shape queries, and element access for [`Matrix`].
//

use crate::{Index, IndexMut, Matrix, Order, whilst};

/* general methods */

impl<T, const R: usize, const C: usize, const LEN: usize> Matrix<T, R, C, LEN> {
    /* dimensions */

    /// The number of matrix rows.
    pub const ROWS: usize = R;

    /// The number of matrix columns.
    pub const COLUMNS: usize = C;

    /// The number of stored elements.
    pub const ELEMENTS: usize = LEN;

    /* construction */

    /// Creates a matrix from its contiguous row-major elements.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// - `R * C` cannot be represented by `usize`, or
    /// - `LEN != R * C`.
    pub const fn new(data: [T; LEN]) -> Self {
        Self::assert_valid_shape();
        Self { data }
    }

    /* shape */

    /// Returns the number of matrix rows.
    #[must_use]
    pub const fn row_count(&self) -> usize {
        R
    }

    /// Returns the number of matrix columns.
    #[must_use]
    pub const fn column_count(&self) -> usize {
        C
    }

    /// Returns the number of stored elements.
    #[must_use]
    pub const fn len(&self) -> usize {
        LEN
    }

    /// Returns whether the matrix contains no elements.
    ///
    /// For a valid matrix this is equivalent to either dimension being zero.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        LEN == 0
    }

    /// Returns whether the matrix has the same number of rows and columns.
    #[must_use]
    pub const fn is_square(&self) -> bool {
        R == C
    }

    /* coordinate mapping */

    /// Returns the flat row-major index of `(row, column)`.
    ///
    /// Returns `None` when either coordinate is out of bounds or when the
    /// declared dimensions cannot be multiplied without overflow.
    ///
    /// Matrix coordinates map to [`Order`] coordinates as:
    ///
    /// - `column → x`,
    /// - `row → y`,
    /// - `C → width`.
    #[must_use]
    pub const fn get_index(row: usize, column: usize) -> Option<usize> {
        Order::row_major_try_from_2d(column, row, C, R)
    }

    /* checked element access */

    /// Returns a shared reference to the element at `(row, column)`.
    ///
    /// Returns `None` when either coordinate is out of bounds.
    #[must_use]
    pub const fn get(&self, row: usize, column: usize) -> Option<&T> {
        match Self::get_index(row, column) {
            Some(index) => Some(&self.data[index]),
            None => None,
        }
    }

    /// Returns an exclusive reference to the element at `(row, column)`.
    ///
    /// Returns `None` when either coordinate is out of bounds.
    pub const fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        match Self::get_index(row, column) {
            Some(index) => Some(&mut self.data[index]),
            None => None,
        }
    }

    /* panicking element access */

    /// Returns a shared reference to the element at `(row, column)`.
    ///
    /// # Panics
    ///
    /// Panics when either coordinate is out of bounds.
    #[must_use]
    pub const fn at_ref(&self, row: usize, column: usize) -> &T {
        match self.get(row, column) {
            Some(value) => value,
            None => panic!("matrix index out of bounds"),
        }
    }

    /// Returns an exclusive reference to the element at `(row, column)`.
    ///
    /// # Panics
    ///
    /// Panics when either coordinate is out of bounds.
    #[must_use]
    pub const fn at_mut(&mut self, row: usize, column: usize) -> &mut T {
        match self.get_mut(row, column) {
            Some(value) => value,
            None => panic!("matrix index out of bounds"),
        }
    }

    /* invariant */

    /// Verifies the relationship between the dimensions and backing length.
    const fn assert_valid_shape() {
        match R.checked_mul(C) {
            Some(expected_len) => assert!(LEN == expected_len, "matrix LEN must equal R * C"),
            None => panic!("matrix dimensions overflow usize"),
        }
    }
}

/* copied element access */

impl<T: Copy, const R: usize, const C: usize, const LEN: usize> Matrix<T, R, C, LEN> {
    /// Returns a copy of the element at `(row, column)`.
    ///
    /// # Panics
    ///
    /// Panics when either coordinate is out of bounds.
    #[must_use]
    pub const fn at(&self, row: usize, column: usize) -> T {
        *self.at_ref(row, column)
    }
    /// Returns a matrix whose elements are all `value`.
    pub const fn splat(value: T) -> Self {
        Self::new([value; LEN])
    }
    /// Returns the transpose of this matrix.
    ///
    /// Rows become columns while preserving the same number of elements:
    ///
    /// `Matrix<T, R, C, LEN> → Matrix<T, C, R, LEN>`.
    pub const fn transpose(&self) -> Matrix<T, C, R, LEN> {
        let mut data = self.data;
        whilst! { row in 0..R; {
            whilst! { col in 0..C; {
                let source = row * C + col;
                let dest = col * R + row;
                data[dest] = self.data[source];
            }}
        }}
        Matrix::<T, C, R, LEN>::new(data)
    }
}

/* indexing traits */

impl<T, const R: usize, const C: usize, const LEN: usize> Index<(usize, usize)>
    for Matrix<T, R, C, LEN>
{
    type Output = T;

    /// Returns the element at `(row, column)`.
    ///
    /// # Panics
    ///
    /// Panics when either coordinate is out of bounds.
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.at_ref(index.0, index.1)
    }
}

impl<T, const R: usize, const C: usize, const LEN: usize> IndexMut<(usize, usize)>
    for Matrix<T, R, C, LEN>
{
    /// Returns the element at `(row, column)` mutably.
    ///
    /// # Panics
    ///
    /// Panics when either coordinate is out of bounds.
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.at_mut(index.0, index.1)
    }
}
