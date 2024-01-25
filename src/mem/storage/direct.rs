// ladata::mem::storage::direct
//
//! Direct storage doesn't affect its content.
//
// API based on https://doc.rust-lang.org/alloc/boxed/struct.Box.html

/// A no-op pointer type, like a [`Box`] that doesn't affect how `T` is stored.
///
/// # Examples
/// ```
/// use devela::mem::Direct;
///
/// let byte = Direct::new(0_u8);
/// ```
pub struct Direct<T>(pub T);

impl<T> Direct<T> {
    /// New `Direct` storage.
    #[inline]
    #[must_use]
    #[allow(unused)]
    pub const fn new(t: T) -> Self {
        Direct(t)
    }
}

mod core_impls {
    use super::Direct;
    use core::{cmp, fmt, hash, ops};

    impl<T> ops::Deref for Direct<T> {
        type Target = T;
        #[inline]
        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T> ops::DerefMut for Direct<T> {
        #[inline]
        #[must_use]
        fn deref_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    impl<T> From<T> for Direct<T> {
        #[inline]
        #[must_use]
        fn from(t: T) -> Self {
            Direct(t)
        }
    }

    impl<T: Clone> Clone for Direct<T> {
        #[inline]
        #[must_use]
        fn clone(&self) -> Self {
            Direct(self.0.clone())
        }
    }
    impl<T: Copy> Copy for Direct<T> {}

    impl<T: Default> Default for Direct<T> {
        #[inline]
        #[must_use]
        fn default() -> Self {
            Direct(T::default())
        }
    }

    impl<T: PartialEq> PartialEq for Direct<T> {
        #[inline]
        #[must_use]
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }
    impl<T: Eq> Eq for Direct<T> {}

    impl<T: PartialOrd> PartialOrd for Direct<T> {
        #[inline]
        #[must_use]
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl<T: Ord> Ord for Direct<T> {
        #[inline]
        #[must_use]
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl<T: fmt::Debug> fmt::Debug for Direct<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl<T: fmt::Display> fmt::Display for Direct<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }

    impl<T: fmt::Pointer> fmt::Pointer for Direct<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Pointer::fmt(&self.0, f)
        }
    }

    impl<T: hash::Hash> hash::Hash for Direct<T> {
        #[inline]
        fn hash<H: hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    impl<T: hash::Hasher> hash::Hasher for Direct<T> {
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

    impl<I: Iterator> Iterator for Direct<I> {
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
