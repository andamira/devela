// devela::data::list:array::d1::impl_traits
//
//! 1-dimensional array trait impls
//
// TOC
// - utility traits
// - iterator related
// - bitwise ops

use crate::{
    Array, AsMut, AsRef, Bare, BareBox, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor,
    BitXorAssign, Borrow, BorrowMut, ConstDefault, Debug, Deref, DerefMut, FmtResult, Formatter,
    Hash, Hasher, Not, Ordering, Storage, array_init,
};

#[cfg(feature = "alloc")]
use crate::{Box, Boxed};

/* utility traits */

// Deref
impl<T, const CAP: usize, S: Storage> Deref for Array<T, CAP, S> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}
// DerefMut
impl<T, const CAP: usize, S: Storage> DerefMut for Array<T, CAP, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.deref_mut()
    }
}
// AsRef
impl<T, const CAP: usize, S: Storage> AsRef<[T; CAP]> for Array<T, CAP, S> {
    fn as_ref(&self) -> &[T; CAP] {
        &self.data
    }
}
// AsMut
impl<T, const CAP: usize, S: Storage> AsMut<[T; CAP]> for Array<T, CAP, S> {
    fn as_mut(&mut self) -> &mut [T; CAP] {
        &mut self.data
    }
}
// Borrow
impl<T, const CAP: usize, S: Storage> Borrow<[T; CAP]> for Array<T, CAP, S> {
    fn borrow(&self) -> &[T; CAP] {
        &self.data
    }
}
// BorrowMut
impl<T, const CAP: usize, S: Storage> BorrowMut<[T; CAP]> for Array<T, CAP, S> {
    fn borrow_mut(&mut self) -> &mut [T; CAP] {
        &mut self.data
    }
}

// T: Clone
impl<T: Clone, const CAP: usize, S: Storage> Clone for Array<T, CAP, S>
where
    S::Stored<[T; CAP]>: Clone,
{
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
}

// T: Copy
impl<T: Copy, const CAP: usize, S: Storage> Copy for Array<T, CAP, S> where S::Stored<[T; CAP]>: Copy
{}

// T: Debug
impl<T: Debug, const CAP: usize, S: Storage> Debug for Array<T, CAP, S>
where
    S::Stored<[T; CAP]>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        let mut debug = f.debug_struct("Array");
        debug.field("T", &core::any::type_name::<T>()).field("S", &S::name()).field("CAP", &CAP);

        const MAX: usize = 16;
        if CAP <= MAX {
            debug.field("data", &self.data);
        } else {
            let first = &self.data[..MAX / 2];
            let last = &self.data[CAP - MAX / 2..];
            debug.field("array", &(&first, "...", &last));
        }
        debug.finish()
    }
}
// T: PartialEq
impl<T: PartialEq, const CAP: usize, S: Storage> PartialEq for Array<T, CAP, S>
where
    S::Stored<[T; CAP]>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.capacity() == other.capacity()
    }
}
// T: Eq
impl<T: Eq, const CAP: usize, S: Storage> Eq for Array<T, CAP, S> where S::Stored<[T; CAP]>: Eq {}
// T: PartialOrd
impl<T: PartialOrd, const CAP: usize, S: Storage> PartialOrd for Array<T, CAP, S>
where
    S::Stored<[T; CAP]>: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}
// T: Ord
impl<T: Ord, const CAP: usize, S: Storage> Ord for Array<T, CAP, S>
where
    S::Stored<[T; CAP]>: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}
// T: Hash
impl<T: Hash, const CAP: usize, S: Storage> Hash for Array<T, CAP, S>
where
    S::Stored<[T; CAP]>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

// T: Default, S: Bare
impl<T: Default, const CAP: usize> Default for Array<T, CAP, Bare> {
    /// Returns an array, allocated in the stack,
    /// using the default value to fill the data.
    fn default() -> Self {
        let data = BareBox::new(array_init!(default [T; CAP], "safe_data", "unsafe_array"));
        Array { data }
    }
}
// T: ConstDefault, S: Bare
impl<T: ConstDefault, const CAP: usize> ConstDefault for Array<T, CAP, Bare> {
    /// Returns an array, allocated in the stack,
    /// using the default value to fill the data.
    const DEFAULT: Self = { Array { data: BareBox::new([T::DEFAULT; CAP]) } };
}

