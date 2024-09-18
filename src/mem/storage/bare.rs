// devela::mem::storage::bare
//
//! *Bare* storage doesn't affect its contents
//
// API based on https://doc.rust-lang.org/alloc/boxed/struct.Box.html

use crate::mem::Storage;
#[cfg(all(doc, feature = "alloc"))]
use crate::mem::{Box, Boxed};

/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
/// A zero-sized marker for a [`Storage`][super::Storage] type that wraps its data in a
/// [`BareBox`][super::BareBox].
///
/// Equivalent to the [`Boxed`] marker struct which uses a [`Box`] for the underlying storage.
pub type Bare = ();

/// A no-op pointer type, like a [`Box`] but without affecting how `T` is stored.
///
/// It is used as the underlying [`Storage`] for the [`Bare`] marker
/// struct, just as a [`Box`] is used as the storage for [`Boxed`].
///
/// # Examples
/// ```
/// # use devela::mem::BareBox;
/// let byte = BareBox::new(0_u8);
/// ```
pub struct BareBox<T>(pub T);

/// A zero-sized marker for a storage type that wraps its data in a [`BareBox`].
///
/// This implementation is equivalent to the one for [`Boxed`] which uses [`Box`] for storage.
#[rustfmt::skip]
impl Storage for Bare {
    type Stored<T> = BareBox<T>;
    fn name() -> &'static str { "BareBox" }
}

#[rustfmt::skip]
impl<T> BareBox<T> {
    /// Creates a new `BareBox` storage for the given `t`.
    #[inline] #[must_use]
    pub const fn new(t: T) -> Self { Self(t) }

    /// Returns the inner stored type.
    ///
    /// Example
    /// ```
    /// # use devela::BareBox;
    /// let b = BareBox::new(42);
    /// let inner = b.into_inner();
    /// assert_eq!(42, inner);
    /// ```
    #[inline] #[must_use]
    pub fn into_inner(self) -> T { self.0 }

    /// Returns a shared reference to the inner stored type, in compile-time.
    ///
    /// Example
    /// ```
    /// # use devela::BareBox;
    /// const B: BareBox<char> = BareBox::new('a');
    /// const REF: &char = B.as_ref();
    /// assert_eq!('a', *REF);
    /// ```
    #[inline] #[must_use]
    pub const fn as_ref(&self) -> &T { &self.0 }

    /// Replaces the stored value with a `new` one, returning the old value.
    ///
    /// Example
    /// ```
    /// # use devela::BareBox;
    /// let mut b = BareBox::new(42);
    /// let old = b.replace(100);
    /// assert_eq!(42, old);
    /// assert_eq!(100, *b);
    /// ```
    #[inline] #[must_use]
    pub fn replace(&mut self, mut new: T) -> T {
        crate::mem_swap(&mut self.0, &mut new); new
    }

    /// Checks if the stored value is the default.
    ///
    /// Example
    /// ```
    /// # use devela::BareBox;
    /// let b = BareBox::new(0);
    /// assert!(b.is_default());
    /// ```
    #[inline] #[must_use]
    pub fn is_default(&self) -> bool where T: Default + PartialEq {
        self.0 == T::default()
    }
}

#[rustfmt::skip]
impl<T: Copy> BareBox<T> {
    /// Returns the inner stored type in compile-time.
    ///
    /// Example
    /// ```
    /// # use devela::BareBox;
    /// const B: BareBox<i32> = BareBox::new(42);
    /// const I: i32 = B.into_inner_copy();
    /// assert_eq!(42, I);
    /// ```
    #[inline] #[must_use]
    pub const fn into_inner_copy(self) -> T {
        self.0
    }

    /// Maps the inner value to another type using the provided function.
    ///
    /// Example
    /// ```
    /// # use devela::BareBox;
    /// let b = BareBox::new(2);
    /// let squared = b.map(|x| x * x);
    /// assert_eq!(4, *squared);
    /// ```
    pub fn map<U: Copy>(self, f: fn(T) -> U) -> BareBox<U> {
        BareBox(f(self.0))
    }
}

