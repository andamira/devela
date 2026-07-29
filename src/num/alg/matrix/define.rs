// devela/src/num/alg/matrix/define.rs
//
//! Defines [`Matrix`].
//

#[doc = crate::_tags!(lin)]
/// A static `R × C` matrix backed by a contiguous row-major array.
#[doc = crate::_doc_meta! { location("num/alg") }]
/// `R` is the number of rows and `C` is the number of columns.
///
/// Elements are stored in row-major order: all columns of the first row,
/// followed by all columns of the second row, and so on.
///
/// `LEN` is the number of stored elements and must equal `R * C`. It is an
/// explicit parameter because stable Rust cannot yet generally use `R * C`
/// as an array length involving outer const parameters.
///
/// The shape relationship is validated by its constructors.
///
/// # Coordinates
///
/// Element coordinates use `(row, column)` order.
/// ```
/// # use devela::Matrix;
/// let matrix = Matrix::<_, 2, 3, 6>::new([
///     1, 2, 3,
///     4, 5, 6,
/// ]);
/// assert_eq!(matrix[(0, 2)], 3);
/// assert_eq!(matrix[(1, 0)], 4);
/// ```
#[must_use]
#[non_exhaustive]
#[repr(transparent)]
pub struct Matrix<T, const R: usize, const C: usize, const LEN: usize> {
    /// The matrix elements in contiguous row-major order.
    pub data: [T; LEN],
}

/* utility traits */

mod impl_traits {
    use crate::{ConstInit, Debug, FmtResult, Formatter, Hash, Hasher, Matrix, init_array};

    impl<T: Clone, const R: usize, const C: usize, const LEN: usize> Clone for Matrix<T, R, C, LEN> {
        fn clone(&self) -> Self {
            Self::new(self.data.clone())
        }
    }
    impl<T: Copy, const R: usize, const C: usize, const LEN: usize> Copy for Matrix<T, R, C, LEN> {}
    impl<T: Default, const R: usize, const C: usize, const LEN: usize> Default
        for Matrix<T, R, C, LEN>
    {
        /// Returns a matrix filled with each element's default value.
        fn default() -> Self {
            Self::new(init_array![default [T; LEN], "safe_num", "unsafe_array"])
        }
    }
    impl<T: ConstInit, const R: usize, const C: usize, const LEN: usize> ConstInit
        for Matrix<T, R, C, LEN>
    {
        /// A matrix filled with each element's initial value.
        const INIT: Self = Self::new(init_array![INIT in ConstInit [T; LEN]]);
    }
    impl<T: Debug, const R: usize, const C: usize, const LEN: usize> Debug for Matrix<T, R, C, LEN> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            f.debug_struct("Matrix")
                .field("rows", &R)
                .field("columns", &C)
                .field("data", &self.data) // IMPROVE: omit the middle for large LEN
                .finish()
        }
    }
    impl<T: PartialEq, const R: usize, const C: usize, const LEN: usize> PartialEq
        for Matrix<T, R, C, LEN>
    {
        fn eq(&self, other: &Self) -> bool {
            self.data == other.data
        }
    }
    impl<T: Eq, const R: usize, const C: usize, const LEN: usize> Eq for Matrix<T, R, C, LEN> {}
    impl<T: Hash, const R: usize, const C: usize, const LEN: usize> Hash for Matrix<T, R, C, LEN> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.data.hash(state);
        }
    }
    impl<T, const R: usize, const C: usize, const LEN: usize> AsRef<[T]> for Matrix<T, R, C, LEN> {
        fn as_ref(&self) -> &[T] {
            &self.data
        }
    }
    impl<T, const R: usize, const C: usize, const LEN: usize> AsMut<[T]> for Matrix<T, R, C, LEN> {
        fn as_mut(&mut self) -> &mut [T] {
            &mut self.data
        }
    }
}
