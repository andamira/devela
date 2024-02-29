// devela::data::collections:array::d1::impl_traits
//
//! 1-dimensional array trait impls
//

use crate::{
    code::ConstDefault,
    data::{array_init, Array},
    mem::{Bare, BareBox, Storage},
};
use core::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    convert::{AsMut, AsRef},
    fmt,
    hash::{Hash, Hasher},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Not},
};

#[cfg(feature = "alloc")]
use crate::{
    _deps::alloc::{boxed::Box, vec::Vec},
    mem::Boxed,
};

// Deref
impl<T, const LEN: usize, S: Storage> Deref for Array<T, LEN, S> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.array.deref()
    }
}
// DerefMut
impl<T, const LEN: usize, S: Storage> DerefMut for Array<T, LEN, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.array.deref_mut()
    }
}
// AsRef
impl<T, const LEN: usize, S: Storage> AsRef<[T; LEN]> for Array<T, LEN, S> {
    fn as_ref(&self) -> &[T; LEN] {
        &self.array
    }
}
// AsMut
impl<T, const LEN: usize, S: Storage> AsMut<[T; LEN]> for Array<T, LEN, S> {
    fn as_mut(&mut self) -> &mut [T; LEN] {
        &mut self.array
    }
}
// Borrow
impl<T, const LEN: usize, S: Storage> Borrow<[T; LEN]> for Array<T, LEN, S> {
    fn borrow(&self) -> &[T; LEN] {
        &self.array
    }
}
// BorrowMut
impl<T, const LEN: usize, S: Storage> BorrowMut<[T; LEN]> for Array<T, LEN, S> {
    fn borrow_mut(&mut self) -> &mut [T; LEN] {
        &mut self.array
    }
}

// T:Clone
impl<T: Clone, const LEN: usize, S: Storage> Clone for Array<T, LEN, S>
where
    S::Stored<[T; LEN]>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            array: self.array.clone(),
        }
    }
}

// T:Copy
impl<T: Copy, const LEN: usize, S: Storage> Copy for Array<T, LEN, S> where S::Stored<[T; LEN]>: Copy
{}

// T:Debug
impl<T: fmt::Debug, const LEN: usize, S: Storage> fmt::Debug for Array<T, LEN, S>
where
    S::Stored<[T; LEN]>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct(stringify![Array]);
        debug.field("LEN", &LEN);
        debug.field("", &self.array);
        debug.finish()
    }
}
// T:PartialEq
impl<T: PartialEq, const LEN: usize, S: Storage> PartialEq for Array<T, LEN, S>
where
    S::Stored<[T; LEN]>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array && self.len() == other.len()
    }
}
// T:Eq
impl<T: Eq, const LEN: usize, S: Storage> Eq for Array<T, LEN, S> where S::Stored<[T; LEN]>: Eq {}
// T:PartialOrd
impl<T: PartialOrd, const LEN: usize, S: Storage> PartialOrd for Array<T, LEN, S>
where
    S::Stored<[T; LEN]>: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.array.partial_cmp(&other.array)
    }
}
// T:Ord
impl<T: Ord, const LEN: usize, S: Storage> Ord for Array<T, LEN, S>
where
    S::Stored<[T; LEN]>: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.array.cmp(&other.array)
    }
}
// T:Hash
impl<T: Hash, const LEN: usize, S: Storage> Hash for Array<T, LEN, S>
where
    S::Stored<[T; LEN]>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.array.hash(state)
    }
}

// S:Bare + T:Default
impl<T: Default, const LEN: usize> Default for Array<T, LEN, Bare> {
    /// Returns an array, allocated in the stack,
    /// using the default value to fill the data.
    fn default() -> Self {
        let array = BareBox::new(array_init!(default [T; LEN], "safe_data", "unsafe_array"));
        Array { array }
    }
}
// S:Bare + T:ConstDefault
impl<T: ConstDefault, const LEN: usize> ConstDefault for Array<T, LEN, Bare> {
    /// Returns an array, allocated in the stack,
    /// using the default value to fill the data.
    const DEFAULT: Self = {
        Array {
            array: BareBox::new([T::DEFAULT; LEN]),
        }
    };
}

