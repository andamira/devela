// devela::data::list::array::d2::methods::coords
//
//! 2-dimensional array coordinates and indexing methods
//

use crate::{Array2d, IndexOutOfBounds, Storage, is};

/// Helper macro for implementing common methods generic on storage order.
macro_rules! impl_maj {
    () => {
        impl_maj![true:r:"row", false:c:"column"];
        impl_maj![false:c:"column", true:r:"row"];
    };

    // $RMAJ:   true for row-major order
    // $D1:     the major dimension name (not used)
    // $D1long: the major dimension long name
    // $D2:     the other dimension name
    // $D2long: the other dimension long name
    // $CMAJ:   false for row-major order
    ( $RMAJ:literal:$D1:ident:$D1long:literal,
      $CMAJ:literal:$D2:ident:$D2long:literal) => { $crate::paste! {

        // T, S
        #[doc = "# Single element indexing (" $D1long "-major order)"]
        impl<T, const C: usize, const R: usize, const CR: usize, S: Storage>
            Array2d<T, C, R, CR, $RMAJ, S>
        {
            /* get_ref */

            /// Returns a reference to the element at the given 2D coordinates
            #[doc = "in the current " $D1long "-major order."]
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if the coordinates are out of bounds.
            pub fn get_ref(&self, col_row: [usize; 2]) -> Result<&T, IndexOutOfBounds> {
                Self::get_index(col_row).map(|idx| &self.data[idx])
            }
            /// Returns a reference to the element at the given 2D coordinates
            #[doc = "in the current " $D1long "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            #[must_use]
            pub fn get_ref_unchecked(&self, col_row: [usize; 2]) -> &T {
                &self.data.data[Self::get_index_unchecked(col_row)]
            }

            /* get_mut */

            /// Returns an exclusive reference to the element at the given 2D coordinates
            #[doc = "in the current " $D1long "-major order."]
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if the coordinates are out of bounds.
            pub fn get_mut(&mut self, col_row: [usize; 2]) -> Result<&mut T, IndexOutOfBounds> {
                Self::get_index(col_row).map(|idx| &mut self.data[idx])
            }
            /// Returns an exclusive reference to the element at the given 2D coordinates
            #[doc = "in the current " $D1long "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            #[must_use]
            pub fn get_mut_unchecked(&mut self, col_row: [usize; 2]) -> &mut T {
                &mut self.data.data[Self::get_index_unchecked(col_row)]
            }

            /* set */

            /// Sets the element at the given 2D coordinates
            #[doc = "in the current " $D1long "-major order."]
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if the coordinates are out of bounds.
            pub fn set(&mut self, element: T, col_row: [usize; 2]) -> Result<(), IndexOutOfBounds> {
                self.get_mut(col_row).map(|e| {
                    *e = element;
                })
            }
            /// Sets the element at the given 2D coordinates
            #[doc = "in the current " $D1long "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            pub fn set_unchecked(&mut self, element: T, col_row: [usize; 2]) {
                let e = self.get_mut_unchecked(col_row);
                *e = element;
            }
        }

        // T: Clone, S
        // # Single element clone
        impl<T: Clone, const C: usize, const R: usize, const CR: usize, S: Storage>
            Array2d<T, C, R, CR, $RMAJ, S>
        {
            /* get */

            /// Returns a clone of the element at the given 2D coordinates
            #[doc = "in the current " $D1long "-major order."]
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if the coordinates are out of bounds.
            pub fn get(&self, col_row: [usize; 2]) -> Result<T, IndexOutOfBounds> {
                Self::get_index(col_row).map(|idx| self.data[idx].clone())
            }
            /// Returns a clone of the element at the given 2D coordinates
            #[doc = "in the current " $D1long "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            #[must_use]
            pub fn get_unchecked(&self, col_row: [usize; 2]) -> T {
                self.data.data[Self::get_index_unchecked(col_row)].clone()
            }
        }

        /* methods in opposite-order */

        // T, S
        #[doc = "# Single element indexing (using opposite " $D2long "-major order)"]
        impl<T, const C: usize, const R: usize, const CR: usize, S: Storage>
            Array2d<T, C, R, CR, $RMAJ, S>
        {
            /* get_ref (opposite order) */

            /// Returns a reference to the element at the given 2D coordinates
            #[doc = "in the opposite " $D2long "-major order."]
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if the coordinates are out of bounds.
            pub fn [<get_ref_ $D2 maj>](&self, col_row: [usize; 2])
                -> Result<&T, IndexOutOfBounds> {
                Self::[<get_index_ $D2 maj>](col_row).map(|idx| &self.data[idx])
            }
            /// Returns a reference to the element at the given 2D coordinates
            #[doc = "in the opposite " $D2long "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            #[must_use]
            pub fn [<get_ref_ $D2 maj_unchecked>](&self, col_row: [usize; 2]) -> &T {
                &self.data.data[Self::[<get_index_ $D2 maj_unchecked>](col_row)]
            }

            /* get_mut (opposite order) */

            /// Returns an exclusive reference to the element at the given 2D coordinates
            #[doc = "in the opposite " $D2long "-major order."]
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if the coordinates are out of bounds.
            pub fn [<get_mut_ $D2 maj>](&mut self, col_row: [usize; 2])
                -> Result<&mut T, IndexOutOfBounds> {
                Self::[<get_index_ $D2 maj>](col_row).map(|idx| &mut self.data[idx])
            }
            /// Returns an exclusive reference to the element at the given 2D coordinates
            #[doc = "in the opposite " $D2long "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            #[must_use]
            pub fn [<get_mut_ $D2 maj_unchecked>](&mut self, col_row: [usize; 2]) -> &mut T {
                &mut self.data.data[Self::[<get_index_ $D2 maj_unchecked>](col_row)]
            }

            /* set (opposite order) */

            /// Sets the element at the given 2D coordinates
            #[doc = "in the opposite " $D2long "-major order."]
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if the coordinates are out of bounds.
            pub fn [<set_ $D2 maj>](&mut self, element: T, col_row: [usize; 2])
                -> Result<(), IndexOutOfBounds> {
                self.[<get_mut_ $D2 maj>](col_row).map(|e| {
                    *e = element;
                })
            }
            /// Sets the element at the given 2D coordinates
            #[doc = "in the opposite " $D2long "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            pub fn [<set_ $D2 maj_unchecked>](&mut self, element: T, col_row: [usize; 2]) {
                let e = self.[<get_mut_ $D2 maj_unchecked>](col_row);
                *e = element;
            }

            /* indexing (opposite order) */

            /// Calculates the 1D array index from the given 2D coordinates
            #[doc = "in the opposite " $D2long "-major order."]
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if the resulting index is `> CR`.
            pub const fn [<get_index_ $D2 maj>](col_row: [usize; 2])
                -> Result<usize, IndexOutOfBounds> {
                Array2d::<T, C, R, CR, $CMAJ, S>::[<get_index>](col_row)
            }
            /// Calculates the 1D array index from the given 2D coordinates
            #[doc = "in the opposite " $D2long "-major order."]
            #[must_use]
            pub const fn [<get_index_ $D2 maj_unchecked>](col_row: [usize; 2]) -> usize {
                Array2d::<T, C, R, CR, $CMAJ, S>::[<get_index_unchecked>](col_row)
            }

            /// Calculates the 2D coordinates from the given 1D array index
            #[doc = "in the opposite " $D2long "-major order."]
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `index` is `> CR`.
            pub const fn [<get_coords_ $D2 maj>](index: usize)
                -> Result<[usize; 2], IndexOutOfBounds> {
                Array2d::<T, C, R, CR, $CMAJ, S>::[<get_coords>](index)
            }
            /// Calculates the 2D coordinates from the given 1D array index
            #[doc = "in the opposite " $D2long "-major order."]
            pub const fn [<get_coords_ $D2 maj_unchecked>](index: usize) -> [usize; 2] {
                Array2d::<T, C, R, CR, $CMAJ, S>::[<get_coords_unchecked>](index)
            }
        }

        // T: Clone, S
        impl<T: Clone, const C: usize, const R: usize, const CR: usize, S: Storage>
            Array2d<T, C, R, CR, $RMAJ, S>
        {
            /* get (opposite order) */

            /// Returns a clone of the element at the given 2D coordinates
            #[doc = "in the opposite " $D2long "-major order."]
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if the coordinates are out of bounds.
            pub fn [<get_ $D2 maj>](&self, col_row: [usize; 2]) -> Result<T, IndexOutOfBounds> {
                Self::[<get_index_ $D2 maj>](col_row).map(|idx| self.data[idx].clone())
            }
            /// Returns a clone of the element at the given 2D coordinates
            #[doc = "in the opposite " $D2long "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            #[must_use]
            pub fn [<get_ $D2 maj_unchecked>](&self, col_row: [usize; 2]) -> T {
                self.data.data[Self::[<get_index_ $D2 maj_unchecked>](col_row)].clone()
            }
        }
    }};
}
impl_maj![];

