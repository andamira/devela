// devela/src/media/visual/image/raster/coverage.rs
//
//! Defines [`Coverage8`].
//

use crate::_impl_init;

#[doc = crate::_tags!(image quant)]
/// Normalized 8-bit raster-sample coverage.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster"),
    test_size_of(Coverage8 = 1|8),
}]
/// `Coverage8` represents how much of one raster sampling footprint
/// is covered by rasterized geometry.
///
/// Its stored value is interpreted as a normalized fraction:
/// ```text
/// value / 255
/// ```
///
/// Therefore:
/// - [`ZERO`][Self::ZERO] means no coverage.
/// - [`FULL`][Self::FULL] means complete coverage.
/// - every intermediate `u8` value is valid.
///
/// Coverage is geometric sampling information. It is not alpha, opacity, luminance,
/// color intensity, or a transfer-encoded component. A later painter or compositor
/// may combine coverage with paint alpha, but that is a separate operation.
///
/// Rasterizers may use wider or higher-precision internal accumulators
/// and quantize to `Coverage8` only when producing their output.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coverage8(u8);

#[rustfmt::skip]
impl Coverage8 {
    /// No coverage.
    pub const ZERO: Self = Self(0);

    /// Complete coverage.
    pub const FULL: Self = Self(u8::MAX);

    /// Creates coverage from its normalized 8-bit representation.
    ///
    /// Every `u8` value is valid.
    #[must_use]
    pub const fn new(value: u8) -> Self { Self(value) }

    /// Returns the normalized 8-bit representation.
    #[must_use]
    pub const fn get(self) -> u8 { self.0 }

    /// Returns whether there is no coverage.
    #[must_use]
    pub const fn is_zero(self) -> bool { self.0 == 0 }

    /// Returns whether coverage is complete.
    #[must_use]
    pub const fn is_full(self) -> bool { self.0 == u8::MAX }
}

_impl_init![Self::ZERO => Coverage8];

impl Default for Coverage8 {
    /// Returns no coverage.
    fn default() -> Self {
        Self::ZERO
    }
}
impl From<u8> for Coverage8 {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}
impl From<Coverage8> for u8 {
    fn from(coverage: Coverage8) -> Self {
        coverage.get()
    }
}

#[cfg(test)]
mod _test {
    use super::*;
    use crate::{ConstInit, const_assert};

    const ZERO: Coverage8 = Coverage8::new(0);
    const MIDDLE: Coverage8 = Coverage8::new(128);
    const FULL: Coverage8 = Coverage8::new(255);

    #[test]
    const fn constants_and_queries_are_const() {
        const_assert!(eq Coverage8::ZERO.get(), 0);
        const_assert!(eq Coverage8::FULL.get(), 255);
        const_assert!(ZERO.is_zero());
        const_assert!(!ZERO.is_full());
        const_assert!(!MIDDLE.is_zero());
        const_assert!(!MIDDLE.is_full());
        const_assert!(eq MIDDLE.get(), 128);
        const_assert!(!FULL.is_zero());
        const_assert!(FULL.is_full());
        const_assert!(eq Coverage8::INIT.get(), 0);
    }
    #[test]
    fn every_encoding_round_trips() {
        let mut value = u8::MIN;
        loop {
            let coverage = Coverage8::new(value);
            assert_eq!(coverage.get(), value);
            assert_eq!(Coverage8::from(value), coverage);
            assert_eq!(u8::from(coverage), value);
            if value == u8::MAX {
                break;
            }
            value += 1;
        }
    }
    #[test]
    fn default_is_zero() {
        assert_eq!(Coverage8::default(), Coverage8::ZERO);
    }
    #[test]
    fn ordering_matches_encoded_coverage() {
        assert!(Coverage8::ZERO < Coverage8::new(1));
        assert!(Coverage8::new(127) < Coverage8::new(128));
        assert!(Coverage8::new(254) < Coverage8::FULL);
    }
}
