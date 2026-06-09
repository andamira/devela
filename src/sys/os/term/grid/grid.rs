// devela::sys::os::term::grid::grid
//
//! Defines [`TermGrid`].
//

use crate::{Extent2, Order, PhantomData, Position2, TermGridError};

#[doc = crate::_tags!(term data_structure)]
/// A dense row-major grid of terminal-space elements.
#[doc = crate::_doc_meta!{location("sys/os/term/grid")}]
///
/// The grid occupies the first `width × height` elements of its storage.
/// Any remaining storage is retained but lies outside the grid.
///
/// # Methods
///
/// - [Construction and structural access](#construction-and-structural-access)
///   - [new](#method.new).
///   - [extent](#method.extent).
///   - [width](#method.width).
///   - [height](#method.height).
///   - [len](#method.len).
///   - [is_empty](#method.is_empty).
///   - [storage](#method.storage)
///     ([*into*](#method.into_storage), [*slice*](#method.storage_slice)).
///   - [as_slice](#method.as_slice).
///   - [spare_len](#method.spare_len).
///
/// - [Coordinate queries](#coordinate-queries)
///   - [contains](#method.contains).
///   - [index_of](#method.index_of).
///   - [position_of](#method.position_of).
///   - [get](#method.get) ([*xy*](#method.get_xy)).
///   - [row](#method.row).
///
/// - [Mutable access](#mutable-access)
///   - [as_mut_slice](#method.as_mut_slice).
///   - [storage_slice_mut](#method.storage_slice_mut).
///   - [get_mut](#method.get_mut) ([*xy*](#method.get_xy_mut)).
///   - [row_mut](#method.row_mut).
///   - [set](#method.set) ([*xy*](#method.set_xy)).
///
/// - [Copy-oriented operations](#copy-oriented-operations)
///   - [fill](#method.fill).
///   - [get_copy](#method.get_copy) ([*xy*](#method.get_xy_copy)).
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TermGrid<E, S> {
    storage: S,
    extent: Extent2<usize>,
    len: usize,
    _element: PhantomData<fn() -> E>,
}
/// # Construction and structural access
#[rustfmt::skip]
impl<E, S: AsRef<[E]>> TermGrid<E, S> {
    /// Creates a grid over the active prefix of `storage`.
    ///
    /// # Errors
    /// Returns an error if the extent overflows or the storage is too short.
    pub fn new(storage: S, extent: Extent2<usize>) -> Result<Self, TermGridError> {
        let [width, height] = extent.dim;
        let len = width.checked_mul(height).ok_or(TermGridError::ExtentOverflow)?;
        let available = storage.as_ref().len();
        if available < len {
            return Err(TermGridError::NotEnoughElements { required: len, available });
        }
        Ok(Self { storage, extent, len, _element: PhantomData })
    }
    /// Returns the grid extent in columns and rows.
    #[must_use]
    pub const fn extent(&self) -> Extent2<usize> { self.extent }

    /// Returns the number of columns.
    #[must_use]
    pub const fn width(&self) -> usize { self.extent.dim[0] }
    /// Returns the number of rows.
    #[must_use]
    pub const fn height(&self) -> usize { self.extent.dim[1] }

