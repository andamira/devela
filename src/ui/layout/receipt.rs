// devela/ui/layout/receipt.rs
//
//! Defines [`Layout1d`], [`LayoutReceipt`].
//

use crate::{Cmp, Interval, LayoutRect, Lunit, Ordering, UiId};

#[doc = crate::_tags!(layout)]
/// One-axis result of spatial negotiation.
#[doc = crate::_doc_meta!{
    location("ui/layout"),
    test_size_of(Layout1d = 8|64),
}]
/// Records how much one-dimensional space was available and how much was
/// consumed by a placement.
///
/// # Invariants
/// `used <= avail`.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Layout1d {
    /// Total available extent.
    avail: Lunit,
    /// Extent actually consumed by the placement.
    used: Lunit,
}

#[rustfmt::skip]
impl Layout1d {
    /* constructors */

    /// Constructs a layout result.
    ///
    /// # Panics
    /// Panics in debug builds if `used > avail`.
    pub const fn new(avail: Lunit, used: Lunit) -> Self {
        debug_assert![Cmp(used.cmp(avail)).ne(Ordering::Greater)];
        Self { avail, used }
    }
    /// Constructs without enforcing invariants.
    pub const fn new_unchecked(avail: Lunit, used: Lunit) -> Self {
        Self { avail, used }
    }
    /// Constructs, clamping `used` to `avail`.
    pub const fn new_clamped(avail: Lunit, used: Lunit) -> Self {
        let used = if Cmp(used.cmp(avail)).eq(Ordering::Greater) { avail } else { used };
        Self { avail, used }
    }

    /* queries */

    
    /// Returns the total available extent.
    pub const fn avail(&self) -> Lunit { self.avail }
    /// Returns the extent actually consumed by the placement.
    pub const fn used(&self) -> Lunit { self.used }

    /// Returns the remaining free extent after placement.
    pub const fn remaining(&self) -> Lunit { self.avail.sub(self.used) }
    /// Returns whether `requested` fits within the available extent.
    pub const fn fits(&self, requested: Lunit) -> bool {
        Cmp(requested.cmp(self.avail)).ne(Ordering::Greater)
    }

    /// Returns the overflow of `requested` beyond the available extent.
    pub const fn overflow(&self, requested: Lunit) -> Lunit {
        requested.saturating_sub(self.avail)
    }

    /// Returns the interval occupied at `offset`.
    #[must_use]
    pub const fn placed_interval(&self, offset: Lunit) -> Interval<Lunit> {
        Interval::closed_open(offset, offset.add(self.used))
    }
    /// Returns the remaining interval after placement at `offset`.
    #[must_use]
    pub const fn remaining_interval(&self, offset: Lunit) -> Interval<Lunit> {
        Interval::closed_open(offset.add(self.used), offset.add(self.avail))
    }
}

#[doc = crate::_tags!(layout)]
/// Frame-local record of assigned layout space.
#[doc = crate::_doc_meta!{
    location("ui/layout"),
    test_size_of(LayoutReceipt = 24|192),
}]
/// Connects a resolved UI identity with the concrete layout region assigned to it.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutReceipt {
    /// Resolved UI identity.
    id: UiId,
    /// The layout positioned extension.
    rect: LayoutRect,
}
impl LayoutReceipt {
    /// Constructs a layout receipt from an identity and region.
    pub const fn new(id: UiId, rect: LayoutRect) -> Self {
        Self { id, rect }
    }
    /// Returns the UI identity.
    pub const fn id(self) -> UiId {
        self.id
    }
    /// Returns the assigned layout region.
    pub const fn rect(self) -> LayoutRect {
        self.rect
    }
}