// T: Default, S: Boxed
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
impl<T: Default, const CAP: usize> Default for Array<T, CAP, Boxed> {
    /// Returns an array, allocated in the heap,
    /// using the default value to fill the data.
    ///
    /// # Examples
    /// ```
    /// # use devela::{Array, Boxed};
    /// let mut s = Array::<i32, 100, Boxed>::default();
    /// ```
    fn default() -> Self {
        let data = array_init!(default_heap [T; CAP], "safe_data", "unsafe_array");
        Array { data }
    }
}

impl<T, const CAP: usize> From<Array<T, CAP, Bare>> for [T; CAP] {
    fn from(array: Array<T, CAP, Bare>) -> [T; CAP] {
        array.data.0
    }
}
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
impl<T, const CAP: usize> From<Array<T, CAP, Boxed>> for Box<[T; CAP]> {
    fn from(array: Array<T, CAP, Boxed>) -> Box<[T; CAP]> {
        array.data
    }
}

/* iterator related */

impl<T: Default, I, const CAP: usize> From<I> for Array<T, CAP, Bare>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a array filled with an iterator, in the stack.
    ///
    /// If the `iterator` length is less than the array capacity `CAP`,
    /// the missing elements will be the default value of `T`.
    ///
    /// # Examples
    /// ```
    /// # use devela::data::Array;
    /// let s: Array<_, 4> = [1, 2, 3].into();
    /// assert_eq![s.as_slice(), &[1, 2, 3, 0]];
    /// ```
    fn from(iterator: I) -> Array<T, CAP, Bare> {
        let data = BareBox::new(array_init!(iter [T; CAP], "safe_data", "unsafe_array", iterator));
        Array { data }
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
impl<T: Default, I, const CAP: usize> From<I> for Array<T, CAP, Boxed>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a array filled with an iterator, in the heap.
    ///
    /// # Examples
    /// ```
    /// # use devela::{Array, Boxed};
    /// let s: Array<_, 4, Boxed> = [1, 2, 3].into();
    ///
    /// assert_eq![s.as_slice(), &[1, 2, 3, 0]];
    /// ```
    fn from(iterator: I) -> Array<T, CAP, Boxed> {
        let data = array_init!(iter_heap [T; CAP], "safe_data", "unsafe_array", iterator);
        Array { data }
    }
}

/* impl bitwise ops */

impl<T, const CAP: usize, S: Storage> BitAnd for Array<T, CAP, S>
where
    S::Stored<[T; CAP]>: Copy,
    T: BitAnd<Output = T> + Copy,
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for i in 0..CAP {
            result.data[i] = result.data[i] & rhs.data[i];
        }
        result
    }
}
impl<T, const CAP: usize, S: Storage> BitOr for Array<T, CAP, S>
where
    S::Stored<[T; CAP]>: Copy,
    T: BitOr<Output = T> + Copy,
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for i in 0..CAP {
            result.data[i] = result.data[i] | rhs.data[i];
        }
        result
    }
}
impl<T, const CAP: usize, S: Storage> BitXor for Array<T, CAP, S>
where
    S::Stored<[T; CAP]>: Copy,
    T: BitXor<Output = T> + Copy,
{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for i in 0..CAP {
            result.data[i] = result.data[i] ^ rhs.data[i];
        }
        result
    }
}

impl<T, const CAP: usize, S: Storage> BitAndAssign for Array<T, CAP, S>
where
    S::Stored<[T; CAP]>: Copy,
    T: BitAndAssign + Copy,
{
    fn bitand_assign(&mut self, rhs: Self) {
        for i in 0..CAP {
            self.data[i] &= rhs.data[i];
        }
    }
}

impl<T, const CAP: usize, S: Storage> BitOrAssign for Array<T, CAP, S>
where
    S::Stored<[T; CAP]>: Copy,
    T: BitOrAssign + Copy,
{
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..CAP {
            self.data[i] |= rhs.data[i];
        }
    }
}

impl<T, const CAP: usize, S: Storage> BitXorAssign for Array<T, CAP, S>
where
    S::Stored<[T; CAP]>: Copy,
    T: BitXorAssign + Copy,
{
    fn bitxor_assign(&mut self, rhs: Self) {
        for i in 0..CAP {
            self.data[i] ^= rhs.data[i];
        }
    }
}

impl<T, const CAP: usize, S: Storage> Not for Array<T, CAP, S>
where
    S::Stored<[T; CAP]>: Clone,
    T: Not<Output = T> + Copy,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut result = self;
        for i in 0..CAP {
            result.data[i] = !result.data[i];
        }
        result
    }
}
