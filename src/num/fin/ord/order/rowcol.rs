// devela/src/num/fin/ord/order/rowcol.rs
//
//! Dense row-major and column-major ordinal encodings.
//!
//! Coordinates and dimensions use spatial axis order:
//! `[x, y, z, ...]` and `[width, height, depth, ...]`.
//!
//! Row-major order keeps the first axis contiguous.
//! Column-major order keeps the last axis contiguous.
//

use crate::{Order, is, unwrap, whilst};

/* row-major */

/// # Row-major ordinal encodings
///
/// The first coordinate axis changes fastest.
impl Order {
    /// Encodes 2D coordinates in row-major order.
    ///
    /// Does not validate coordinates or arithmetic overflow.
    #[must_use]
    #[inline]
    pub const fn row_major_from_2d(x: usize, y: usize, width: usize) -> usize {
        y * width + x
    }

    /// Encodes bounded 2D coordinates in row-major order.
    ///
    /// Returns `None` if the coordinates are out of bounds or the grid length
    /// cannot be represented by `usize`.
    #[must_use]
    pub const fn row_major_try_from_2d(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Option<usize> {
        is! { x >= width || y >= height, return None }
        let _len = unwrap![some? width.checked_mul(height)];
        Some(Self::row_major_from_2d(x, y, width))
    }

    /// Decodes a row-major ordinal into 2D coordinates.
    ///
    /// Does not validate the ordinal. `width` must be nonzero.
    #[must_use]
    #[inline]
    pub const fn row_major_to_2d(i: usize, width: usize) -> (usize, usize) {
        (i % width, i / width)
    }

    /// Decodes a bounded row-major ordinal into 2D coordinates.
    ///
    /// Returns `None` if the grid is empty, its length overflows, or `i` is
    /// outside the grid.
    #[must_use]
    pub const fn row_major_try_to_2d(
        i: usize,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)> {
        is! { width == 0 || height == 0, return None }
        let len = unwrap![some? width.checked_mul(height)];
        is! { i >= len, return None }
        Some(Self::row_major_to_2d(i, width))
    }

    /// Encodes 3D coordinates in row-major order.
    ///
    /// Does not validate coordinates or arithmetic overflow.
    #[must_use]
    #[inline]
    pub const fn row_major_from_3d(
        x: usize,
        y: usize,
        z: usize,
        width: usize,
        height: usize,
    ) -> usize {
        (z * height + y) * width + x
    }

    /// Encodes bounded 3D coordinates in row-major order.
    ///
    /// Returns `None` if the coordinates are out of bounds or the grid length
    /// cannot be represented by `usize`.
    #[must_use]
    pub const fn row_major_try_from_3d(
        x: usize,
        y: usize,
        z: usize,
        width: usize,
        height: usize,
        depth: usize,
    ) -> Option<usize> {
        is! { x >= width || y >= height || z >= depth,return None }
        let area = unwrap![some? width.checked_mul(height)];
        let _len = unwrap![some? area.checked_mul(depth)];
        Some(Self::row_major_from_3d(x, y, z, width, height))
    }

    /// Decodes a row-major ordinal into 3D coordinates.
    ///
    /// Does not validate the ordinal. `width` and `height` must be nonzero.
    #[must_use]
    #[inline]
    pub const fn row_major_to_3d(i: usize, width: usize, height: usize) -> (usize, usize, usize) {
        let area = width * height;
        let z = i / area;
        let rem = i % area;
        let y = rem / width;
        let x = rem % width;
        (x, y, z)
    }

    /// Decodes a bounded row-major ordinal into 3D coordinates.
    #[must_use]
    pub const fn row_major_try_to_3d(
        i: usize,
        width: usize,
        height: usize,
        depth: usize,
    ) -> Option<(usize, usize, usize)> {
        is! { width == 0 || height == 0 || depth == 0, return None }
        let area = unwrap![some? width.checked_mul(height)];
        let len = unwrap![some? area.checked_mul(depth)];
        is! { i >= len, return None }
        Some(Self::row_major_to_3d(i, width, height))
    }
}

/* column-major */

/// # Column-major ordinal encodings
///
/// The last coordinate axis changes fastest.
impl Order {
    /// Encodes 2D coordinates in column-major order.
    ///
    /// Does not validate coordinates or arithmetic overflow.
    #[must_use]
    #[inline]
    pub const fn col_major_from_2d(x: usize, y: usize, height: usize) -> usize {
        x * height + y
    }

