// devela/src/media/visual/image/raster/coverage.rs
//
//! Defines [`Coverage8`].
//

use crate::{_impl_init, word};

#[doc = crate::_tags!(image quant)]
/// Normalized 8-bit raster-sample coverage.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster", struct Coverage8),
    test_size_of(Coverage8 = 1|8; niche !Option),
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
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coverage8(u8);

#[rustfmt::skip]
impl Coverage8 {
    /// No coverage.
    pub const ZERO: Self = Self(0);

    /// Complete coverage.
    pub const FULL: Self = Self(u8::MAX);

    /* constructors */

    /// Creates coverage from its normalized 8-bit representation.
    ///
    /// Every `u8` value is valid.
    pub const fn new(value: u8) -> Self { Self(value) }

    /* queries */

    /// Returns the normalized 8-bit representation.
    #[must_use]
    pub const fn get(self) -> u8 { self.0 }

    /// Returns whether there is no coverage.
    #[must_use]
    pub const fn is_zero(self) -> bool { self.0 == 0 }

    /// Returns whether coverage is complete.
    #[must_use]
    pub const fn is_full(self) -> bool { self.0 == u8::MAX }

    /* arithmetic */

    /// Returns the complementary coverage.
    ///
    /// The result represents `1 - coverage`.
    pub const fn complement(self) -> Self { Self(u8::MAX - self.0) }

    /// Returns the normalized product of two coverage factors.
    ///
    /// The exact real-valued operation is:
    /// ```text
    /// (self / 255) × (other / 255)
    /// ```
    /// and the result is rounded to the nearest representable `Coverage8`.
    ///
    /// This is numerical modulation of coverage factors,
    /// not a geometric intersection operation.
    pub const fn product(self, other: Self) -> Self {
        Self(Self::mul_unorm8(self.0, other.0))
    }
    /// Scales an unsigned normalized 8-bit value by this coverage.
    ///
    /// `value` is interpreted as a normalized scalar in `0..=255`.
    ///
    /// The result is rounded to the nearest representable `u8`.
    #[must_use]
    pub const fn scale_u8(self, value: u8) -> u8 {
        Self::mul_unorm8(self.0, value)
    }

    /* private */

    // Divide-free form of the reference formula:
    // ((a as u16 * b as u16 + 127) / 255) as u8
    //
    // Can't panic: 255 × 255 = 65025 && 65025 + 128 = 65153
    const fn mul_unorm8(a: u8, b: u8) -> u8 {
        let product = a as u16 * b as u16;
        let biased = product + 128;
        ((biased + (biased >> 8)) >> 8) as u8
    }
}

word! { impl Coverage8(u8); }

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
    use crate::{ConstInit, const_assert, is};

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
    const fn arithmetic_is_const() {
        const_assert!(eq Coverage8::ZERO.complement().get(), 255);
        const_assert!(eq Coverage8::FULL.complement().get(), 0);
        const_assert!(eq Coverage8::new(128).complement().get(), 127);
        const_assert!(eq Coverage8::new(137).product(Coverage8::FULL).get(), 137);
        const_assert!(eq Coverage8::new(137).product(Coverage8::ZERO).get(), 0);
        const_assert!(eq Coverage8::new(128).product(Coverage8::new(128)).get(), 64);
        const_assert!(eq Coverage8::new(128).scale_u8(200), 100);
    }
    #[test]
    fn every_encoding_round_trips() {
        let mut value = u8::MIN;
        loop {
            let coverage = Coverage8::new(value);
            assert_eq!(coverage.get(), value);
            assert_eq!(Coverage8::from(value), coverage);
            assert_eq!(u8::from(coverage), value);
            is! { value == u8::MAX, break }
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
    #[test]
    fn complement_is_exact_and_involutive() {
        let mut value = u8::MIN;
        loop {
            let coverage = Coverage8::new(value);
            let complement = coverage.complement();
            assert_eq!(value as u16 + complement.get() as u16, u8::MAX as u16);
            assert_eq!(complement.complement(), coverage);
            is! { value == u8::MAX, break }
            value += 1;
        }
    }
    #[test]
    fn product_matches_rounded_normalized_reference() {
        for a in u8::MIN..=u8::MAX {
            for b in u8::MIN..=u8::MAX {
                let expected = ((a as u16 * b as u16 + 127) / 255) as u8;
                let ca = Coverage8::new(a);
                let cb = Coverage8::new(b);
                assert_eq!(ca.product(cb).get(), expected);
                assert_eq!(ca.scale_u8(b), expected);
                assert_eq!(ca.product(cb), cb.product(ca));
            }
        }
    }
    #[test]
    fn product_has_zero_and_full_identities() {
        for value in u8::MIN..=u8::MAX {
            let coverage = Coverage8::new(value);
            assert_eq!(coverage.product(Coverage8::ZERO), Coverage8::ZERO);
            assert_eq!(coverage.product(Coverage8::FULL), coverage);
            assert_eq!(Coverage8::ZERO.product(coverage), Coverage8::ZERO);
            assert_eq!(Coverage8::FULL.product(coverage), coverage);
        }
    }
}