/* storage order specific implementations */

// T, S
/// # Fundamental indexing methods in row-major order.
impl<T, const C: usize, const R: usize, const CR: usize, S: Storage> Array2d<T, C, R, CR, true, S> {
    /// Calculates the 1D array index from the given 2D coordinates
    /// in the current row-major order.
    /// # Errors
    /// Returns [`IndexOutOfBounds`] if the resulting index is `>= CR`.
    pub const fn get_index(col_row: [usize; 2]) -> Result<usize, IndexOutOfBounds> {
        let idx = Self::get_index_unchecked(col_row);
        is![idx < CR; Ok(idx); Err(IndexOutOfBounds(Some(idx)))]
    }
    /// Calculates the 1D array index from the given 2D coordinates
    /// in the current row-major order.
    // # Performance
    // This function seems to be 2x faster than
    // [`get_coords_unchecked`](#method.get_coords_unchecked).
    // BENCH: 0.63 ns
    #[must_use]
    pub const fn get_index_unchecked(col_row: [usize; 2]) -> usize {
        col_row[1] * R + col_row[0]
    }

    /// Calculates the 2D coordinates from the given 1D array index
    /// in the current row-major order.
    /// # Errors
    /// Returns [`IndexOutOfBounds`] if `index` is `>= CR`.
    pub const fn get_coords(index: usize) -> Result<[usize; 2], IndexOutOfBounds> {
        is![index < CR; Ok(Self::get_coords_unchecked(index)); Err(IndexOutOfBounds(Some(index)))]
    }
    /// Calculates the 2D coordinates from the given 1D array index
    /// in the current row-major order.
    // # Performance
    // This function seems to be 2x slower than
    // [`get_index_unchecked`](#method.get_index_unchecked).
    // BENCH: 1.4 ns
    #[must_use]
    pub const fn get_coords_unchecked(index: usize) -> [usize; 2] {
        [index % R, index / R]
    }
}

