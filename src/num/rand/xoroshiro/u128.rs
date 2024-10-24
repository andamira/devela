// devela::num::rand::xoroshiro::u128
//
//! 128-bit versions of Xoroshiro generators.
//
// TOC
// - Xoroshiro128pp
//   - public definitions
//   - private items and helpers
//   - trait implementations

use crate::{code::ConstDefault, error::Own, num::Cast};

/* public definitions */

/// The `Xoroshiro128++` pseudo-random number generator.
///
/// It has a 128-bit state and generates 32-bit numbers.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Xoroshiro128pp([u32; 4]);

impl Xoroshiro128pp {
    /// Creates a new `Xoroshiro128pp` generator with the given seed.
    ///
    /// Returns `None` if all seeds are zero.
    #[inline]
    #[must_use]
    pub const fn new(seeds: [u32; 4]) -> Option<Self> {
        if (seeds[0] | seeds[1] | seeds[2] | seeds[3]) == 0 {
            Self::cold_path_result()
        } else {
            Some(Self(seeds))
        }
    }

    /// Creates a new Xoroshiro128++ PRNG with the given seed without any checks.
    ///
    /// # Panics
    /// Panics in debug mode if the seeds are all `0`.
    #[inline]
    pub const fn new_unchecked(seeds: [u32; 4]) -> Self {
        debug_assert!((seeds[0] | seeds[1] | seeds[2] | seeds[3]) != 0, "Seeds must be non-zero");
        Self(seeds)
    }

    /// Generates the next random `u32` value.
    // Note how the output of the RNG is computed before updating the state.
    // unlike on Xorshift128, for example.
    #[inline]
    #[must_use]
    pub fn next_u32(&mut self) -> u32 {
        let result = self.current_u32();
        let t = self.0[1] << 9;
        self.0[2] ^= self.0[0];
        self.0[3] ^= self.0[1];
        self.0[1] ^= self.0[2];
        self.0[0] ^= self.0[3];
        self.0[2] ^= t;
        self.0[3] = Self::rotl(self.0[3], 11);
        result
    }

    /// Returns the current random `u32`, without updating the state.
    #[inline]
    #[must_use]
    pub const fn current_u32(self) -> u32 {
        Self::rotl(self.0[0].wrapping_add(self.0[3]), 7).wrapping_add(self.0[0])
    }

    /// Returns a copy of the next new random state.
    #[inline]
    pub const fn copy_next_state(self) -> Self {
        let mut x = self.0;
        let t = x[1] << 9;
        x[2] ^= x[0];
        x[3] ^= x[1];
        x[1] ^= x[2];
        x[0] ^= x[3];
        x[2] ^= t;
        x[3] = Self::rotl(x[3], 11);
        Self(x)
    }

    /// Returns both the next random state and the `u32` value in a tuple.
    #[inline]
    pub fn own_next_u32(self) -> Own<Self, u32> {
        let next_state = self.copy_next_state();
        let next_value = next_state.current_u32();
        Own::new(next_state, next_value)
    }

    /// The jump function for the generator, equivalent to 2^64 `next_u32` calls.
    #[inline]
    pub fn jump(&mut self) {
        self.jump_with_constant(Self::JUMP);
    }

    /// The long jump function for the generator, equivalent to 2^96 `next_u32` calls.
    #[inline]
    pub fn long_jump(&mut self) {
        self.jump_with_constant(Self::LONG_JUMP);
    }

    /// Returns a copy of the state jumped ahead by 2^64 steps.
    #[inline]
    pub const fn copy_jump(self) -> Self {
        self.copy_jump_with_constant(Self::JUMP)
    }

    /// Returns a copy of the state long-jumped ahead by 2^96 steps.
    #[inline]
    pub const fn copy_long_jump(self) -> Self {
        self.copy_jump_with_constant(Self::LONG_JUMP)
    }
}

/// # Extra constructors
impl Xoroshiro128pp {
    /// Returns a seeded `Xoroshiro128pp` generator from the given 128-bit seed.
    ///
    /// The seeds will be split in little endian order.
    #[inline]
    pub const fn new1_u128(seed: u128) -> Option<Self> {
        Self::new(Cast(seed).into_u32_le())
    }

    /// Returns a seeded `Xoroshiro128pp` generator from the given 2 × 64-bit seeds.
    ///
    /// The seeds will be split in little endian order.
    #[inline]
    pub const fn new2_u64(seeds: [u64; 2]) -> Option<Self> {
        let [x, y] = Cast(seeds[0]).into_u32_le();
        let [z, a] = Cast(seeds[1]).into_u32_le();
        Self::new([x, y, z, a])
    }