    /// Encodes bounded 2D coordinates in column-major order.
    #[must_use]
    pub const fn col_major_try_from_2d(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Option<usize> {
        is! { x >= width || y >= height, return None }
        let _len = unwrap![some? width.checked_mul(height)];
        Some(Self::col_major_from_2d(x, y, height))
    }

    /// Decodes a column-major ordinal into 2D coordinates.
    ///
    /// Does not validate the ordinal. `height` must be nonzero.
    #[must_use]
    #[inline]
    pub const fn col_major_to_2d(i: usize, height: usize) -> (usize, usize) {
        (i / height, i % height)
    }

    /// Decodes a bounded column-major ordinal into 2D coordinates.
    #[must_use]
    pub const fn col_major_try_to_2d(
        i: usize,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)> {
        is! { width == 0 || height == 0, return None }
        let len = unwrap![some? width.checked_mul(height)];
        is! { i >= len, return None }
        Some(Self::col_major_to_2d(i, height))
    }

    /// Encodes 3D coordinates in column-major order.
    ///
    /// Does not validate coordinates or arithmetic overflow.
    #[must_use]
    #[inline]
    pub const fn col_major_from_3d(
        x: usize,
        y: usize,
        z: usize,
        height: usize,
        depth: usize,
    ) -> usize {
        (x * height + y) * depth + z
    }

    /// Encodes bounded 3D coordinates in column-major order.
    #[must_use]
    pub const fn col_major_try_from_3d(
        x: usize,
        y: usize,
        z: usize,
        width: usize,
        height: usize,
        depth: usize,
    ) -> Option<usize> {
        is! { x >= width || y >= height || z >= depth, return None }
        let area = unwrap![some? height.checked_mul(depth)];
        let _len = unwrap![some? width.checked_mul(area)];
        Some(Self::col_major_from_3d(x, y, z, height, depth))
    }

    /// Decodes a column-major ordinal into 3D coordinates.
    ///
    /// Does not validate the ordinal. `height` and `depth` must be nonzero.
    #[must_use]
    #[inline]
    pub const fn col_major_to_3d(i: usize, height: usize, depth: usize) -> (usize, usize, usize) {
        let area = height * depth;
        let x = i / area;
        let rem = i % area;
        let y = rem / depth;
        let z = rem % depth;
        (x, y, z)
    }

    /// Decodes a bounded column-major ordinal into 3D coordinates.
    #[must_use]
    pub const fn col_major_try_to_3d(
        i: usize,
        width: usize,
        height: usize,
        depth: usize,
    ) -> Option<(usize, usize, usize)> {
        is! { width == 0 || height == 0 || depth == 0, return None }
        let area = unwrap![some? height.checked_mul(depth)];
        let len = unwrap![some? width.checked_mul(area)];
        is! { i >= len, return None }
        Some(Self::col_major_to_3d(i, height, depth))
    }
}

/* N-dimensional */

/// # N-dimensional ordinal encodings
impl Order {
    /// Returns the number of coordinates in the grid.
    ///
    /// Returns `None` if the product overflows `usize`.
    ///
    /// An empty dimension makes the grid empty. A zero-dimensional grid has
    /// one coordinate, following the empty-product convention.
    #[must_use]
    pub const fn grid_volume<const N: usize>(dims: [usize; N]) -> Option<usize> {
        // The mathematical product is zero regardless of the other dimensions.
        whilst! { k in 0..N; {
            is! { dims[k] == 0, return Some(0) }
        }}
        let mut volume = 1_usize;
        whilst! { k in 0..N; {
            volume = unwrap![some? volume.checked_mul(dims[k])];
        }}
        Some(volume)
    }

