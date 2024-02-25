// devela::data::collections::array::d2::methods::coords
//
//! 2-dimensional array coordinates and indexing methods
//

use crate::{
    code::iif,
    data::{
        error::{DataError, DataResult as Result},
        Array2d,
    },
    mem::Storage,
};
use DataError::Overflow;

// Helper macro for implementing common methods generic on storage order
macro_rules! impl_maj {
    () => {
        impl_maj![true:x, false:y];
        impl_maj![false:y, true:x];
    };

    // $XMAJ:  true for X-major order
    // $D1:    the major dimension name
    // $YMAJ:  false for X-major order
    // $D2:    the other dimension name
    ( $XMAJ:literal:$D1:ident, $YMAJ:literal:$D2:ident) => { crate::code::paste! {

        #[doc = "# Single element indexing (" $D1 "-major order)"]
        impl<T, S: Storage, const X: usize, const Y: usize, const LEN: usize>
            Array2d<T, S, X, Y, LEN, $XMAJ>
        {
            /* get_ref */

            /// Returns a reference to the element at the given 2D coordinates
            #[doc = "in the current " $D1 "-major order."]
            /// # Errors
            /// Returns [`Overflow`] if the coordinates are out of bounds.
            #[inline]
            pub fn get_ref(&self, xy: [usize; 2]) -> Result<&T> {
                Self::get_index(xy).map(|idx| &self.array[idx])
            }
            /// Returns a reference to the element at the given 2D coordinates
            #[doc = "in the current " $D1 "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            #[inline]
            #[must_use]
            pub fn get_ref_unchecked(&self, xy: [usize; 2]) -> &T {
                &self.array.array[Self::get_index_unchecked(xy)]
            }

            /* get_mut */

            /// Returns an exclusive reference to the element at the given 2D coordinates
            #[doc = "in the current " $D1 "-major order."]
            /// # Errors
            /// Returns [`Overflow`] if the coordinates are out of bounds.
            #[inline]
            pub fn get_mut(&mut self, xy: [usize; 2]) -> Result<&mut T> {
                Self::get_index(xy).map(|idx| &mut self.array[idx])
            }
            /// Returns an exclusive reference to the element at the given 2D coordinates
            #[doc = "in the current " $D1 "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            #[inline]
            #[must_use]
            pub fn get_mut_unchecked(&mut self, xy: [usize; 2]) -> &mut T {
                &mut self.array.array[Self::get_index_unchecked(xy)]
            }

            /* set */

            /// Sets the element at the given 2D coordinates
            #[doc = "in the current " $D1 "-major order."]
            /// # Errors
            /// Returns [`Overflow`] if the coordinates are out of bounds.
            #[inline]
            pub fn set(&mut self, element: T, xy: [usize; 2]) -> Result<()> {
                self.get_mut(xy).map(|e| {
                    *e = element;
                })
            }
            /// Sets the element at the given 2D coordinates
            #[doc = "in the current " $D1 "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            #[inline]
            pub fn set_unchecked(&mut self, element: T, xy: [usize; 2]) {
                let e = self.get_mut_unchecked(xy);
                *e = element;
            }
        }

        // # Single element clone
        impl<T: Clone, S: Storage, const X: usize, const Y: usize, const LEN: usize>
            Array2d<T, S, X, Y, LEN, $XMAJ>
        {
            /* get */

            /// Returns a clone of the element at the given 2D coordinates
            #[doc = "in the current " $D1 "-major order."]
            /// # Errors
            /// Returns [`Overflow`] if the coordinates are out of bounds.
            #[inline]
            pub fn get(&self, xy: [usize; 2]) -> Result<T> {
                Self::get_index(xy).map(|idx| self.array[idx].clone())
            }
            /// Returns a clone of the element at the given 2D coordinates
            #[doc = "in the current " $D1 "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            #[inline]
            #[must_use]
            pub fn get_unchecked(&self, xy: [usize; 2]) -> T {
                self.array.array[Self::get_index_unchecked(xy)].clone()
            }
        }

        /* methods in opposite-order */

        #[doc = "# Single element indexing (using opposite " $D2 "-major order)"]
        impl<T, S: Storage, const X: usize, const Y: usize, const LEN: usize>
            Array2d<T, S, X, Y, LEN, $XMAJ>
        {
            /* get_ref (opposite order) */

            /// Returns a reference to the element at the given 2D coordinates
            #[doc = "in the opposite " $D2 "-major order."]
            /// # Errors
            /// Returns [`Overflow`] if the coordinates are out of bounds.
            #[inline]
            pub fn [<get_ref_ $D2 maj>](&self, xy: [usize; 2]) -> Result<&T> {
                Self::[<get_index_ $D2 maj>](xy).map(|idx| &self.array[idx])
            }
            /// Returns a reference to the element at the given 2D coordinates
            #[doc = "in the opposite " $D2 "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            #[inline]
            #[must_use]
            pub fn [<get_ref_ $D2 maj_unchecked>](&self, xy: [usize; 2]) -> &T {
                &self.array.array[Self::[<get_index_ $D2 maj_unchecked>](xy)]
            }

            /* get_mut (opposite order) */

            /// Returns an exclusive reference to the element at the given 2D coordinates
            #[doc = "in the opposite " $D2 "-major order."]
            /// # Errors
            /// Returns [`Overflow`] if the coordinates are out of bounds.
            #[inline]
            pub fn [<get_mut_ $D2 maj>](&mut self, xy: [usize; 2]) -> Result<&mut T> {
                Self::[<get_index_ $D2 maj>](xy).map(|idx| &mut self.array[idx])
            }
            /// Returns an exclusive reference to the element at the given 2D coordinates
            #[doc = "in the opposite " $D2 "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            #[inline]
            #[must_use]
            pub fn [<get_mut_ $D2 maj_unchecked>](&mut self, xy: [usize; 2]) -> &mut T {
                &mut self.array.array[Self::[<get_index_ $D2 maj_unchecked>](xy)]
            }

            /* set (opposite order) */

            /// Sets the element at the given 2D coordinates
            #[doc = "in the opposite " $D2 "-major order."]
            /// # Errors
            /// Returns [`Overflow`] if the coordinates are out of bounds.
            #[inline]
            pub fn [<set_ $D2 maj>](&mut self, element: T, xy: [usize; 2]) -> Result<()> {
                self.[<get_mut_ $D2 maj>](xy).map(|e| {
                    *e = element;
                })
            }
            /// Sets the element at the given 2D coordinates
            #[doc = "in the opposite " $D2 "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            #[inline]
            pub fn [<set_ $D2 maj_unchecked>](&mut self, element: T, xy: [usize; 2]) {
                let e = self.[<get_mut_ $D2 maj_unchecked>](xy);
                *e = element;
            }

            /* indexing (opposite order) */

            /// Calculates the 1D array index from the given 2D coordinates
            #[doc = "in the opposite " $D2 "-major order."]
            /// # Errors
            /// Returns [`Overflow`] if the resulting index is `> LEN`.
            #[inline]
            pub const fn [<get_index_ $D2 maj>](xy: [usize; 2]) -> Result<usize> {
                Array2d::<T, S, X, Y, LEN, $YMAJ>::[<get_index>](xy)
            }
            /// Calculates the 1D array index from the given 2D coordinates
            #[doc = "in the opposite " $D2 "-major order."]
            #[inline]
            #[must_use]
            pub const fn [<get_index_ $D2 maj_unchecked>](xy: [usize; 2]) -> usize {
                Array2d::<T, S, X, Y, LEN, $YMAJ>::[<get_index_unchecked>](xy)
            }

            /// Calculates the 2D coordinates from the given 1D array index
            #[doc = "in the opposite " $D2 "-major order."]
            /// # Errors
            /// Returns [`Overflow`] if `index` is `> LEN`.
            #[inline]
            pub const fn [<get_coords_ $D2 maj>](index: usize) -> Result<[usize; 2]> {
                Array2d::<T, S, X, Y, LEN, $YMAJ>::[<get_coords>](index)
            }
            /// Calculates the 2D coordinates from the given 1D array index
            #[doc = "in the opposite " $D2 "-major order."]
            #[inline]
            pub const fn [<get_coords_ $D2 maj_unchecked>](index: usize) -> [usize; 2] {
                Array2d::<T, S, X, Y, LEN, $YMAJ>::[<get_coords_unchecked>](index)
            }
        }

        impl<T: Clone, S: Storage, const X: usize, const Y: usize, const LEN: usize>
            Array2d<T, S, X, Y, LEN, $XMAJ>
        {
            /* get (opposite order) */

            /// Returns a clone of the element at the given 2D coordinates
            #[doc = "in the opposite " $D2 "-major order."]
            /// # Errors
            /// Returns [`Overflow`] if the coordinates are out of bounds.
            #[inline]
            pub fn [<get_ $D2 maj>](&self, xy: [usize; 2]) -> Result<T> {
                Self::[<get_index_ $D2 maj>](xy).map(|idx| self.array[idx].clone())
            }
            /// Returns a clone of the element at the given 2D coordinates
            #[doc = "in the opposite " $D2 "-major order."]
            /// # Panics
            /// Panics if the coordinates are out of bounds.
            #[inline]
            #[must_use]
            pub fn [<get_ $D2 maj_unchecked>](&self, xy: [usize; 2]) -> T {
                self.array.array[Self::[<get_index_ $D2 maj_unchecked>](xy)].clone()
            }
        }
    }};
}
impl_maj![];