    /// Returns a seeded `Xoroshiro128pp` generator from the given 4 × 32-bit seeds.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    pub const fn new4_u32(seeds: [u32; 4]) -> Option<Self> {
        Self::new(seeds)
    }

    /// Returns a seeded `Xoroshiro128pp` generator from the given 8 × 16-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new8_u16(seeds: [u16; 8]) -> Option<Self> {
        Self::new([
            Cast::<u32>::from_u16_le([seeds[0], seeds[1]]),
            Cast::<u32>::from_u16_le([seeds[2], seeds[3]]),
            Cast::<u32>::from_u16_le([seeds[4], seeds[5]]),
            Cast::<u32>::from_u16_le([seeds[6], seeds[7]]),
        ])
    }

    /// Returns a seeded `Xoroshiro128pp` generator from the given 16 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new16_u8(seeds: [u8; 16]) -> Option<Self> {
        Self::new([
            Cast::<u32>::from_u8_le([seeds[0], seeds[1], seeds[2], seeds[3]]),
            Cast::<u32>::from_u8_le([seeds[4], seeds[5], seeds[6], seeds[7]]),
            Cast::<u32>::from_u8_le([seeds[8], seeds[9], seeds[10], seeds[11]]),
            Cast::<u32>::from_u8_le([seeds[12], seeds[13], seeds[14], seeds[15]]),
        ])
    }
}

/* private items and helpers */

impl Xoroshiro128pp {
    const DEFAULT_SEED: [u32; 4] = [0xDEFA_0017; 4];
    const JUMP: [u32; 4] = [0x8764_000b, 0xf542_d2d3, 0x6fa0_35c3, 0x77f2_db5b];
    const LONG_JUMP: [u32; 4] = [0xb523_952e, 0x0b6f_099f, 0xccf_5a0ef, 0x1c58_0662];

    #[cold] #[rustfmt::skip]
    const fn cold_path_result() -> Option<Self> { None }
    #[cold] #[allow(dead_code)] #[rustfmt::skip]
    const fn cold_path_default() -> Self { Self::new_unchecked(Self::DEFAULT_SEED) }

    // rotates `x` left by `k` bits.
    #[inline]
    const fn rotl(x: u32, k: i32) -> u32 {
        (x << k) | (x >> (32 - k))
    }

    #[inline]
    fn jump_with_constant(&mut self, jump: [u32; 4]) {
        let (mut s0, mut s1, mut s2, mut s3) = (0, 0, 0, 0);
        for &j in jump.iter() {
            for b in 0..32 {
                if (j & (1 << b)) != 0 {
                    s0 ^= self.0[0];
                    s1 ^= self.0[1];
                    s2 ^= self.0[2];
                    s3 ^= self.0[3];
                }
                let _ = self.next_u32();
            }
        }
        self.0 = [s0, s1, s2, s3];
    }

    #[inline]
    const fn copy_jump_with_constant(self, jump: [u32; 4]) -> Self {
        let (mut s0, mut s1, mut s2, mut s3) = (0, 0, 0, 0);
        let mut state = self;
        let mut i = 0;
        while i < jump.len() {
            let mut b = 0;
            while b < 32 {
                if (jump[i] & (1 << b)) != 0 {
                    s0 ^= state.0[0];
                    s1 ^= state.0[1];
                    s2 ^= state.0[2];
                    s3 ^= state.0[3];
                }
                state = state.copy_next_state();
                b += 1;
            }
            i += 1;
        }
        Self([s0, s1, s2, s3])
    }
}

/* trait implementations */

impl Default for Xoroshiro128pp {
    #[inline]
    fn default() -> Self {
        Self::new_unchecked(Self::DEFAULT_SEED)
    }
}
impl ConstDefault for Xoroshiro128pp {
    const DEFAULT: Self = Self::new_unchecked(Self::DEFAULT_SEED);
}

#[cfg(feature = "dep_rand_core")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_rand_core")))]
mod impl_rand {
    use super::{Cast, Xoroshiro128pp};
    use crate::_dep::rand_core::{Error, RngCore, SeedableRng};

    impl RngCore for Xoroshiro128pp {
        /// Returns the next random `u32`.
        #[inline]
        fn next_u32(&mut self) -> u32 {
            self.next_u32()
        }

        /// Returns the next random `u64`.
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

    impl SeedableRng for Xoroshiro128pp {
        type Seed = [u8; 16];

        /// When seeded with zero this implementation uses the default seed
        /// value as the cold path.
        #[inline]
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
