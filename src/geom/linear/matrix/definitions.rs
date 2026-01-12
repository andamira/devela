// devela::geom::alg::matrix::definitions
//
//! Defines [`Matrix`].
//

#[doc = crate::_tags!(geom)]
/// A static `R×C` shaped matrix backed by an array.
#[doc = crate::_doc_location!("num/geom/linear")]
///
/// - `T` is the type of elements in the matrix.
/// - `R` and `C` represent the dimensions in terms of columns and rows.
/// - `LEN` is the total number of elements, and the product of `C` and `R`.
/// - `RMAJ` indicates if the storage is row-major (`true`) or column-major (`false`).
/// - `MAX_LEN_DET` is the maximum matrix length for calculating the determinant for
///   square matrices of dimension > 3.
#[derive(Debug)]
pub struct Matrix<
    T,
    const R: usize,
    const C: usize,
    const LEN: usize,
    const RMAJ: bool = true,
    const MAX_LEN_DET: usize = { 4 * 4 },
> {
    /// Internal storage of matrix elements in a fixed-size array.
    ///
    /// - Stored in row-major or column-major order depending on `RMAJ`.
    /// - Indexed using `(row, col)` accessor methods.
    pub data: [T; LEN],
}
