// devela::num::rand::xorshift::u128
//
//! 128-bit + version of XorShift.
//

#[cfg(feature = "alloc")]
use crate::Box;
use crate::{Cast, ConstInit, Own};
#[cfg(feature = "std")]
use crate::{Hasher, HasherBuild, RandomState};

#[doc = crate::_TAG_RAND!()]
/// The `XorShift128+` <abbr title="Pseudo-Random Number Generator">PRNG</abbr>.
#[doc = crate::_doc_location!("num/rand")]
///
/// It has a 128-bit state and generates 64-bit numbers.
///
/// It offers a good balance between quality, speed and state size, and has
/// better statistical properties than [`XorShift128`][crate::XorShift128].
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift128p([u64; 2]);

/// Creates a new PRNG initialized with the default fixed seed.
impl Default for XorShift128p {
    fn default() -> Self {
        Self::INIT
    }
}
/// Creates a new PRNG initialized with the default fixed seed.
impl ConstInit for XorShift128p {
    const INIT: Self = Self::new_unchecked(Self::DEFAULT_SEED);
}

// private associated items
impl XorShift128p {
    const DEFAULT_SEED: [u64; 2] = [0xDEFA_0017_DEFA_0017; 2];

    #[cold] #[allow(dead_code)] #[rustfmt::skip]
    const fn cold_path_default() -> Self { Self::new_unchecked(Self::DEFAULT_SEED) }
}

impl XorShift128p {
    /// Returns a seeded `XorShift128+` generator from the given 2 × 64-bit seeds.
    ///
    /// If the seed is `0`, the default seed is used instead.
    pub const fn new(seeds: [u64; 2]) -> Self {
        if (seeds[0] | seeds[1]) == 0 { Self::cold_path_default() } else { Self(seeds) }
    }

    /// Returns a seeded `XorShift128p` generator from the given 8-bit seed,
    /// unchecked.
    ///
    /// The seeds must not be all `0`, otherwise every result will also be `0`.
    ///
    /// # Panics
    /// Panics in debug if the seeds are all `0`.
    pub const fn new_unchecked(seeds: [u64; 2]) -> Self {
        debug_assert![(seeds[0] | seeds[1]) != 0, "Seeds must be non-zero"];
        Self(seeds)
    }

    #[must_use]
    /// Returns the PRNG's inner state as a raw snapshot.
    pub const fn inner_state(self) -> [u64; 2] {
        self.0
    }
    /// Restores the PRNG from the given state.
    pub const fn from_state(state: [u64; 2]) -> Self {
        Self(state)
    }

    /// Returns the current random `u64`.
    #[must_use]
    pub const fn current_u64(&self) -> u64 {
        self.0[0].wrapping_add(self.0[1])
    }

    /// Returns the next random `u64`.
    #[must_use]
    pub fn next_u64(&mut self) -> u64 {
        let [s0, mut s1] = [self.0[0], self.0[1]];
        let result = s0.wrapping_add(s1);

        s1 ^= s0;
        self.0[0] = s0.rotate_left(55) ^ s1 ^ (s1 << 14); // a, b
        self.0[1] = s1.rotate_left(36); // c

        result
    }

    /// Returns a copy of the next new random state.
    pub const fn peek_next_state(&self) -> Self {
        let mut x = self.0;
        let [s0, mut s1] = [x[0], x[1]];

        s1 ^= s0;
        x[0] = s0.rotate_left(55) ^ s1 ^ (s1 << 14); // a, b
        x[1] = s1.rotate_left(36); // c

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
impl XorShift128p {
    /// Returns a seeded `XorShift128+` generator from the given 128-bit seed.
    ///
    /// The seeds will be split in little endian order.
    pub const fn new1_u128(seed: u128) -> Self {
        Self::new(Cast(seed).into_u64_le())
    }

    /// Returns a seeded `XorShift128+` generator from the given 2 × 64-bit seeds.
    ///
    /// This is an alias of [`new`][Self#method.new].
    pub const fn new2_u64(seeds: [u64; 2]) -> Self {
        Self::new(seeds)
    }

    /// Returns a seeded `XorShift128+` generator from the given 4 × 32-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    pub const fn new4_u32(seeds: [u32; 4]) -> Self {
        Self::new([
            Cast::<u64>::from_u32_le([seeds[0], seeds[1]]),
            Cast::<u64>::from_u32_le([seeds[2], seeds[3]]),
        ])
    }

    /// Returns a seeded `XorShift128+` generator from the given 8 × 16-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    pub const fn new8_u16(seeds: [u16; 8]) -> Self {
        Self::new([
            Cast::<u64>::from_u16_le([seeds[0], seeds[1], seeds[2], seeds[3]]),
            Cast::<u64>::from_u16_le([seeds[4], seeds[5], seeds[6], seeds[7]]),
        ])
    }

    /// Returns a seeded `XorShift128` generator from the given 4 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    pub const fn new16_u8(seeds: [u8; 16]) -> Self {
        let s = seeds;
        Self::new([
            u64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]),
            u64::from_le_bytes([s[8], s[9], s[10], s[11], s[12], s[13], s[14], s[15]]),
        ])
    }
}

#[cfg(feature = "dep_rand_core")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_rand_core")))]
mod impl_rand {
    use super::XorShift128p;
    use crate::_dep::rand_core::{RngCore, SeedableRng};

    impl RngCore for XorShift128p {
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

    impl SeedableRng for XorShift128p {
        type Seed = [u8; 16];

        /// When seeded with zero this implementation uses the default seed
        /// value as the cold path.
        fn from_seed(seed: Self::Seed) -> Self {
            let mut seed_u64s = [0u64; 2];
            if seed == [0; 16] {
                Self::cold_path_default()
            } else {
                for i in 0..2 {
                    seed_u64s[i] = u64::from_le_bytes([
                        seed[i * 8],
                        seed[i * 8 + 1],
                        seed[i * 8 + 2],
                        seed[i * 8 + 3],
                        seed[i * 8 + 4],
                        seed[i * 8 + 5],
                        seed[i * 8 + 6],
                        seed[i * 8 + 7],
                    ]);
                }
                Self::new_unchecked(seed_u64s)
            }
        }
    }
}
