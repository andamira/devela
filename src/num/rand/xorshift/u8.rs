// devela::num::rand::xorshift::u8
//
//! 8-bit versions of XorShift generators.
//

use crate::{code::ConstDefault, result::Own};

/// The `XorShift8` pseudo-random number generator.
///
/// It has an 8-bit state and generates 8-bit numbers.
///
/// This is a simple 8-bit version (3, 4, 2) of [`XorShift16`][super::XorShift16].
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift8(u8);

impl Default for XorShift8 {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl ConstDefault for XorShift8 {
    const DEFAULT: Self = Self::new_unchecked(Self::DEFAULT_SEED);
}

// private associated items
impl XorShift8 {
    const DEFAULT_SEED: u8 = 0xDE;

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

impl XorShift8 {
    /// Returns a seeded `XorShift8` generator from the given 8-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    #[inline]
    #[must_use]
    pub const fn new(seed: u8) -> Option<Self> {
        if seed == 0 {
            Self::cold_path_result()
        } else {
            Some(Self(seed))
        }
    }

    /// Returns a seeded `XorShift8` generator from the given 8-bit seed, unchecked.
    ///
    /// The seed must not be `0`, otherwise every result will also be `0`.
    #[inline]
    pub const fn new_unchecked(seed: u8) -> Self {
        debug_assert![seed != 0, "Seed must be non-zero"];
        Self(seed)
    }

    /// Returns the current random `u8`.
    #[inline]
    #[must_use]
    pub const fn current_u8(&self) -> u8 {
        self.0
    }

    /// Returns the next random `u8`.
    #[inline]
    #[must_use]
    pub fn next_u8(&mut self) -> u8 {
        let mut x = self.0;
        x ^= x << 3;
        x ^= x >> 4;
        x ^= x << 2;
        self.0 = x;
        x
    }

    /// Returns a copy of the next new random state.
    #[inline]
    pub const fn next_new(&self) -> Self {
        let mut x = self.0;
        x ^= x << 3;
        x ^= x >> 4;
        x ^= x << 2;
        Self(x)
    }

    /// Returns both the next random state and the `u8` value.
    pub const fn own_next_u8(self) -> Own<Self, u8> {
        let s = self.next_new();
        let v = s.current_u8();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl XorShift8 {
    /// Returns a seeded `XorShift8` generator from the given 8-bit seed.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    pub const fn new1_u8(seed: u8) -> Option<Self> {
        Self::new(seed)
    }
}

/// A version of [`XorShift8`] that allows customizing the shift values.
///
/// It has an 8-bit state and generates 8-bit numbers.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift8Custom<const SH1: usize = 3, const SH2: usize = 4, const SH3: usize = 2>(u8);

impl<const SH1: usize, const SH2: usize, const SH3: usize> Default
    for XorShift8Custom<SH1, SH2, SH3>
{
    #[inline]
    fn default() -> Self {
        Self::new_unchecked(Self::DEFAULT_SEED)
    }
}
impl<const SH1: usize, const SH2: usize, const SH3: usize> ConstDefault
    for XorShift8Custom<SH1, SH2, SH3>
{
    const DEFAULT: Self = Self::new_unchecked(Self::DEFAULT_SEED);
}

// private associated items
impl<const SH1: usize, const SH2: usize, const SH3: usize> XorShift8Custom<SH1, SH2, SH3> {
    const DEFAULT_SEED: u8 = 0xDE;

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

impl<const SH1: usize, const SH2: usize, const SH3: usize> XorShift8Custom<SH1, SH2, SH3> {
    /// Returns a seeded `XorShift8Custom` generator from the given 8-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    ///
    /// # Panics
    /// Panics in debug if either `SH1`, `SH2` or `SH3` are < 1 or > 7.
    #[inline]
    pub const fn new(seed: u8) -> Option<Self> {
        debug_assert![SH1 > 0 && SH1 <= 7];
        debug_assert![SH2 > 0 && SH1 <= 7];
        debug_assert![SH3 > 0 && SH1 <= 7];

        if seed == 0 {
            Self::cold_path_result()
        } else {
            Some(Self(seed))
        }
    }

