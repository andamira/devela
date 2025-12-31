// devela::sys::mem::borrow::alloc
//
//! Defines the [`Backing`] enum.
//

#[cfg(feature = "alloc")]
use devela::RangeFull;

#[doc = crate::_TAG_MAYBE!()]
#[doc = crate::_TAG_LIFETIME!()]
/// Controls how memory is provided for a value.
///
/// Complements [`MaybeOwned`][super::MaybeOwned] by specifying whether to:
/// - Use an existing mutable buffer (`Buf`)
/// - Allocate new memory (`Alloc`)
///
/// Enables APIs to flexibly accept either caller-provided buffers or fall back to
/// allocations (when `alloc` is enabled)
///
/// # Example
/// ```ignore
/// # use devela::{Backing, MaybeOwned};
/// fn process(mode: Backing<'_>) -> MaybeOwned<'_, str> {
///     match mode {
///         Backing::Buf(buffer) => MaybeOwned::borrowed(/*...*/),
///         Backing::Alloc => MaybeOwned::owned(/*...*/),
///     }
/// }
/// ```
/// ## Features
/// The `Alloc` variant disappears when `alloc` is disabled.
#[derive(Debug, PartialEq)]
pub enum Backing<'a> {
    /// Reuse an existing exclusive buffer.
    Buf(&'a mut [u8]),

    /// Allocate new memory via the global allocator.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    Alloc,
}

impl Backing<'_> {
    /// Returns `true` if this mode requires allocation.
    #[inline(always)]
    pub const fn is_buf(&self) -> bool {
        #[cfg(not(feature = "alloc"))]
        return false;
        #[cfg(feature = "alloc")]
        matches!(self, Self::Alloc)
    }

    /// Returns `true` if this mode uses existing memory.
    #[inline(always)]
    pub const fn is_alloc(&self) -> bool {
        matches!(self, Self::Buf(_))
    }
}

// The following conversions can be used when accepting Into<Backing>.
// The problem is the buffer would get moved if not re-referenced.

impl<'a> From<&'a mut [u8]> for Backing<'a> {
    /// Convert an exclusive slice into borrowing mode.
    fn from(buf: &'a mut [u8]) -> Self {
        Self::Buf(buf)
    }
}
impl<'a, const LEN: usize> From<&'a mut [u8; LEN]> for Backing<'a> {
    /// Convert an exclusive reference to an array into borrowing mode.
    fn from(buf: &'a mut [u8; LEN]) -> Self {
        Self::Buf(buf)
    }
}
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
impl<'a> From<RangeFull> for Backing<'a> {
    /// Convert `..` into allocation mode.
    ///
    /// The range syntax visually suggests "fill everything":
    /// ```ignore
    /// #[cfg(feature = "alloc")]
    /// let s = load_string(..); // Indicates allocation
    /// ```
    fn from(_: RangeFull) -> Self {
        Self::Alloc
    }
}
