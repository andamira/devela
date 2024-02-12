// devela::data::collections:array::core_impls
//
//!
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
impl<T, S: Storage, const LEN: usize> Deref for Array<T, S, LEN> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.array.deref()
    }
}
// DerefMut
impl<T, S: Storage, const LEN: usize> DerefMut for Array<T, S, LEN> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.array.deref_mut()
    }
}
// AsRef
impl<T, S: Storage, const LEN: usize> AsRef<[T; LEN]> for Array<T, S, LEN> {
    fn as_ref(&self) -> &[T; LEN] {
        &self.array
    }
}
// AsMut
impl<T, S: Storage, const LEN: usize> AsMut<[T; LEN]> for Array<T, S, LEN> {
    fn as_mut(&mut self) -> &mut [T; LEN] {
        &mut self.array
    }
}
// Borrow
impl<T, S: Storage, const LEN: usize> Borrow<[T; LEN]> for Array<T, S, LEN> {
    fn borrow(&self) -> &[T; LEN] {
        &self.array
    }
}
// BorrowMut
impl<T, S: Storage, const LEN: usize> BorrowMut<[T; LEN]> for Array<T, S, LEN> {
    fn borrow_mut(&mut self) -> &mut [T; LEN] {
        &mut self.array
    }
}

// T:Clone
impl<T: Clone, S: Storage, const LEN: usize> Clone for Array<T, S, LEN>
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
impl<T: Copy, S: Storage, const LEN: usize> Copy for Array<T, S, LEN> where S::Stored<[T; LEN]>: Copy
{}

// T:Debug
impl<T: fmt::Debug, S: Storage, const LEN: usize> fmt::Debug for Array<T, S, LEN>
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
impl<T: PartialEq, S: Storage, const LEN: usize> PartialEq for Array<T, S, LEN>
where
    S::Stored<[T; LEN]>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array && self.len() == other.len()
    }
}
// T:Eq
impl<T: Eq, S: Storage, const LEN: usize> Eq for Array<T, S, LEN> where S::Stored<[T; LEN]>: Eq {}
// T:PartialOrd
impl<T: PartialOrd, S: Storage, const LEN: usize> PartialOrd for Array<T, S, LEN>
where
    S::Stored<[T; LEN]>: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.array.partial_cmp(&other.array)
    }
}
// T:Ord
impl<T: Ord, S: Storage, const LEN: usize> Ord for Array<T, S, LEN>
where
    S::Stored<[T; LEN]>: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.array.cmp(&other.array)
    }
}
// T:Hash
impl<T: Hash, S: Storage, const LEN: usize> Hash for Array<T, S, LEN>
where
    S::Stored<[T; LEN]>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.array.hash(state)
    }
}

// S:Bare + T:Default
impl<T: Default, const LEN: usize> Default for Array<T, Bare, LEN> {
    /// Returns an empty array, allocated in the stack,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        let array = BareBox::new(array_init!(default [T; LEN], "safe_data", "unsafe_array"));
        Array { array }
    }
}
// S:Bare + T:ConstDefault
impl<T: ConstDefault, const LEN: usize> ConstDefault for Array<T, Bare, LEN> {
    /// Returns an empty array, allocated in the stack,
    /// using the default value to fill the remaining free data.
    const DEFAULT: Self = {
        Array {
            array: BareBox::new([T::DEFAULT; LEN]),
        }
    };
}

// S:Boxed + T:Default
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T: Default, const LEN: usize> Default for Array<T, Boxed, LEN> {
    /// Returns an empty array, allocated in the heap,
    /// using the default value to fill the remaining free data.
    ///
    /// # Examples
    /// ```
    /// use devela::data::BoxedArray;
    ///
    /// let mut s = BoxedArray::<i32, 100>::default();
    /// ```
    fn default() -> Self {
        let array = array_init!(default_heap [T; LEN], "safe_data", "unsafe_array");
        Array { array }
    }
}

impl<T, const LEN: usize> From<Array<T, Bare, LEN>> for [T; LEN] {
    fn from(array: Array<T, Bare, LEN>) -> [T; LEN] {
        array.array.0
    }
}
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T, const LEN: usize> From<Array<T, Boxed, LEN>> for Box<[T; LEN]> {
    fn from(array: Array<T, Boxed, LEN>) -> Box<[T; LEN]> {
        array.array
    }
}

impl<T: Default, I, const LEN: usize> From<I> for Array<T, Bare, LEN>
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
    /// # use devela::data::BareArray;
    /// let s: BareArray<_, 4> = [1, 2, 3].into();
    /// assert_eq![s.as_slice(), &[1, 2, 3, 0]];
    /// ```
    fn from(iterator: I) -> Array<T, Bare, LEN> {
        let array = BareBox::new(array_init!(iter [T; LEN], "safe_data", "unsafe_array", iterator));
        Array { array }
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T: Default, I, const LEN: usize> From<I> for Array<T, Boxed, LEN>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a array filled with an iterator, in the heap.
    ///
    /// # Examples
    /// ```
    /// use devela::data::BoxedArray;
    ///
    /// let s: BoxedArray<_, 4> = [1, 2, 3].into();
    ///
    /// assert_eq![s.as_slice(), &[1, 2, 3, 0]];
    /// ```
    fn from(iterator: I) -> Array<T, Boxed, LEN> {
        let array = array_init!(iter_heap [T; LEN], "safe_data", "unsafe_array", iterator);
        Array { array }
    }
}

/* impl bitwise ops */

impl<T, S: Storage, const LEN: usize> BitAnd for Array<T, S, LEN>
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
impl<T, S: Storage, const LEN: usize> BitOr for Array<T, S, LEN>
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
impl<T, S: Storage, const LEN: usize> BitXor for Array<T, S, LEN>
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

impl<T, S: Storage, const LEN: usize> BitAndAssign for Array<T, S, LEN>
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

impl<T, S: Storage, const LEN: usize> BitOrAssign for Array<T, S, LEN>
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

impl<T, S: Storage, const LEN: usize> BitXorAssign for Array<T, S, LEN>
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

impl<T, S: Storage, const LEN: usize> Not for Array<T, S, LEN>
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