impl<T: Copy> BareBox<Option<T>> {
    /// Unwraps the inner `Option`, returning the contained value or a default,
    /// in compile time.
    ///
    /// Example
    /// ```
    /// # use devela::BareBox;
    /// const B: BareBox<Option<char>> = BareBox::new(Some('a'));
    /// const BU: char = B.unwrap_copy_or('c');
    /// assert_eq!['a', BU];
    ///
    /// // We could also auto-dereference to Option::unwrap_or():
    /// assert_eq!['a', B.unwrap_or('b')];
    /// ```
    #[inline] #[rustfmt::skip]
    pub const fn unwrap_copy_or(self, default: T) -> T {
        match self.0 { Some(val) => val, None => default }
    }
}
impl<T: Copy, E: Copy> BareBox<Result<T, E>> {
    /// Unwraps the inner `Result`, returning the `Ok` value or a default,
    /// in compile time.
    ///
    /// Example
    /// ```
    /// # use devela::BareBox;
    /// const B: BareBox<Result<char, ()>> = BareBox::new(Ok('a'));
    ///
    /// const BU: char = B.unwrap_copy_or('c');
    /// assert_eq!['a', BU];
    ///
    /// // We could also auto-dereference to Result::unwrap_or():
    /// assert_eq!['a', B.unwrap_or('b')];
    /// ```
    #[inline] #[rustfmt::skip]
    pub const fn unwrap_copy_or(self, default: T) -> T {
        match self.0 { Ok(val) => val, Err(_) => default }
    }
}

mod core_impls {
    use super::BareBox;
    use crate::code::ConstDefault;
    use core::{cmp, convert, fmt, hash, ops};

    impl<T> ops::Deref for BareBox<T> {
        type Target = T;
        #[inline]
        fn deref(&self) -> &T {
            &self.0
        }
    }
    impl<T> ops::DerefMut for BareBox<T> {
        #[inline]
        #[must_use]
        fn deref_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    impl<T> convert::AsRef<T> for BareBox<T> {
        #[inline]
        fn as_ref(&self) -> &T {
            &self.0
        }
    }
    impl<T> convert::AsMut<T> for BareBox<T> {
        #[inline]
        #[must_use]
        fn as_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    impl<T> From<T> for BareBox<T> {
        #[inline]
        #[must_use]
        fn from(t: T) -> Self {
            BareBox(t)
        }
    }

    impl<T: Clone> Clone for BareBox<T> {
        #[inline]
        #[must_use]
        fn clone(&self) -> Self {
            BareBox(self.0.clone())
        }
    }
    impl<T: Copy> Copy for BareBox<T> {}

    impl<T: Default> Default for BareBox<T> {
        #[inline]
        #[must_use]
        fn default() -> Self {
            BareBox(T::default())
        }
    }

    impl<T: ConstDefault> ConstDefault for BareBox<T> {
        const DEFAULT: Self = BareBox(T::DEFAULT);
    }

    impl<T: PartialEq> PartialEq for BareBox<T> {
        #[inline]
        #[must_use]
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }
    impl<T: Eq> Eq for BareBox<T> {}

    impl<T: PartialOrd> PartialOrd for BareBox<T> {
        #[inline]
        #[must_use]
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl<T: Ord> Ord for BareBox<T> {
        #[inline]
        #[must_use]
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl<T: fmt::Debug> fmt::Debug for BareBox<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl<T: fmt::Display> fmt::Display for BareBox<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }

    impl<T: fmt::Pointer> fmt::Pointer for BareBox<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Pointer::fmt(&self.0, f)
        }
    }

    impl<T: hash::Hash> hash::Hash for BareBox<T> {
        #[inline]
        fn hash<H: hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    impl<T: hash::Hasher> hash::Hasher for BareBox<T> {
        #[inline]
        #[must_use]
        fn finish(&self) -> u64 {
            self.0.finish()
        }
        #[inline]
        fn write(&mut self, bytes: &[u8]) {
            self.0.write(bytes);
        }
    }

    impl<I: Iterator> Iterator for BareBox<I> {
        type Item = I::Item;

        #[inline]
        #[must_use]
        fn next(&mut self) -> Option<I::Item> {
            self.0.next()
        }
        #[inline]
        #[must_use]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
        #[inline]
        #[must_use]
        fn nth(&mut self, n: usize) -> Option<I::Item> {
            self.0.nth(n)
        }
        #[inline]
        #[must_use]
        fn last(self) -> Option<I::Item> {
            self.0.last()
        }
    }
}
