// devela::sys::os::term::grid::impls::core
//
//! Core methods for `TermGrid`.
//
// - Construction and structural access
// - Mutable Access
// - Copy oriented operations

use crate::{Extent2, Order, PhantomData, Position2, TermGrid, TermGridError};

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
    pub fn set(&mut self, pos: Position2<usize>, element: impl Into<E>) -> bool {
        match self.get_mut(pos) {
            Some(dst) => { *dst = element.into(); true }
            None => false,
        }
    }
    /// Replaces the element at the given coordinates.
    ///
    /// Returns `false` when the coordinates lie outside the grid.
    pub fn set_xy(&mut self, x: usize, y: usize, element: impl Into<E>) -> bool {
        self.set(Position2::new([x, y]), element.into())
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