/* storage order specific implementations */

/// # Fundamental indexing methods in X-major order.
impl<T, S: Storage, const X: usize, const Y: usize, const LEN: usize>
    Array2d<T, S, X, Y, LEN, true>
{
    /// Calculates the 1D array index from the given 2D coordinates
    /// in the current X-major order.
    /// # Errors
    /// Returns [`Overflow`] if the resulting index is `>= LEN`.
    #[inline]
    pub const fn get_index(xy: [usize; 2]) -> Result<usize> {
        let idx = Self::get_index_unchecked(xy);
        iif![idx < LEN; Ok(idx); Err(Overflow)]
    }
    /// Calculates the 1D array index from the given 2D coordinates
    /// in the current X-major order.
    // # Performance
    // This function seems to be 2x faster than
    // [`get_coords_unchecked`](#method.get_coords_unchecked).
    // BENCH: 0.63 ns
    #[inline]
    #[must_use]
    pub const fn get_index_unchecked(xy: [usize; 2]) -> usize {
        xy[1] * Y + xy[0]
    }

    /// Calculates the 2D coordinates from the given 1D array index
    /// in the current X-major order.
    /// # Errors
    /// Returns [`Overflow`] if `index` is `>= LEN`.
    #[inline]
    pub const fn get_coords(index: usize) -> Result<[usize; 2]> {
        iif![index < LEN; Ok(Self::get_coords_unchecked(index)); Err(Overflow)]
    }
    /// Calculates the 2D coordinates from the given 1D array index
    /// in the current X-major order.
    // # Performance
    // This function seems to be 2x slower than
    // [`get_index_unchecked`](#method.get_index_unchecked).
    // BENCH: 1.4 ns
    #[inline]
    #[must_use]
    pub const fn get_coords_unchecked(index: usize) -> [usize; 2] {
        [index % Y, index / Y]
    }
}

