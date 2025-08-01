// devela::sys::mem::borrow::alloc
//
//! Defines the [`AllocMode`] enum.
//

use devela::RangeFull;

/// Controls how values should be acquired, between borrowing and allocation.
///
/// Complements [`MaybeOwned`] by controlling the *acquisition strategy* for values.
///
/// # Philosophy
/// Like Rust's `Cow` but for acquisition rather than storage. Enables APIs to:
/// - Be allocation-aware without duplication
/// - Support `no_std` environments gracefully
/// - Allow caller-controlled optimization
///
/// # Usage Patterns
///
/// ## Basic Flow
/// ```ignore
/// # use devela::{AllocMode, MaybeOwned};
/// fn fetch(mode: impl Into<AllocMode<'_>>) -> MaybeOwned<'_, str> {
///     match mode.into() {
///         AllocMode::Borrowed(buf) => { /* ... */ },
///         AllocMode::Heap => { /* ... */ },
///     }
/// }
/// ```
/// ## Features
/// The `Heap` variant disappears when `alloc` is disabled.
#[non_exhaustive]
#[derive(Debug)]
pub enum AllocMode<'a> {
    /// Borrow existing storage. The zero-cost choice.
    ///
    /// Uses a mutable reference to avoid allocations, ideal for:
    /// - Stack buffers
    /// - Reusable scratch space
    /// - Caller-managed lifetimes
    /// # Example
    /// ```ignore
    /// let mut buf = [0u8; 256];
    /// let data = get_data(&mut buf); // No allocation
    /// ```
    Borrowed(&'a mut [u8]),

    /// Allocate new memory. The flexible choice.
    ///
    /// Requires the `alloc` feature. Preferred when:
    /// - No suitable buffer exists
    /// - Lifetime extension is needed
    /// - Convenience outweighs performance
    ///
    /// # Example
    /// ```ignore
    /// #[cfg(feature = "alloc")]
    /// let data = get_data(..); // Allocates as needed
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    Heap,
    // MAYBE
    // Custom(),
}

/* Conversions with clear semantics */

impl<'a> From<&'a mut [u8]> for AllocMode<'a> {
    /// Convert an exclusive slice into borrowing mode.
    fn from(buf: &'a mut [u8]) -> Self {
        Self::Borrowed(buf)
    }
}
impl<'a, const LEN: usize> From<&'a mut [u8; LEN]> for AllocMode<'a> {
    /// Convert an exclusive reference to an array into borrowing mode.
    fn from(buf: &'a mut [u8; LEN]) -> Self {
        Self::Borrowed(buf)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
impl<'a> From<RangeFull> for AllocMode<'a> {
    /// Convert `..` into allocation mode.
    ///
    /// The range syntax visually suggests "fill everything":
    /// ```ignore
    /// #[cfg(feature = "alloc")]
    /// let s = load_string(..); // Clearly indicates allocation
    /// ```
    fn from(_: RangeFull) -> Self {
        Self::Heap
    }
}

impl AllocMode<'_> {
    /// Returns `true` if this mode requires allocation.
    #[inline(always)]
    pub fn is_heap(&self) -> bool {
        #[cfg(not(feature = "alloc"))]
        return false;
        #[cfg(feature = "alloc")]
        matches!(self, Self::Heap)
    }

    /// Returns `true` if this mode uses existing memory.
    #[inline(always)]
    pub fn is_borrowed(&self) -> bool {
        matches!(self, Self::Borrowed(_))
    }
}