    /// Returns the number of active grid elements.
    #[must_use]
    pub const fn len(&self) -> usize { self.len }
    /// Returns whether the grid contains no active elements.
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.len == 0 }

    /// Returns the complete backing storage.
    #[must_use]
    pub const fn storage(&self) -> &S { &self.storage }
    /// Consumes the grid and returns its backing storage.
    #[must_use]
    pub fn into_storage(self) -> S { self.storage }
    /// Returns the complete backing-storage slice.
    #[must_use]
    pub fn storage_slice(&self) -> &[E] { self.storage.as_ref() }

    /// Returns the active row-major grid elements.
    #[must_use]
    pub fn as_slice(&self) -> &[E] { &self.storage.as_ref()[..self.len] }

    /// Returns the number of elements outside the active grid.
    #[must_use]
    pub fn spare_len(&self) -> usize { self.storage.as_ref().len() - self.len }
}
/// # Coordinate queries
#[rustfmt::skip]
impl<E, S: AsRef<[E]>> TermGrid<E, S> {
    /// Returns whether the position lies within the grid.
    #[must_use]
    pub const fn contains(&self, pos: Position2<usize>) -> bool {
        let [x, y] = pos.dim;
        x < self.width() && y < self.height()
    }
    /// Returns the row-major index of `pos`, or `None` when outside the grid.
    #[must_use]
    pub const fn index_of(&self, pos: Position2<usize>) -> Option<usize> {
        let [x, y] = pos.dim;
        Order::row_major_try_from_2d(x, y, self.width(), self.height())
    }
    /// Returns the position of an active row-major index.
    #[must_use]
    pub const fn position_of(&self, index: usize) -> Option<Position2<usize>> {
        match Order::row_major_try_to_2d(index, self.width(), self.height()) {
            Some((x, y)) => Some(Position2::new([x, y])),
            None => None,
        }
    }
    /// Returns the element at `pos`.
    #[must_use]
    pub fn get(&self, pos: Position2<usize>) -> Option<&E> {
        let index = Order::row_major_try_from_2d(
            pos.dim[0],
            pos.dim[1],
            self.width(),
            self.height(),
        )?;
        self.storage.as_ref().get(index)
    }
    /// Returns the element at the given coordinates.
    #[must_use]
    pub fn get_xy(&self, x: usize, y: usize) -> Option<&E> { self.get(Position2::new([x, y])) }
    /// Returns the active row.
    #[must_use]
    pub fn row(&self, y: usize) -> Option<&[E]> {
        if y >= self.height() { return None; }
        let width = self.width();
        let start = y * width;
        Some(&self.as_slice()[start..start + width])
    }
}
/// # Mutable access
#[rustfmt::skip]
impl<E, S: AsRef<[E]> + AsMut<[E]>> TermGrid<E, S> {
    /// Returns the active row-major grid elements exclusively.
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [E] {
        &mut self.storage.as_mut()[..self.len]
    }
    /// Returns the complete backing-storage slice exclusively.
    #[must_use]
    pub fn storage_slice_mut(&mut self) -> &mut [E] {
        self.storage.as_mut()
    }
    /// Returns the element at `pos` exclusively.
    #[must_use]
    pub fn get_mut(&mut self, pos: Position2<usize>) -> Option<&mut E> {
        let index = Order::row_major_try_from_2d(
            pos.dim[0],
            pos.dim[1],
            self.width(),
            self.height(),
        )?;
        self.storage.as_mut().get_mut(index)
    }
    /// Returns the element at the given coordinates exclusively.
    #[must_use]
    pub fn get_xy_mut(&mut self, x: usize, y: usize) -> Option<&mut E> {
        self.get_mut(Position2::new([x, y]))
    }
    /// Returns the active row exclusively.
    #[must_use]
    pub fn row_mut(&mut self, y: usize) -> Option<&mut [E]> {
        if y >= self.height() { return None; }
        let width = self.width();
        let start = y * width;
        Some(&mut self.as_mut_slice()[start..start + width])
    }
    /// Replaces the element at `pos`.
    ///
    /// Returns `false` when the position lies outside the grid.
    pub fn set(&mut self, pos: Position2<usize>, element: E) -> bool {
        match self.get_mut(pos) {
            Some(dst) => { *dst = element; true }
            None => false,
        }
    }
    /// Replaces the element at the given coordinates.
    ///
    /// Returns `false` when the coordinates lie outside the grid.
    pub fn set_xy(&mut self, x: usize, y: usize, element: E) -> bool {
        self.set(Position2::new([x, y]), element)
    }
}

/// # Copy-oriented operations
impl<E: Copy, S: AsRef<[E]> + AsMut<[E]>> TermGrid<E, S> {
    /// Fills the active grid with `element`.
    pub fn fill(&mut self, element: E) {
        self.as_mut_slice().fill(element);
    }

    /// Returns a copy of the element at `pos`.
    #[must_use]
    pub fn get_copy(&self, pos: Position2<usize>) -> Option<E> {
        self.get(pos).copied()
    }

    /// Returns a copy of the element at the given coordinates.
    #[must_use]
    pub fn get_xy_copy(&self, x: usize, y: usize) -> Option<E> {
        self.get_xy(x, y).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ext, pos};

    #[test]
    fn construction_and_geometry() {
        let storage = [0u8; 16];
        let grid = TermGrid::new(storage, ext!(3usize, 4usize)).unwrap();
        assert_eq!(grid.extent(), ext!(3, 4));
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 4);
        assert_eq!(grid.len(), 12);
        assert_eq!(grid.spare_len(), 4);
        assert_eq!(grid.as_slice(), &[0; 12]);
    }
    #[test]
    fn insufficient_storage() {
        let error = TermGrid::new([0u8; 5], ext!(3usize, 2usize)).unwrap_err();
        assert_eq!(error, TermGridError::NotEnoughElements { required: 6, available: 5 },);
    }
    #[test]
    fn coordinate_mapping() {
        let grid = TermGrid::new([0u8; 12], ext!(4usize, 3usize)).unwrap();
        assert_eq!(grid.index_of(pos!(0usize, 0usize)), Some(0));
        assert_eq!(grid.index_of(pos!(3usize, 2usize)), Some(11));
        assert_eq!(grid.index_of(pos!(4usize, 2usize)), None);
        assert_eq!(grid.position_of(0), Some(pos!(0usize, 0usize)));
        assert_eq!(grid.position_of(11), Some(pos!(3usize, 2usize)));
        assert_eq!(grid.position_of(12), None);
    }
    #[test]
    fn access_and_rows() {
        let mut grid = TermGrid::new([0, 1, 2, 3, 4, 5], ext!(3usize, 2usize)).unwrap();
        assert_eq!(grid.get_xy(1, 0), Some(&1));
        assert_eq!(grid.get_xy(2, 1), Some(&5));
        assert_eq!(grid.get_xy(3, 0), None);
        assert_eq!(grid.row(0), Some(&[0, 1, 2][..]));
        assert_eq!(grid.row(1), Some(&[3, 4, 5][..]));
        assert_eq!(grid.row(2), None);
        assert!(grid.set_xy(1, 1, 9));
        assert!(!grid.set_xy(3, 1, 9));
        assert_eq!(grid.row(1), Some(&[3, 9, 5][..]));
    }
    #[test]
    fn fill_only_affects_active_grid() {
        let mut grid = TermGrid::new([1u8; 8], ext!(3usize, 2usize)).unwrap();
        grid.fill(7);
        assert_eq!(grid.as_slice(), &[7; 6]);
        assert_eq!(grid.storage_slice(), &[7, 7, 7, 7, 7, 7, 1, 1]);
    }
}