    /// Computes row-major strides.
    ///
    /// The returned first stride is `1`.
    ///
    /// For `[width, height, depth]`, this returns
    /// `[1, width, width * height]`.
    #[must_use]
    pub const fn row_major_strides<const N: usize>(dims: [usize; N]) -> Option<[usize; N]> {
        let mut strides = [1_usize; N];
        whilst! { k in 1..N; {
            strides[k] = unwrap![some? strides[k - 1].checked_mul(dims[k - 1])];
        }}
        Some(strides)
    }

    /// Computes column-major strides.
    ///
    /// The returned last stride is `1`.
    ///
    /// For `[width, height, depth]`, this returns
    /// `[height * depth, depth, 1]`.
    #[must_use]
    pub const fn col_major_strides<const N: usize>(dims: [usize; N]) -> Option<[usize; N]> {
        let mut strides = [1_usize; N];
        whilst! { k in rev 1..N; {
            strides[k - 1] = unwrap![some? strides[k].checked_mul(dims[k])];
        }}
        Some(strides)
    }

    /// Encodes N-dimensional coordinates in row-major order.
    ///
    /// Does not validate coordinates, dimensions, or arithmetic overflow.
    #[must_use]
    pub const fn row_major_from<const N: usize>(coords: [usize; N], dims: [usize; N]) -> usize {
        let mut i = 0;
        whilst! { k in rev 0..N; {
            i = i * dims[k] + coords[k];
        }}
        i
    }

    /// Encodes bounded N-dimensional coordinates in row-major order.
    #[must_use]
    pub const fn row_major_try_from<const N: usize>(
        coords: [usize; N],
        dims: [usize; N],
    ) -> Option<usize> {
        whilst! { k in 0..N; {
            is! { coords[k] >= dims[k], return None }
        }}
        let _volume = unwrap![some? Self::grid_volume(dims)];
        Some(Self::row_major_from(coords, dims))
    }

    /// Decodes a row-major ordinal into N-dimensional coordinates.
    ///
    /// Does not validate the ordinal. Every dimension must be nonzero.
    #[must_use]
    pub const fn row_major_to<const N: usize>(mut i: usize, dims: [usize; N]) -> [usize; N] {
        let mut coords = [0; N];
        whilst! { k in 0..N; {
            coords[k] = i % dims[k];
            i /= dims[k];
        }}
        coords
    }

    /// Decodes a bounded row-major ordinal into N-dimensional coordinates.
    #[must_use]
    pub const fn row_major_try_to<const N: usize>(
        i: usize,
        dims: [usize; N],
    ) -> Option<[usize; N]> {
        let volume = unwrap![some? Self::grid_volume(dims)];
        is! { i >= volume, return None }
        Some(Self::row_major_to(i, dims))
    }

    /// Encodes N-dimensional coordinates in column-major order.
    ///
    /// Does not validate coordinates, dimensions, or arithmetic overflow.
    #[must_use]
    pub const fn col_major_from<const N: usize>(coords: [usize; N], dims: [usize; N]) -> usize {
        let mut i = 0;
        whilst! { k in 0..N; {
            i = i * dims[k] + coords[k];
        }}
        i
    }

    /// Encodes bounded N-dimensional coordinates in column-major order.
    #[must_use]
    pub const fn col_major_try_from<const N: usize>(
        coords: [usize; N],
        dims: [usize; N],
    ) -> Option<usize> {
        whilst! { k in 0..N; {
            if coords[k] >= dims[k] {
                return None;
            }
        }}
        let _volume = unwrap![some? Self::grid_volume(dims)];
        Some(Self::col_major_from(coords, dims))
    }

