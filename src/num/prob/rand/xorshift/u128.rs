// devela::num::prob::rand::xorshift::u128
//
//! 128-bit version of XorShift.
//

use crate::{Cast, ConstInit, Own};

#[doc = crate::_tags!(rand)]
/// The `XorShift128` <abbr title="Pseudo-Random Number Generator">PRNG</abbr>.
#[doc = crate::_doc_location!("num/rand")]
///
/// It has a 128-bit state and generates 64-bit numbers.
///
/// See also [`XorShift128p`][crate::XorShift128p].
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift128([u32; 4]);

/// Creates a new PRNG initialized with the default fixed seed.
impl Default for XorShift128 {
    fn default() -> Self {
        Self::INIT
    }
}
/// Creates a new PRNG initialized with the default fixed seed.
impl ConstInit for XorShift128 {
    const INIT: Self = Self::new_unchecked(Self::DEFAULT_SEED);
}

// private associated items
impl XorShift128 {
    const DEFAULT_SEED: [u32; 4] = [0xDEFA_0017; 4];

    #[cold] #[allow(dead_code)] #[rustfmt::skip]
    const fn cold_path_default() -> Self { Self::new_unchecked(Self::DEFAULT_SEED) }
}

impl XorShift128 {
    /// Returns a seeded `XorShift128` generator from the given 4 × 32-bit seeds.
    ///
    /// If the seeds are all `0`, the default seed is used instead.
    pub const fn new(seeds: [u32; 4]) -> Self {
        if (seeds[0] | seeds[1] | seeds[2] | seeds[3]) == 0 {
            Self::cold_path_default()
        } else {
            Self([seeds[0], seeds[1], seeds[2], seeds[3]])
        }
    }

    /// Returns a seeded `XorShift128` generator from the given 8-bit seed,
    /// unchecked.
    ///
    /// The seeds must not be all `0`, otherwise every result will also be `0`.
    ///
    /// # Panics
    /// Panics in debug if the seeds are all `0`.
    pub const fn new_unchecked(seeds: [u32; 4]) -> Self {
        debug_assert![(seeds[0] | seeds[1] | seeds[2] | seeds[3]) != 0, "Seeds must be non-zero"];
        Self(seeds)
    }

    #[must_use]
    /// Returns the PRNG's inner state as a raw snapshot.
    pub const fn inner_state(self) -> [u32; 4] {
        self.0
    }
    /// Restores the PRNG from the given state.
    pub const fn from_state(state: [u32; 4]) -> Self {
        Self(state)
    }

    #[must_use]
    /// Returns the current random `u64`.
    pub const fn current_u64(&self) -> u64 {
        ((self.0[0] as u64) << 32) | (self.0[1] as u64)
    }

    #[must_use]
    /// Returns the next random `u64`.
    // Note how the output of the RNG is computed before updating the state,
    // unlike on Xoroshiro128pp, for example.
    pub const fn next_u64(&mut self) -> u64 {
        let t = self.0[3];
        let mut s = self.0[0];
        self.0[3] = self.0[2];
        self.0[2] = self.0[1];
        self.0[1] = s;
        s ^= s << 11;
        s ^= s >> 8;
        self.0[0] = s ^ t ^ (t >> 19);

        ((self.0[0] as u64) << 32) | (self.0[1] as u64)
    }

    /// Returns a copy of the next new random state.
    pub const fn peek_next_state(&self) -> Self {
        let mut x = self.0;

        let t = x[3];
        let mut s = x[0];
        x[3] = x[2];
        x[2] = x[1];
        x[1] = s;
        s ^= s << 11;
        s ^= s >> 8;
        x[0] = s ^ t ^ (t >> 19);

        Self(x)
    }

    /// Returns both the next random state and the `u64` value.
    pub const fn own_next_u64(self) -> Own<Self, u64> {
        let s = self.peek_next_state();
        let v = s.current_u64();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl XorShift128 {
    /// Returns a seeded `XorShift128` generator from the given 128-bit seed.
    ///
    /// The seeds will be split in little endian order.
    pub const fn new1_u128(seed: u128) -> Self {
        Self::new(Cast(seed).into_u32_le())
    }

    /// Returns a seeded `XorShift128` generator from the given 2 × 64-bit seeds.
    ///
    /// The seeds will be split in little endian order.
    pub const fn new2_u64(seeds: [u64; 2]) -> Self {
        let [x, y] = Cast(seeds[0]).into_u32_le();
        let [z, a] = Cast(seeds[1]).into_u32_le();
        Self::new([x, y, z, a])
    }

    /// Returns a seeded `XorShift128` generator from the given 4 × 32-bit seeds.
    ///
    /// This is an alias of [`new`][Self#method.new].
    pub const fn new4_u32(seeds: [u32; 4]) -> Self {
        Self::new(seeds)
    }

    /// Returns a seeded `XorShift128` generator from the given 8 × 16-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    pub const fn new8_u16(seeds: [u16; 8]) -> Self {
        Self::new([
            Cast::<u32>::from_u16_le([seeds[0], seeds[1]]),
            Cast::<u32>::from_u16_le([seeds[2], seeds[3]]),
            Cast::<u32>::from_u16_le([seeds[4], seeds[5]]),
            Cast::<u32>::from_u16_le([seeds[6], seeds[7]]),
        ])
    }

    /// Returns a seeded `XorShift128` generator from the given 16 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    pub const fn new16_u8(seeds: [u8; 16]) -> Self {
        Self::new([
            u32::from_le_bytes([seeds[0], seeds[1], seeds[2], seeds[3]]),
            u32::from_le_bytes([seeds[4], seeds[5], seeds[6], seeds[7]]),
            u32::from_le_bytes([seeds[8], seeds[9], seeds[10], seeds[11]]),
            u32::from_le_bytes([seeds[12], seeds[13], seeds[14], seeds[15]]),
        ])
    }
}

#[cfg(feature = "dep_rand_core")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_rand_core")))]
mod impl_rand {
    use super::XorShift128;
    use crate::_dep::rand_core::{RngCore, SeedableRng};

    impl RngCore for XorShift128 {
        /// Returns the next random `u32`,
        /// from the first 32-bits of `next_u64`.
        fn next_u32(&mut self) -> u32 {
            self.next_u64() as u32
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
    }

    impl SeedableRng for XorShift128 {
        type Seed = [u8; 16];

        /// When seeded with zero this implementation uses the default seed
        /// value as the cold path.
        fn from_seed(seed: Self::Seed) -> Self {
            let mut seed_u32s = [0u32; 4];
            if seed == [0; 16] {
                Self::cold_path_default()
            } else {
                for i in 0..4 {
                    seed_u32s[i] = u32::from_le_bytes([
                        seed[i * 4],
                        seed[i * 4 + 1],
                        seed[i * 4 + 2],
                        seed[i * 4 + 3],
                    ]);
                }
                Self::new_unchecked(seed_u32s)
            }
        }
    }
}