    /// Returns a seeded `XorShift8Custom` generator from the given 8-bit seed,
    /// unchecked.
    ///
    /// The seed must not be `0`, otherwise every result will also be `0`.
    ///
    /// # Panics
    /// Panics in debug if either `SH1`, `SH2` or `SH3` are < 1 or > 7,
    /// or if the seed is `0`.
    #[inline]
    pub const fn new_unchecked(seed: u8) -> Self {
        debug_assert![SH1 > 0 && SH1 <= 7];
        debug_assert![SH2 > 0 && SH1 <= 7];
        debug_assert![SH3 > 0 && SH1 <= 7];
        debug_assert![seed != 0, "Seed must be non-zero"];
        Self(seed)
    }

    /// Returns the current random `u8`.
    #[inline]
    #[must_use]
    pub const fn current_u8(&self) -> u8 {
        self.0
    }

    /// Updates the state and returns the next random `u8`.
    ///
    #[inline]
    pub fn next_u8(&mut self) -> u8 {
        let mut x = self.0;
        x ^= x << SH1;
        x ^= x >> SH2;
        x ^= x << SH3;
        self.0 = x;
        x
    }

    /// Returns a copy of the next new random state.
    #[inline]
    pub const fn next_new(&self) -> Self {
        let mut x = self.0;
        x ^= x << SH1;
        x ^= x >> SH2;
        x ^= x << SH3;
        Self(x)
    }

    /// Returns both the next random state and the `u8` value.
    pub const fn own_next_u8(self) -> Own<Self, u8> {
        let s = self.next_new();
        let v = s.current_u8();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl<const SH1: usize, const SH2: usize, const SH3: usize> XorShift8Custom<SH1, SH2, SH3> {
    /// Returns a seeded `XorShift8Custom` generator from the given 8-bit seed.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    pub const fn new1_u8(seed: u8) -> Option<Self> {
        Self::new(seed)
    }
}

#[cfg(feature = "rand_core")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rand_core")))]
mod impl_rand {
    use super::{XorShift8, XorShift8Custom};
    use crate::_deps::rand_core::{Error, RngCore, SeedableRng};

    impl RngCore for XorShift8 {
        /// Returns the next 4 × random `u8` combined as a single `u32`.
        #[inline]
        fn next_u32(&mut self) -> u32 {
            u32::from_le_bytes([
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
            ])
        }

        /// Returns the next 8 × random `u8` combined as a single `u64`.
        #[inline]
        fn next_u64(&mut self) -> u64 {
            u64::from_le_bytes([
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
            ])
        }

        #[inline]
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest {
                *byte = self.next_u8();
            }
        }

        #[inline]
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl SeedableRng for XorShift8 {
        type Seed = [u8; 1];

        /// When seeded with zero this implementation uses the default seed
        /// value as the cold path.
        #[inline]
        fn from_seed(seed: Self::Seed) -> Self {
            if seed[0] == 0 {
                Self::cold_path_default()
            } else {
                Self::new_unchecked(seed[0])
            }
        }
    }

    impl<const SH1: usize, const SH2: usize, const SH3: usize> RngCore
        for XorShift8Custom<SH1, SH2, SH3>
    {
        /// Returns the next 4 × random `u8` combined as a single `u32`.
        #[inline]
        fn next_u32(&mut self) -> u32 {
            u32::from_le_bytes([
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
            ])
        }

        /// Returns the next 8 × random `u8` combined as a single `u64`.
        #[inline]
        fn next_u64(&mut self) -> u64 {
            u64::from_le_bytes([
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
            ])
        }

        #[inline]
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest {
                *byte = self.next_u8();
            }
        }

        #[inline]
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl<const SH1: usize, const SH2: usize, const SH3: usize> SeedableRng
        for XorShift8Custom<SH1, SH2, SH3>
    {
        type Seed = [u8; 1];

        /// When seeded with zero this implementation uses the default seed
        /// value as the cold path.
        #[inline]
        fn from_seed(seed: Self::Seed) -> Self {
            if seed[0] == 0 {
                Self::cold_path_default()
            } else {
                Self::new_unchecked(seed[0])
            }
        }
    }
}
