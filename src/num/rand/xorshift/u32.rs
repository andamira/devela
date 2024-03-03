// devela::num::rand::xorshift::u32
//
//! 32-bit versions of XorShift generators.
//

use crate::{code::ConstDefault, num::Cast, result::Own};

/// The `XorShift32` pseudo-random number generator.
///
/// It has a 32-bit state and generates 32-bit numbers.
///
/// This is the classic 32-bit XorShift algorithm (13, 17, 5),
/// by George Marsaglia.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift32(u32);

impl Default for XorShift32 {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl ConstDefault for XorShift32 {
    const DEFAULT: Self = Self::new_unchecked(Self::DEFAULT_SEED);
}

// private associated items
impl XorShift32 {
    const DEFAULT_SEED: u32 = 0xDEFA0017;

    #[cold]
    #[inline]
    const fn cold_path_result() -> Option<Self> {
        None
    }

    #[cold]
    #[inline]
    #[allow(dead_code)]
    const fn cold_path_default() -> Self {
        Self::new_unchecked(Self::DEFAULT_SEED)
    }
}

impl XorShift32 {
    /// Returns a seeded `XorShift32` generator from the given 32-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    #[inline]
    #[must_use]
    pub const fn new(seed: u32) -> Option<Self> {
        if seed == 0 {
            Self::cold_path_result()
        } else {
            Some(Self(seed))
        }
    }

    /// Returns a seeded `XorShift32` generator from the given 8-bit seed, unchecked.
    ///
    /// The seed must not be `0`, otherwise every result will also be `0`.
    #[inline]
    pub const fn new_unchecked(seed: u32) -> Self {
        debug_assert![seed != 0, "Seed must be non-zero"];
        Self(seed)
    }

    /// Returns the current random `u32`.
    #[inline]
    #[must_use]
    pub const fn current_u32(&self) -> u32 {
        self.0
    }

    /// Returns the next random `u32`.
    //
    // Algorithm "xor" from p. 4 of Marsaglia, "Xorshift RNGs"
    #[inline]
    #[must_use]
    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    /// Returns a copy of the next new random state.
    #[inline]
    pub const fn next_new(&self) -> Self {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        Self(x)
    }

    /// Returns both the next random state and the `u32` value.
    pub const fn own_next_u32(self) -> Own<Self, u32> {
        let s = self.next_new();
        let v = s.current_u32();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl XorShift32 {
    /// Returns a seeded `XorShift32` generator from the given 32-bit seed.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    #[must_use]
    pub const fn new1_u32(seed: u32) -> Option<Self> {
        Self::new(seed)
    }

    /// Returns a seeded `XorShift32` generator from the given 2 × 16-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    #[must_use]
    pub const fn new2_u16(seeds: [u16; 2]) -> Option<Self> {
        Self::new(Cast::<u32>::from_u16_le(seeds))
    }

    /// Returns a seeded `XorShift32` generator from the given 4 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    #[must_use]
    pub const fn new4_u8(seeds: [u8; 4]) -> Option<Self> {
        Self::new(Cast::<u32>::from_u8_le(seeds))
    }
}

#[cfg(feature = "rand_core")]
// #[cfg(any(feature = "rand_core", all(feature = "dep", feature = "num")))] // MAYBE
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rand_core")))]
mod impl_rand {
    use super::{Cast, XorShift32};
    use crate::_deps::rand_core::{Error, RngCore, SeedableRng};

    impl RngCore for XorShift32 {
        /// Returns the next random `u32`.
        #[inline]
        fn next_u32(&mut self) -> u32 {
            self.next_u32()
        }

        /// Returns the next 2 × random `u32` combined as a single `u64`.
        #[inline]
        fn next_u64(&mut self) -> u64 {
            Cast::<u64>::from_u32_le([self.next_u32(), self.next_u32()])
        }

        #[inline]
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            let mut i = 0;
            while i < dest.len() {
                let random_u32 = self.next_u32();
                let bytes = random_u32.to_le_bytes();
                let remaining = dest.len() - i;

                if remaining >= 4 {
                    dest[i..i + 4].copy_from_slice(&bytes);
                    i += 4;
                } else {
                    dest[i..].copy_from_slice(&bytes[..remaining]);
                    break;
                }
            }
        }

        #[inline]
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl SeedableRng for XorShift32 {
        type Seed = [u8; 4];

        /// When seeded with zero this implementation uses the default seed
        /// value as the cold path.
        #[inline]
        fn from_seed(seed: Self::Seed) -> Self {
            if seed == [0; 4] {
                Self::cold_path_default()
            } else {
                Self::new_unchecked(u32::from_le_bytes(seed))
            }
        }
    }
}
