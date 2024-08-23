// devela::num::rand::xorshift::u128
//
//! 128-bit versions of XorShift generators.
//

use crate::{code::ConstDefault, error::Own, num::Cast};

/// The `XorShift128` pseudo-random number generator.
///
/// It has a 128-bit state and generates 64-bit numbers.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift128([u32; 4]);

impl Default for XorShift128 {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl ConstDefault for XorShift128 {
    const DEFAULT: Self = Self::new_unchecked(Self::DEFAULT_SEED);
}

// private associated items
impl XorShift128 {
    const DEFAULT_SEED: [u32; 4] = [0xDEFA_0017; 4];

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

impl XorShift128 {
    /// Returns a seeded `XorShift128` generator from the given 4 × 32-bit seeds.
    ///
    /// Returns `None` if all given seeds are `0`.
    #[inline]
    #[must_use]
    pub const fn new(seeds: [u32; 4]) -> Option<Self> {
        if (seeds[0] | seeds[1] | seeds[2] | seeds[3]) == 0 {
            Self::cold_path_result()
        } else {
            Some(Self([seeds[0], seeds[1], seeds[2], seeds[3]]))
        }
    }

    /// Returns a seeded `XorShift128` generator from the given 8-bit seed,
    /// unchecked.
    ///
    /// The seeds must not be all `0`, otherwise every result will also be `0`.
    ///
    /// # Panics
    /// Panics in debug if the seeds are all `0`.
    #[inline]
    pub const fn new_unchecked(seeds: [u32; 4]) -> Self {
        debug_assert![(seeds[0] | seeds[1] | seeds[2] | seeds[3]) != 0, "Seeds must be non-zero"];
        Self(seeds)
    }

    /// Returns the current random `u64`.
    #[inline]
    #[must_use]
    pub const fn current_u64(&self) -> u64 {
        ((self.0[0] as u64) << 32) | (self.0[1] as u64)
    }

