// devela::data::id::pin
//
//!
//

#[cfg(feature = "alloc")]
mod r#box;
#[cfg(feature = "alloc")]
pub use r#box::IdPinBox;

use core::{pin::Pin, ptr};

/// A unique identifier based on a pinned stack-allocated reference.
///
/// `IdPin` generates a unique ID by pinning a value on the stack,
/// ensuring that the ID is based on the stack memory address
/// for the lifetime of the reference.
///
/// It doesn't implement `Clone` or `Default`.
///
/// # Example
/// ```
/// # use devela::IdPin;
/// let mut data1: u8 = 0;
/// let id1 = IdPin::new(&mut data1);
/// ```
pub struct IdPin<'a> {
    inner: Pin<&'a mut u8>,
}

impl<'a> IdPin<'a> {
    /// Creates a new `IdPin` with a unique stack memory address.
    ///
    /// Expects a mutable reference to a `u8` `data` that will be pinned.
    pub fn new(data: &'a mut u8) -> Self {
        let inner = Pin::new(data);
        Self { inner }
    }

    /// Returns the unique ID as a `usize`, derived from the stack memory address.
    pub fn as_usize(&self) -> usize {
        ptr::addr_of!(*self.inner) as usize
    }
}

mod impl_traits {
    use super::IdPin;
    use core::{cmp::Ordering, fmt, hash, ptr};

    impl fmt::Debug for IdPin<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.as_usize())
        }
    }
    impl hash::Hash for IdPin<'_> {
        fn hash<H: hash::Hasher>(&self, state: &mut H) {
            self.as_usize().hash(state);
        }
    }

    impl PartialEq for IdPin<'_> {
        fn eq(&self, other: &Self) -> bool {
            ptr::eq(&*self.inner, &*other.inner)
        }
    }
    impl Eq for IdPin<'_> {}

    impl PartialOrd for IdPin<'_> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.as_usize().cmp(&other.as_usize()))
        }
    }
    impl Ord for IdPin<'_> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.as_usize().cmp(&other.as_usize())
        }
    }
}
