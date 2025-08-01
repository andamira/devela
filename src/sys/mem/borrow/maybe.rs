// devela::sys::mem::borrow::maybe
//
//! Defines [`MaybeOwned`].
//

#[cfg(all(doc, feature = "alloc"))]
use devela::Cow;
#[cfg(feature = "alloc")]
use devela::{Box, ToOwned};
use devela::{Deref, Hash, Hasher, Ordering};

/// A container that may hold either a borrowed or owned value, with zero-cost borrowing.
///
/// This is similar to [`Cow`][], but more flexible:
/// - Works with any `T: ?Sized` (including `str`, `[T]`, and trait objects)
/// - Doesn't require `T: ToOwned` unless converting borrowed→owned
///
/// # Features
/// If the `alloc` feature is disabled it can only contain borrowed values.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum MaybeOwned<'a, T: ?Sized> {
    /// A borrowed value.
    Borrowed(&'a T),
    /// An owned, heap-allocated value.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    Owned(Box<T>),
}

impl<'a, T: ?Sized> MaybeOwned<'a, T> {
    /// Returns `true` if the value is borrowed.
    pub const fn is_borrowed(&self) -> bool {
        matches!(self, MaybeOwned::Borrowed(_))
    }

    /// Returns `true` if the value is owned.
    pub const fn is_owned(&self) -> bool {
        #[cfg(not(feature = "alloc"))]
        return false;
        #[cfg(feature = "alloc")]
        matches!(self, MaybeOwned::Owned(_))
    }

    /// Returns a reference to the contained value, whether borrowed or owned.
    pub const fn borrowed(&self) -> &T {
        match self {
            MaybeOwned::Borrowed(r) => r,
            #[cfg(feature = "alloc")]
            MaybeOwned::Owned(b) => b,
        }
    }

    /// Converts the container into an owned `Box<T>` if it isn't already.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn owned(self) -> Box<T>
    where
        T: ToOwned<Owned = Box<T>>,
    {
        match self {
            MaybeOwned::Borrowed(r) => r.to_owned(),
            MaybeOwned::Owned(b) => b,
        }
    }

    /// Converts the container into an owned value.
    ///
    /// If the value is borrowed, it will be cloned using `ToOwned`.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn into_owned(self) -> T::Owned
    where
        T: ToOwned<Owned = Box<T>>,
    {
        match self {
            MaybeOwned::Borrowed(t) => t.to_owned(),
            MaybeOwned::Owned(t) => t,
        }
    }
}

/* impl traits */

// AsRef, Deref
impl<'a, T: ?Sized> AsRef<T> for MaybeOwned<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            MaybeOwned::Borrowed(t) => t,
            #[cfg(feature = "alloc")]
            MaybeOwned::Owned(t) => t.as_ref(),
        }
    }
}
impl<'a, T: ?Sized> Deref for MaybeOwned<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.borrowed()
    }
}

// Eq, Ord, Hash
impl<'a, T: ?Sized + PartialEq> PartialEq for MaybeOwned<'a, T> {
    fn eq(&self, other: &MaybeOwned<'a, T>) -> bool {
        self.as_ref() == other.as_ref()
    }
}
impl<'a, T: ?Sized + Eq> Eq for MaybeOwned<'a, T> {}
impl<'a, T: ?Sized + PartialOrd> PartialOrd for MaybeOwned<'a, T> {
    fn partial_cmp(&self, other: &MaybeOwned<'a, T>) -> Option<Ordering> {
        self.as_ref().partial_cmp(other)
    }
}
impl<'a, T: ?Sized + Ord> Ord for MaybeOwned<'a, T> {
    fn cmp(&self, other: &MaybeOwned<'a, T>) -> Ordering {
        self.as_ref().cmp(other)
    }
}
impl<'a, T: ?Sized + Hash> Hash for MaybeOwned<'a, T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_ref().hash(hasher);
    }
}

// From
impl<'a, T: ?Sized> From<&'a T> for MaybeOwned<'a, T> {
    fn from(value: &'a T) -> Self {
        MaybeOwned::Borrowed(value)
    }
}
#[cfg(feature = "alloc")]
impl<'a, T: ?Sized> From<Box<T>> for MaybeOwned<'a, T> {
    fn from(value: Box<T>) -> Self {
        MaybeOwned::Owned(value)
    }
}
