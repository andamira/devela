// alazar::xorshift::u64
//
//! 64-bit versions of XorShift generators.
//

#[cfg(feature = "join")]
use crate::Cast;
use crate::{ConstDefault, Own};

/// The `XorShift64` <abbr title="Pseudo-Random Number Generator">PRNG</abbr>.
///
/// It has a 64-bit state and generates 64-bit numbers.
///
/// This is the classic 64-bit *XorShift* algorithm (13, 7, 17),
/// by George Marsaglia.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift64(u64);

impl Default for XorShift64 {
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl ConstDefault for XorShift64 {
    const DEFAULT: Self = Self::new_unchecked(Self::DEFAULT_SEED);
}

// private associated items
impl XorShift64 {
    const DEFAULT_SEED: u64 = 0xDEFA_0017_DEFA_0017;

    #[cold] #[rustfmt::skip]
    const fn cold_path_result() -> Option<Self> { None }
    #[cold] #[allow(dead_code)] #[rustfmt::skip]
    const fn cold_path_default() -> Self { Self::new_unchecked(Self::DEFAULT_SEED) }
}

impl XorShift64 {
    /// Returns a seeded `XorShift64` generator from the given 64-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    #[must_use]
    pub const fn new(seed: u64) -> Option<Self> {
        if seed == 0 {
            Self::cold_path_result()
        } else {
            Some(Self(seed))
        }
    }

    /// Returns a seeded `XorShift64` generator from the given 8-bit seed, unchecked.
    ///
    /// The seed must not be `0`, otherwise every result will also be `0`.
    pub const fn new_unchecked(seed: u64) -> Self {
        debug_assert![seed != 0, "Seed must be non-zero"];
        Self(seed)
    }

    /// Returns the current random `u64`.
    #[must_use]
    pub const fn current_u64(&self) -> u64 {
        self.0
    }

    /// Returns the next random `u64`.
    #[must_use]
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// Returns a copy of the next new random state.
    pub const fn next_state(&self) -> Self {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        Self(x)
    }

    /// Returns both the next random state and the `u64` value.
    pub const fn own_next_u64(self) -> Own<Self, u64> {
        let s = self.next_state();
        let v = s.current_u64();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl XorShift64 {
    /// Returns a seeded `XorShift64` generator from the given 64-bit seed.
    ///
    /// This is an alias of [`new`][Self#method.new].
    pub const fn new1_u64(seed: u64) -> Option<Self> {
        Self::new(seed)
    }

    /// Returns a seeded `XorShift64` generator from the given 2 × 32-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[cfg(feature = "join")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "join")))]
    pub const fn new2_u32(seeds: [u32; 2]) -> Option<Self> {
        Self::new(Cast::<u64>::from_u32_le(seeds))
    }

    /// Returns a seeded `XorShift64` generator from the given 4 × 16-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[cfg(feature = "join")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "join")))]
    pub const fn new4_u16(seeds: [u16; 4]) -> Option<Self> {
        Self::new(Cast::<u64>::from_u16_le(seeds))
    }

    /// Returns a seeded `XorShift64` generator from the given 4 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    pub const fn new8_u8(seeds: [u8; 8]) -> Option<Self> {
        Self::new(u64::from_le_bytes(seeds))
    }
}

#[cfg(feature = "dep_rand_core")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_rand_core")))]
mod impl_rand {
    use super::XorShift64;
    use crate::_dep::rand_core::{Error, RngCore, SeedableRng};

    impl RngCore for XorShift64 {
        /// Returns the next random `u32`,
        /// from the first 32-bits of `next_u64`.
        fn next_u32(&mut self) -> u32 {
            (self.next_u64() & 0xFFFF_FFFF) as u32
        }

        /// Returns the next random `u64`.
        fn next_u64(&mut self) -> u64 {
            self.next_u64()
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            let mut i = 0;
            while i < dest.len() {
                let random_u64 = self.next_u64();
                let bytes = random_u64.to_le_bytes();
                let remaining = dest.len() - i;

                if remaining >= 8 {
                    dest[i..i + 8].copy_from_slice(&bytes);
                    i += 8;
                } else {
                    dest[i..].copy_from_slice(&bytes[..remaining]);
                    break;
                }
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl SeedableRng for XorShift64 {
        type Seed = [u8; 8];

        /// When seeded with zero this implementation uses the default seed
        /// value as the cold path.
        fn from_seed(seed: Self::Seed) -> Self {
            if seed == [0; 8] {
                Self::cold_path_default()
            } else {
                Self::new_unchecked(u64::from_le_bytes(seed))
            }
        }
    }
}
