// devela::data::collections:array::d1::uninit::methods

use crate::{
    iif, Ptr,
    DataError::{NotEnoughSpace, OutOfBounds, PartiallyAdded},
    DataResult as Result, MaybeUninit, Mem, Storage, UninitArray,
};

// T, S
impl<T, const CAP: usize, S: Storage> UninitArray<T, CAP, S> {
    /* construct */

    /// Returns a new uninitialized empty array.
    pub fn new() -> Self {
        // WAIT: [MaybeUninit array methods](https://github.com/rust-lang/rust/issues/96097)
        // let data = MaybeUninit::uninit_array::<CAP>();
        //
        #[allow(clippy::uninit_assumed_init)]
        // SAFETY: we are ensuring elements are only accessed after initialization.
        let data = unsafe { MaybeUninit::uninit().assume_init() };

        Self { data, init_len: 0 }
    }

    /// Initializes the array from an iterator until it's either full or the iterator is exhausted.
    ///
    /// # Returns
    /// - Returns a new array initialized with the elements from the `iterator`.
    /// - Returns [`PartiallyAdded`] if not all elements could be initialized.
    /// - Returns [`NotEnoughSpace`] if the array had no uninitialized elements.
    pub fn from_range<I>(iterator: I) -> Result<Self>
    where
        I: IntoIterator<Item = T>,
    {
        let mut array = Self::new();
        let _ = array.init_range(iterator)?;
        Ok(array)
    }

    /* query */

    /// Returns the count of initialized elements.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn len(&self) -> usize { self.init_len }

    /// Returns `true` if no elements are yet initialized.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn is_empty(&self) -> bool { self.init_len == 0 }

    /// Returns `true` if all the elements are already initialized.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn is_full(&self) -> bool { self.init_len >= CAP }

    /// Returns `index` back if it's within the range already initialized.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if the index is larger than the initialized length.
    #[inline] #[rustfmt::skip]
    pub const fn verify_index(&self, index: usize) -> Result<usize> {
        iif![index < self.init_len; Ok(index); Err(OutOfBounds(Some(index)))]
    }

    /* get */

    /// Returns a shared reference to an initialized element at a given index.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if the index is larger than the initialized length.
    /// or if the element at that index is not initialized.
    pub fn get(&self, index: usize) -> Result<&T> {
        let _ = self.verify_index(index)?;
        // SAFETY: the index is verified
        Ok(unsafe { self.data[index].assume_init_ref() })
    }

    /// Returns an exclusive reference to an initialized element at a given index.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if the index is larger than the initialized length.
    /// or if the element at that index is not initialized.
    pub fn get_mut(&mut self, index: usize) -> Result<&mut T> {
        let _ = self.verify_index(index)?;
        // SAFETY: the index is verified
        Ok(unsafe { self.data[index].assume_init_mut() })
    }

    /* push */

    /// Initializes the next uninitialized element with the provided value.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if the index is larger than the initialized length.
    pub fn init_next(&mut self, value: T) -> Result<()> {
        if self.is_full() {
            Err(OutOfBounds(None))
        } else {
            self.data[self.init_len] = MaybeUninit::new(value);
            self.init_len += 1;
            Ok(())
        }
    }

    /// Initializes elements from an iterator
    ///
    /// Starts at the last initialized element, continues until either the array
    /// is full or the iterator is exhausted.
    ///
    /// # Returns
    /// - Returns the number of elements initialized.
    /// - Returns [`PartiallyAdded`] if not all elements could be initialized.
    /// - Returns [`NotEnoughSpace`] if the array had no uninitialized elements.
    pub fn init_range<I>(&mut self, values: I) -> Result<usize>
    where
        I: IntoIterator<Item = T>,
    {
        if self.is_full() {
            Err(NotEnoughSpace(None))
        } else {
            let start_len = self.init_len;
            for value in values {
                if self.is_full() {
                    return Ok(self.init_len - start_len);
                }
                self.data[self.init_len] = MaybeUninit::new(value);
                self.init_len += 1;
            }
            Err(PartiallyAdded(Some(self.init_len - start_len)))
        }
    }

    /// Replaces the value at a given index with a new value and returns the old value.
    /// # Errors
    /// Returns [`OutOfBounds`] if the index is not within the range of initialized elements.
    #[inline]
    pub fn replace(&mut self, index: usize, value: T) -> Result<T> {
        let index = self.verify_index(index)?;
        // SAFETY: If the index is verified, the value is initialized
        let slot = unsafe { self.data[index].assume_init_mut() };
        Ok(Mem::replace(slot, value))
    }

    /// Swaps the values at two indices.
    /// # Errors
    /// Returns [`OutOfBounds`] if either index is not within the range of initialized elements.
    pub fn swap(&mut self, index1: usize, index2: usize) -> Result<()> {
        let idx1 = self.verify_index(index1)?;
        let idx2 = self.verify_index(index2)?;
        // SAFETY: If the indices are verified, the values are initialized
        unsafe {
            Ptr::swap(self.data[idx1].assume_init_mut(), self.data[idx2].assume_init_mut());
        }
        Ok(())
    }
}