// S:Boxed + T:Default
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T: Default, const LEN: usize> Default for Array<T, LEN, Boxed> {
    /// Returns an array, allocated in the heap,
    /// using the default value to fill the data.
    ///
    /// # Examples
    /// ```
    /// # use devela::all::{Array, Boxed};
    /// let mut s = Array::<i32, 100, Boxed>::default();
    /// ```
    fn default() -> Self {
        let array = array_init!(default_heap [T; LEN], "safe_data", "unsafe_array");
        Array { array }
    }
}

impl<T, const LEN: usize> From<Array<T, LEN, Bare>> for [T; LEN] {
    fn from(array: Array<T, LEN, Bare>) -> [T; LEN] {
        array.array.0
    }
}
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T, const LEN: usize> From<Array<T, LEN, Boxed>> for Box<[T; LEN]> {
    fn from(array: Array<T, LEN, Boxed>) -> Box<[T; LEN]> {
        array.array
    }
}

/* iterator related */

impl<T: Default, I, const LEN: usize> From<I> for Array<T, LEN, Bare>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a array filled with an iterator, in the stack.
    ///
    /// If the `iterator` length is less than the array length `LEN`,
    /// the missing elements will be the default value of `T`.
    ///
    /// # Examples
    /// ```
    /// # use devela::data::Array;
    /// let s: Array<_, 4> = [1, 2, 3].into();
    /// assert_eq![s.as_slice(), &[1, 2, 3, 0]];
    /// ```
    fn from(iterator: I) -> Array<T, LEN, Bare> {
        let array = BareBox::new(array_init!(iter [T; LEN], "safe_data", "unsafe_array", iterator));
        Array { array }
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T: Default, I, const LEN: usize> From<I> for Array<T, LEN, Boxed>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a array filled with an iterator, in the heap.
    ///
    /// # Examples
    /// ```
    /// # use devela::all::{Array, Boxed};
    /// let s: Array<_, 4, Boxed> = [1, 2, 3].into();
    ///
    /// assert_eq![s.as_slice(), &[1, 2, 3, 0]];
    /// ```
    fn from(iterator: I) -> Array<T, LEN, Boxed> {
        let array = array_init!(iter_heap [T; LEN], "safe_data", "unsafe_array", iterator);
        Array { array }
    }
}

/* impl bitwise ops */

impl<T, const LEN: usize, S: Storage> BitAnd for Array<T, LEN, S>
where
    S::Stored<[T; LEN]>: Copy,
    T: BitAnd<Output = T> + Copy,
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for i in 0..LEN {
            result.array[i] = result.array[i] & rhs.array[i];
        }
        result
    }
}
impl<T, const LEN: usize, S: Storage> BitOr for Array<T, LEN, S>
where
    S::Stored<[T; LEN]>: Copy,
    T: BitOr<Output = T> + Copy,
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for i in 0..LEN {
            result.array[i] = result.array[i] | rhs.array[i];
        }
        result
    }
}
impl<T, const LEN: usize, S: Storage> BitXor for Array<T, LEN, S>
where
    S::Stored<[T; LEN]>: Copy,
    T: BitXor<Output = T> + Copy,
{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for i in 0..LEN {
            result.array[i] = result.array[i] ^ rhs.array[i];
        }
        result
    }
}

impl<T, const LEN: usize, S: Storage> BitAndAssign for Array<T, LEN, S>
where
    S::Stored<[T; LEN]>: Copy,
    T: BitAndAssign + Copy,
{
    fn bitand_assign(&mut self, rhs: Self) {
        for i in 0..LEN {
            self.array[i] &= rhs.array[i];
        }
    }
}

impl<T, const LEN: usize, S: Storage> BitOrAssign for Array<T, LEN, S>
where
    S::Stored<[T; LEN]>: Copy,
    T: BitOrAssign + Copy,
{
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..LEN {
            self.array[i] |= rhs.array[i];
        }
    }
}

impl<T, const LEN: usize, S: Storage> BitXorAssign for Array<T, LEN, S>
where
    S::Stored<[T; LEN]>: Copy,
    T: BitXorAssign + Copy,
{
    fn bitxor_assign(&mut self, rhs: Self) {
        for i in 0..LEN {
            self.array[i] ^= rhs.array[i];
        }
    }
}

impl<T, const LEN: usize, S: Storage> Not for Array<T, LEN, S>
where
    S::Stored<[T; LEN]>: Clone,
    T: Not<Output = T> + Copy,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut result = self;
        for i in 0..LEN {
            result.array[i] = !result.array[i];
        }
        result
    }
}