    /// Returns the next random `u64`.
    #[inline]
    #[must_use]
    pub fn next_u64(&mut self) -> u64 {
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
    #[inline]
    pub const fn next_new(&self) -> Self {
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
        let s = self.next_new();
        let v = s.current_u64();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl XorShift128 {
    /// Returns a seeded `XorShift128` generator from the given 128-bit seed.
    ///
    /// The seeds will be split in little endian order.
    #[inline]
    pub const fn new1_u128(seed: u128) -> Option<Self> {
        Self::new(Cast(seed).into_u32_le())
    }

    /// Returns a seeded `XorShift128` generator from the given 2 × 64-bit seeds.
    ///
    /// The seeds will be split in little endian order.
    #[inline]
    pub const fn new2_u64(seeds: [u64; 2]) -> Option<Self> {
        let [x, y] = Cast(seeds[0]).into_u32_le();
        let [z, a] = Cast(seeds[1]).into_u32_le();
        Self::new([x, y, z, a])
    }

    /// Returns a seeded `XorShift128` generator from the given 4 × 32-bit seeds.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    pub const fn new4_u32(seeds: [u32; 4]) -> Option<Self> {
        Self::new(seeds)
    }

    /// Returns a seeded `XorShift128` generator from the given 8 × 16-bit seeds.
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

    /// Returns a seeded `XorShift128` generator from the given 16 × 8-bit seeds.
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

/// The `XorShift128+` pseudo-random number generator.
///
/// It has a 128-bit state and generates 64-bit numbers.
///
/// It is generally considered to have better statistical properties than
/// [`XorShift128`].
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift128p([u64; 2]);

impl Default for XorShift128p {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl ConstDefault for XorShift128p {
    const DEFAULT: Self = Self::new_unchecked(Self::DEFAULT_SEED);
}

// private associated items
impl XorShift128p {
    const DEFAULT_SEED: [u64; 2] = [0xDEFA_0017_DEFA_0017; 2];

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

impl XorShift128p {
    /// Returns a seeded `XorShift128+` generator from the given 2 × 64-bit seeds.
    ///
    /// Returns `None` if all given seeds are `0`.
    #[inline]
    #[must_use]
    pub const fn new(seeds: [u64; 2]) -> Option<Self> {
        if (seeds[0] | seeds[1]) == 0 {
            Self::cold_path_result()
        } else {
            Some(Self(seeds))
        }
    }

    /// Returns a seeded `XorShift128p` generator from the given 8-bit seed,
    /// unchecked.
    ///
    /// The seeds must not be all `0`, otherwise every result will also be `0`.
    ///
    /// # Panics
    /// Panics in debug if the seeds are all `0`.
    #[inline]
    pub const fn new_unchecked(seeds: [u64; 2]) -> Self {
        debug_assert![(seeds[0] | seeds[1]) != 0, "Seeds must be non-zero"];
        Self(seeds)
    }

    /// Returns the current random `u64`.
    #[inline]
    #[must_use]
    pub const fn current_u64(&self) -> u64 {
        self.0[0].wrapping_add(self.0[1])
    }

    /// Returns the next random `u64`.
    #[inline]
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
    #[inline]
    pub const fn next_new(&self) -> Self {
        let mut x = self.0;
        let [s0, mut s1] = [x[0], x[1]];

        s1 ^= s0;
        x[0] = s0.rotate_left(55) ^ s1 ^ (s1 << 14); // a, b
        x[1] = s1.rotate_left(36); // c

        Self(x)
    }

    /// Returns both the next random state and the `u64` value.
    pub const fn own_next_u64(self) -> Own<Self, u64> {
        let s = self.next_new();
        let v = s.current_u64();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl XorShift128p {
    /// Returns a seeded `XorShift128+` generator from the given 128-bit seed.
    ///
    /// The seeds will be split in little endian order.
    #[inline]
    pub const fn new1_u128(seed: u128) -> Option<Self> {
        Self::new(Cast(seed).into_u64_le())
    }

    /// Returns a seeded `XorShift128+` generator from the given 2 × 64-bit seeds.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    pub const fn new2_u64(seeds: [u64; 2]) -> Option<Self> {
        Self::new(seeds)
    }

    /// Returns a seeded `XorShift128+` generator from the given 4 × 32-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new4_u32(seeds: [u32; 4]) -> Option<Self> {
        Self::new([
            Cast::<u64>::from_u32_le([seeds[0], seeds[1]]),
            Cast::<u64>::from_u32_le([seeds[2], seeds[3]]),
        ])
    }

    /// Returns a seeded `XorShift128+` generator from the given 8 × 16-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new8_u16(seeds: [u16; 8]) -> Option<Self> {
        Self::new([
            Cast::<u64>::from_u16_le([seeds[0], seeds[1], seeds[2], seeds[3]]),
            Cast::<u64>::from_u16_le([seeds[4], seeds[5], seeds[6], seeds[7]]),
        ])
    }

    /// Returns a seeded `XorShift128` generator from the given 4 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new16_u8(seeds: [u8; 16]) -> Option<Self> {
        let s = seeds;
        Self::new([
            Cast::<u64>::from_u8_le([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]),
            Cast::<u64>::from_u8_le([s[8], s[9], s[10], s[11], s[12], s[13], s[14], s[15]]),
        ])
    }
}

#[cfg(feature = "rand_core")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rand_core")))]
mod impl_rand {
    use super::{XorShift128, XorShift128p};
    use crate::_dep::rand_core::{Error, RngCore, SeedableRng};

    impl RngCore for XorShift128 {
        /// Returns the next random `u32`,
        /// from the first 32-bits of `next_u64`.
        #[inline]
        fn next_u32(&mut self) -> u32 {
            (self.next_u64() & 0xFFFF_FFFF) as u32
        }

        /// Returns the next random `u64`.
        #[inline]
        fn next_u64(&mut self) -> u64 {
            self.next_u64()
        }

        #[inline]
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

        #[inline]
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl SeedableRng for XorShift128 {
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

    impl RngCore for XorShift128p {
        /// Returns the next random `u32`,
        /// from the first 32-bits of `next_u64`.
        #[inline]
        fn next_u32(&mut self) -> u32 {
            (self.next_u64() & 0xFFFF_FFFF) as u32
        }

        /// Returns the next random `u64`.
        #[inline]
        fn next_u64(&mut self) -> u64 {
            self.next_u64()
        }

        #[inline]
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

        #[inline]
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl SeedableRng for XorShift128p {
        type Seed = [u8; 16];

        /// When seeded with zero this implementation uses the default seed
        /// value as the cold path.
        #[inline]
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
