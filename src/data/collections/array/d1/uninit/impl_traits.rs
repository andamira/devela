// devela::data::collections:array::d1::uninit::impl_traits
//
//! 1-dimensional uninit array trait impls
//

use crate::{
    array_init, Array, AsMut, AsRef, Bare, BareBox, BitAnd, BitAndAssign, BitOr, BitOrAssign,
    BitXor, BitXorAssign, Borrow, BorrowMut, ConstDefault, Deref, DerefMut, Hash, Hasher, Not,
    Ordering, Storage, _core::fmt,
};

impl<T, const CAP: usize, S: Storage> Drop for UninitArray<T, CAP, S> {
    fn drop(&mut self) {
        // SAFETY: we are only dropping initialized elements.
        for i in 0..self.len() {
            unsafe {
                self.data[i].assume_init_drop();
            }
        }
    }
}

// AsRef
impl<T, const CAP: usize, S: Storage> AsRef<[T]> for UninitArray<T, CAP, S> {
    /// Returns a shared slice of the initialized elements.
    fn as_ref(&self) -> &[T] {
        // SAFETY: the slice is constructed from the initialized range
        unsafe { from_raw_parts(self.data.as_ptr() as *const T, self.init_len) }
    }
}
// AsMut
impl<T, const CAP: usize, S: Storage> AsMut<[T]> for UninitArray<T, CAP, S> {
    /// Returns an exclusive slice of the initialized elements.
    fn as_mut(&mut self) -> &mut [T] {
        // SAFETY: the slice  from the initialized range
        unsafe { from_raw_parts_mut(self.data.as_mut_ptr() as *mut T, self.init_len) }
    }
}
// Borrow
impl<T, const CAP: usize, S: Storage> Borrow<[T]> for UninitArray<T, CAP, S> {
    /// Returns a shared slice of the initialized elements.
    fn borrow(&self) -> &[T] {
        // SAFETY: we're only creating a slice from the initialized part
        unsafe { from_raw_parts(self.data.as_ptr() as *const T, self.init_len) }
    }
}
// AsMut
impl<T, const CAP: usize, S: Storage> BorrowMut<[T]> for UninitArray<T, CAP, S> {
    /// Returns an exclusive slice of the initialized elements.
    fn borrow_mut(&mut self) -> &mut [T] {
        // SAFETY: we're only creating a slice from the initialized part
        unsafe { from_raw_parts_mut(self.data.as_mut_ptr() as *mut T, self.init_len) }
    }
}
// T:Clone
impl<T: Clone, const CAP: usize, S: Storage> Clone for UninitArray<T, CAP, S>
where
    S::Stored<[MaybeUninit<T>; CAP]>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            // Directly clone the data, including uninitialized parts.
            data: self.data.clone(),
            init_len: self.init_len,
        }
    }
}