// T, S
/// # Fundamental indexing methods in column-major order.
impl<T, const C: usize, const R: usize, const CR: usize, S: Storage>
    Array2d<T, C, R, CR, false, S>
{
    /// Calculates the 1D array index from the given 2D coordinates
    /// in the current column-major order.
    /// # Errors
    /// Returns [`IndexOutOfBounds`] if the resulting index is `>= CR`.
    pub const fn get_index(col_row: [usize; 2]) -> Result<usize, IndexOutOfBounds> {
        let idx = Self::get_index_unchecked(col_row);
        is![idx < CR; Ok(idx); Err(IndexOutOfBounds(Some(idx)))]
    }
    /// Calculates the 1D array index from the given 2D coordinates
    /// in the current column-major order.
    // # Performance
    // This function seems to be 1.5x faster than
    // [`get_coords_unchecked`](#method.get_coords_unchecked-1).
    // BENCH: 0.62 ns
    #[must_use]
    pub const fn get_index_unchecked(col_row: [usize; 2]) -> usize {
        col_row[0] * C + col_row[1]
    }

    /// Calculates the 2D coordinates from the given 1D array index
    /// in the current column-major order.
    /// # Errors
    /// Returns [`IndexOutOfBounds`] if `index` is `>= CR`.
    pub const fn get_coords(index: usize) -> Result<[usize; 2], IndexOutOfBounds> {
        is![index < CR; Ok(Self::get_coords_unchecked(index)); Err(IndexOutOfBounds(Some(index)))]
    }
    /// Calculates the 2D coordinates from the given 1D array index
    /// in the current column-major order.
    // # Performance
    // This function seems to be 1.5x slower than
    // [`get_index_unchecked`](#method.get_index_unchecked-1).
    // BENCH: 0.94 ns
    #[must_use]
    pub const fn get_coords_unchecked(index: usize) -> [usize; 2] {
        [index / C, index % C]
    }
}
