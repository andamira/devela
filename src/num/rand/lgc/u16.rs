// devela::num::rand::lgc::u16
//
//! 16-bit Linear Congruential Generator
//

use crate::{Cast, ConstDefault, Own};

/// A 16-bit linear congruential generator (LCG) pseudo-random number generator.
///
/// Based on original code from Ken Musgrave, 1985, in Graphics Gems II.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lgc16(u16);

impl Default for Lgc16 {
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl ConstDefault for Lgc16 {
    const DEFAULT: Self = Self::new(Self::DEFAULT_SEED);
}

// Constant defaults for the Lgc16
impl Lgc16 {
    const DEFAULT_SEED: u16 = 0xDEFA;

    /// Multiplier.
    const MUL: u16 = 25173;
    /// Increment.
    const INC: u16 = 13849;
    /// Modulus.
    const MOD: u16 = 65535;
}

impl Lgc16 {
    /// Creates a new `Lgc16` instance with the given seed.
    #[must_use]
    pub const fn new(seed: u16) -> Self {
        Self(seed)
    }

    /// Reseeds the generator with a new seed.
    pub fn reseed(&mut self, seed: u16) {
        self.0 = seed;
    }

    /// Returns the current seed value.
    #[must_use]
    pub const fn current_u16(&self) -> u16 {
        self.0
    }

    /// Advances to the next random `u16` value.
    #[must_use]
    pub fn next_u16(&mut self) -> u16 {
        self.0 = (Self::MUL.wrapping_mul(self.0).wrapping_add(Self::INC)) & Self::MOD;
        self.0
    }

    /// Returns a copy of the next state of the generator.
    #[must_use]
    pub const fn next_state(&self) -> Self {
        let x = (Self::MUL.wrapping_mul(self.0).wrapping_add(Self::INC)) & Self::MOD;
        Self(x)
    }

    /// Returns both the next state and the `u16` value.
    pub const fn own_next_u16(self) -> Own<Self, u16> {
        let s = self.next_state();
        let v = s.current_u16();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl Lgc16 {
    /// Returns a seeded `Lgc16` generator from the given 16-bit seed.
    ///
    /// This is an alias of [`new`][Self#method.new].
    pub const fn new1_u16(seed: u16) -> Self {
        Self::new(seed)
    }

    /// Returns a seeded `Lgc16` generator from the given 2 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[must_use]
    pub const fn new2_u8(seeds: [u8; 2]) -> Self {
        Self::new(Cast::<u16>::from_u8_le(seeds))
    }
}

#[cfg(feature = "dep_rand_core")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_rand_core")))]
mod impl_rand {
    use super::{Cast, Lgc16};
    use crate::_dep::rand_core::{Error, RngCore, SeedableRng};

    impl RngCore for Lgc16 {
        /// Returns the next 2 × random `u16` combined as a single `u32`.
        fn next_u32(&mut self) -> u32 {
            Cast::<u32>::from_u16_le([self.next_u16(), self.next_u16()])
        }

        /// Returns the next 4 × random `u16` combined as a single `u64`.
        fn next_u64(&mut self) -> u64 {
            Cast::<u64>::from_u16_le([
                self.next_u16(),
                self.next_u16(),
                self.next_u16(),
                self.next_u16(),
            ])
        }

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

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl SeedableRng for Lgc16 {
        type Seed = [u8; 2];

        /// When seeded with zero this implementation uses the default seed
        /// value as the cold path.
        fn from_seed(seed: Self::Seed) -> Self {
            Self::new(u16::from_le_bytes(seed))
        }
    }
}