    /// Decodes a column-major ordinal into N-dimensional coordinates.
    ///
    /// Does not validate the ordinal. Every dimension must be nonzero.
    #[must_use]
    pub const fn col_major_to<const N: usize>(mut i: usize, dims: [usize; N]) -> [usize; N] {
        let mut coords = [0; N];
        whilst! { k in rev 0..N; {
            coords[k] = i % dims[k];
            i /= dims[k];
        }}
        coords
    }

    /// Decodes a bounded column-major ordinal into N-dimensional coordinates.
    #[must_use]
    pub const fn col_major_try_to<const N: usize>(
        i: usize,
        dims: [usize; N],
    ) -> Option<[usize; N]> {
        let volume = unwrap![some? Self::grid_volume(dims)];
        is! { i >= volume, return None }
        Some(Self::col_major_to(i, dims))
    }
}

#[cfg(test)]
mod tests {
    use crate::Order;
    #[test]
    fn row_major_2d_roundtrip() {
        let (width, height) = (4, 3);
        for y in 0..height {
            for x in 0..width {
                let i = Order::row_major_from_2d(x, y, width);
                assert_eq!(Order::row_major_to_2d(i, width), (x, y));
                assert_eq!(Order::row_major_try_from_2d(x, y, width, height), Some(i),);
                assert_eq!(Order::row_major_try_to_2d(i, width, height), Some((x, y)),);
            }
        }
    }
    #[test]
    fn col_major_2d_roundtrip() {
        let (width, height) = (4, 3);
        for x in 0..width {
            for y in 0..height {
                let i = Order::col_major_from_2d(x, y, height);
                assert_eq!(Order::col_major_to_2d(i, height), (x, y));
                assert_eq!(Order::col_major_try_from_2d(x, y, width, height), Some(i),);
                assert_eq!(Order::col_major_try_to_2d(i, width, height), Some((x, y)),);
            }
        }
    }
    #[test]
    fn major_3d_roundtrip() {
        let (width, height, depth) = (4, 3, 2);
        let len = width * height * depth;
        for i in 0..len {
            let row = Order::row_major_to_3d(i, width, height);
            assert_eq!(Order::row_major_from_3d(row.0, row.1, row.2, width, height), i,);
            let col = Order::col_major_to_3d(i, height, depth);
            assert_eq!(Order::col_major_from_3d(col.0, col.1, col.2, height, depth), i,);
        }
        assert_eq!(Order::row_major_try_from_3d(2, 1, 1, width, height, depth), Some(18),);
        assert_eq!(Order::col_major_try_from_3d(2, 1, 1, width, height, depth), Some(15),);
    }
    #[test]
    fn generic_matches_fixed() {
        let dims = [4, 3, 2];
        assert_eq!(Order::row_major_strides(dims), Some([1, 4, 12]),);
        assert_eq!(Order::col_major_strides(dims), Some([6, 2, 1]),);
        assert_eq!(Order::row_major_from([2, 1, 1], dims), Order::row_major_from_3d(2, 1, 1, 4, 3),);
        assert_eq!(Order::col_major_from([2, 1, 1], dims), Order::col_major_from_3d(2, 1, 1, 3, 2),);
        for i in 0..24 {
            let row = Order::row_major_to(i, dims);
            let col = Order::col_major_to(i, dims);
            assert_eq!(Order::row_major_from(row, dims), i);
            assert_eq!(Order::col_major_from(col, dims), i);
        }
    }
    #[test]
    fn checked_boundaries() {
        assert_eq!(Order::row_major_try_from_2d(4, 0, 4, 3), None);
        assert_eq!(Order::row_major_try_to_2d(12, 4, 3), None);
        assert_eq!(Order::col_major_try_to_2d(0, 4, 0), None);
        assert_eq!(Order::grid_volume([usize::MAX, 2]), None);
        assert_eq!(Order::grid_volume([usize::MAX, 2, 0]), Some(0),);
        assert_eq!(Order::row_major_try_from([], []), Some(0));
        assert_eq!(Order::row_major_try_to(0, []), Some([]));
        assert_eq!(Order::row_major_try_to(1, []), None);
    }
}
