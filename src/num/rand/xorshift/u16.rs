// devela::num::rand::xorshift::u16
//
//! 16-bit versions of XorShift generators.
//

use crate::{Cast, ConstDefault, Own};

/// The `XorShift16` pseudo-random number generator.
///
/// It has a 16-bit state and generates 16-bit numbers.
///
/// This is John Metcalf's 16-bit (7, 8, 9) version of George Marsaglia's
/// original [`XorShift32`][super::XorShift32].
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift16(u16);

impl Default for XorShift16 {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl ConstDefault for XorShift16 {
    const DEFAULT: Self = Self::new_unchecked(Self::DEFAULT_SEED);
}

// private associated items
impl XorShift16 {
    const DEFAULT_SEED: u16 = 0xDEFA;

    #[cold] #[rustfmt::skip]
    const fn cold_path_result() -> Option<Self> { None }
    #[cold] #[allow(dead_code)] #[rustfmt::skip]
    const fn cold_path_default() -> Self { Self::new_unchecked(Self::DEFAULT_SEED) }
}

impl XorShift16 {
    /// Returns a seeded `XorShift16` generator from the given 16-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    #[inline]
    #[must_use]
    pub const fn new(seed: u16) -> Option<Self> {
        if seed == 0 {
            Self::cold_path_result()
        } else {
            Some(Self(seed))
        }
    }

    /// Returns a seeded `XorShift16` generator from the given 8-bit seed, unchecked.
    ///
    /// The seed must not be `0`, otherwise every result will also be `0`.
    #[inline]
    pub const fn new_unchecked(seed: u16) -> Self {
        debug_assert![seed != 0, "Seed must be non-zero"];
        Self(seed)
    }

    /// Returns the current random `u16`.
    #[inline]
    #[must_use]
    pub const fn current_u16(&self) -> u16 {
        self.0
    }

    /// Returns the next random `u16`.
    ///
    #[inline]
    #[must_use]
    pub fn next_u16(&mut self) -> u16 {
        let mut x = self.0;
        x ^= x << 7;
        x ^= x >> 9;
        x ^= x << 8;
        self.0 = x;
        x
    }

    /// Returns a copy of the next new random state.
    #[inline]
    pub const fn next_new(&self) -> Self {
        let mut x = self.0;
        x ^= x << 7;
        x ^= x >> 9;
        x ^= x << 8;
        Self(x)
    }

    /// Returns both the next random state and the `u16` value.
    pub const fn own_next_u16(self) -> Own<Self, u16> {
        let s = self.next_new();
        let v = s.current_u16();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl XorShift16 {
    /// Returns a seeded `XorShift16` generator from the given 16-bit seed.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    pub const fn new1_u16(seed: u16) -> Option<Self> {
        Self::new(seed)
    }

    /// Returns a seeded `XorShift16` generator from the given 2 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    #[must_use]
    pub const fn new2_u8(seeds: [u8; 2]) -> Option<Self> {
        Self::new(Cast::<u16>::from_u8_le(seeds))
    }
}

#[cfg(feature = "dep_rand_core")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_rand_core")))]
mod impl_rand {
    use super::{Cast, XorShift16};
    use crate::_dep::rand_core::{Error, RngCore, SeedableRng};

    impl RngCore for XorShift16 {
        /// Returns the next 2 × random `u16` combined as a single `u32`.
        #[inline]
        fn next_u32(&mut self) -> u32 {
            Cast::<u32>::from_u16_le([self.next_u16(), self.next_u16()])
        }

        /// Returns the next 4 × random `u16` combined as a single `u64`.
        #[inline]
        fn next_u64(&mut self) -> u64 {
            Cast::<u64>::from_u16_le([
                self.next_u16(),
                self.next_u16(),
                self.next_u16(),
                self.next_u16(),
            ])
        }

        #[inline]
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            let mut i = 0;
            while i < dest.len() {
                let random_u16 = self.next_u16();
                let bytes = random_u16.to_le_bytes();
                let remaining = dest.len() - i;

                if remaining >= 2 {
                    dest[i] = bytes[0];
                    dest[i + 1] = bytes[1];
                    i += 2;
                } else {
                    dest[i] = bytes[0];
                    i += 1;
                }
            }
        }

        #[inline]
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl SeedableRng for XorShift16 {
        type Seed = [u8; 2];

        /// When seeded with zero this implementation uses the default seed
        /// value as the cold path.
        #[inline]
        fn from_seed(seed: Self::Seed) -> Self {
            if seed == [0; 2] {
                Self::cold_path_default()
            } else {
                Self::new_unchecked(u16::from_le_bytes(seed))
            }
        }
    }
}
