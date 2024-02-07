// ladata::mem::storage::bare
//
//! *Bare* storage doesn't affect its contents
//
// API based on https://doc.rust-lang.org/alloc/boxed/struct.Box.html

use crate::mem::{Bare, Storage};
#[cfg(all(doc, feature = "alloc"))]
use crate::{_deps::alloc::boxed::Box, mem::Boxed};

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

/// A marker struct for a storage type that wraps its data in a [`BareBox`].
///
/// This implementation is equivalent to the one for [`Boxed`] which uses [`Box`] for storage.
impl Storage for Bare {
    type Stored<T> = BareBox<T>;

    fn name() -> &'static str {
        "BareBox"
    }
}

impl<T> BareBox<T> {
    /// Creates a new `BareBox` storage for the given `t`.
    #[inline]
    #[must_use]
    pub const fn new(t: T) -> Self {
        BareBox(t)
    }

    /// Returns the inner stored type.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> T {
        self.0
    }
}
impl<T: Copy> BareBox<T> {
    /// Returns the inner stored type in compile-time.
    #[inline]
    #[must_use]
    pub const fn into_inner_const(self) -> T {
        self.0
    }
}

mod core_impls {
    use super::BareBox;
    use core::{cmp, fmt, hash, ops};

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
            self.0.write(bytes)
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