/// # Fundamental indexing methods in Y-major order.
impl<T, S: Storage, const X: usize, const Y: usize, const LEN: usize>
    Array2d<T, S, X, Y, LEN, false>
{
    /// Calculates the 1D array index from the given 2D coordinates
    /// in the current Y-major order.
    /// # Errors
    /// Returns [`Overflow`] if the resulting index is `>= LEN`.
    #[inline]
    pub const fn get_index(xy: [usize; 2]) -> Result<usize> {
        let idx = Self::get_index_unchecked(xy);
        iif![idx < LEN; Ok(idx); Err(Overflow)]
    }
    /// Calculates the 1D array index from the given 2D coordinates
    /// in the current Y-major order.
    // # Performance
    // This function seems to be 1.5x faster than
    // [`get_coords_unchecked`](#method.get_coords_unchecked-1).
    // BENCH: 0.62 ns
    #[inline]
    #[must_use]
    pub const fn get_index_unchecked(xy: [usize; 2]) -> usize {
        xy[0] * X + xy[1]
    }

    /// Calculates the 2D coordinates from the given 1D array index
    /// in the current Y-major order.
    /// # Errors
    /// Returns [`Overflow`] if `index` is `>= LEN`.
    #[inline]
    pub const fn get_coords(index: usize) -> Result<[usize; 2]> {
        iif![index < LEN; Ok(Self::get_coords_unchecked(index)); Err(Overflow)]
    }
    /// Calculates the 2D coordinates from the given 1D array index
    /// in the current Y-major order.
    // # Performance
    // This function seems to be 1.5x slower than
    // [`get_index_unchecked`](#method.get_index_unchecked-1).
    // BENCH: 0.94 ns
    #[inline]
    #[must_use]
    pub const fn get_coords_unchecked(index: usize) -> [usize; 2] {
        [index / X, index % X]
    }
}
