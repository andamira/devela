// devela::data::uid::pin
//
//!
//

#[cfg(feature = "alloc")]
mod r#box;
#[cfg(feature = "alloc")]
pub use r#box::IdPinBox;

use crate::{Pin, addr_of};

/// A unique identifier based on a pinned stack-allocated reference.
///
/// `IdPin` generates a unique ID by pinning a value on the stack,
/// ensuring that the ID is based on the stack memory address
/// for the lifetime of the reference.
///
/// It doesn't implement `Clone` or `Default`.
///
#[cfg_attr(feature = "alloc", doc = "See also [`IdPinBox`].")]
///
/// # Example
/// ```
/// # use devela::IdPin;
/// let mut data1: u8 = 0;
/// let id1 = IdPin::new(&mut data1);
/// ```
///
#[doc = crate::doc_!(vendor: "object-id")]
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
        addr_of!(*self.inner) as usize
    }
}

mod impl_traits {
    use crate::{IdPin, Ordering, Ptr, impl_trait};

    impl_trait![fmt::Debug for IdPin<'a> |self, f| write!(f, "{}", self.as_usize())];

    impl_trait![Hash for IdPin<'a> |self, s| self.as_usize().hash(s)];

    impl PartialEq for IdPin<'_> {
        fn eq(&self, other: &Self) -> bool {
            Ptr::eq(&*self.inner, &*other.inner)
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
