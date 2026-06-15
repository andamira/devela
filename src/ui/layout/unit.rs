// devela/ui/layout/unit.rs
//
//! Defines metric aliases and [`Lunit`].
//

use crate::{
    Ordering::{self, Equal, Greater, Less},
    unwrap,
};

#[doc = crate::_tags!(layout)]
/// A 2-dimensional layout extent.
pub type LayoutExt = crate::Extent2<Lunit>;
#[doc = crate::_tags!(layout)]
/// A 2-dimensional layout position.
pub type LayoutPos = crate::Position2<Lunit>;
#[doc = crate::_tags!(layout)]
/// A 2-dimensional layout region.
pub type LayoutRect = crate::RegionS2<Lunit>;
#[doc = crate::_tags!(layout)]
/// A 2-dimensional layout stride.
pub type LayoutStride = crate::Stride2<Lunit>;

#[doc = crate::_tags!(layout)]
/// Scalar unit of UI layout negotiation.
#[doc = crate::_doc_meta!{
    location("ui/layout"),
    test_size_of(Lunit = 4|32),
}]
/// Represents one abstract layout coordinate or extent component
/// after a presentation context has chosen its working coordinate space.
///
/// A layout unit is not necessarily a physical pixel,
/// device-independent pixel, CSS pixel, text cell, or game pixel.
/// It is the scalar currency used by UI layout negotiation.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lunit(u32);

#[rustfmt::skip]
impl Lunit {
    /// The zero layout unit.
    pub const ZERO: Self = Self(0);

    /// The largest representable layout unit.
    pub const MAX: Self = Self(u32::MAX);

    /// Constructs a layout unit from its raw integer value.
    pub const fn new(raw_units: u32) -> Self { Self(raw_units) }

    /// Returns the raw integer value.
    pub const fn raw(self) -> u32 { self.0 }

    /* arithmetic */

    /// Adds two layout units using primitive integer addition semantics.
    pub const fn add(self, other: Self) -> Self { Self(self.0 + other.0) }
    /// Adds two layout units, returning `None` if overflow ocurred.
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self(unwrap![some? self.0.checked_add(other.0)]))
    }
    /// Adds two layout units, saturating to the maximum value.
    pub const fn saturating_add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    /// Subtracts two layout units using primitive integer subtraction semantics.
    pub const fn sub(self, other: Self) -> Self { Self(self.0 - other.0) }
    /// Subtracts two layout units, returning `None` if overflow ocurred.
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        Some(Self(unwrap![some? self.0.checked_sub(other.0)]))
    }
    /// Subtracts two layout units, saturating to zero.
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }

    /* comparison */

    /// Equality comparison.
    pub const fn eq(self, other: Self) -> bool { self.0 == other.0 }
    /// Compares two layout units.
    pub const fn cmp(self, other: Self) -> Ordering {
        if self.0 < other.0 { Less } else if self.0 > other.0 { Greater } else { Equal }
    }
}
#[rustfmt::skip]
mod impls {
    use crate::Lunit;

    impl From<u32> for Lunit { fn from(from: u32) -> Self { Self(from) } }
    impl From<Lunit> for u32 { fn from(from: Lunit) -> Self { from.0 } }
    impl PartialEq<u32> for Lunit { fn eq(&self, other: &u32) -> bool { self.0 == *other } }
    impl PartialEq<Lunit> for u32 { fn eq(&self, other: &Lunit) -> bool { *self == other.0 } }

    // Intentionally not implementing Add/Sub yet.
    // Layout arithmetic will often wantexplicit checked or saturating behavior.
    //
    // impl Add for Lunit { type Output = Self;
    //     fn add(self, other: Self) -> Self { self.add(other) } }
    // impl Sub for Lunit { type Output = Self;
    //     fn sub(self, other: Self) -> Self { self.sub(other) } }
}
