// devela::num::rand::xyza8
//
//!

use crate::{ConstDefault, Own};

/// A simple 8-bit <abbr title="Pseudo-Random Number Generator">PRNG</abbr>
/// with 32-bit of state, based on the *XorShift* algorithm.
///
/// It has a 0.8% chance of falling into a poor quality short chain,
/// a some degree of care is required to seed it. However, the quality of the
/// random numbers is excellent for such a small state (32 bits), and it passes
/// almost all of the die hard tests.
///
/// Its longest cycle is 4_261_412_736.
// (== u32::MAX + u16::MAX * 512 + 639).
#[doc = crate::doc_!(vendor: "8bit_rng")]
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Xyza8a {
    x: u8,
    y: u8,
    z: u8,
    a: u8,
}

/// Creates a new PRNG initialized with the default fixed seed.
impl Default for Xyza8a {
    fn default() -> Self {
        Self::DEFAULT
    }
}
/// Creates a new PRNG initialized with the default fixed seed.
impl ConstDefault for Xyza8a {
    const DEFAULT: Self = Self::new(Self::DEFAULT_SEED);
}

// private associated items
impl Xyza8a {
    const DEFAULT_SEED: [u8; 4] = [0xDE, 0xFA, 0x00, 0x17];
}

impl Xyza8a {
    /// Returns a seeded `Xyza8a` generator from the given 4 × 8-bit seeds.
    pub const fn new(seeds: [u8; 4]) -> Self {
        Self { x: seeds[0], y: seeds[1], z: seeds[2], a: seeds[3] }
    }

    #[must_use]
    /// Returns the PRNG's inner state as a raw snapshot.
    pub const fn inner_state(self) -> [u8; 4] {
        [self.x, self.y, self.z, self.a]
    }
    /// Restores the PRNG from the given state.
    pub const fn from_state(state: [u8; 4]) -> Self {
        Self { x: state[0], y: state[1], z: state[2], a: state[3] }
    }

    #[must_use]
    /// Returns the current random `u8`.
    pub const fn current_u8(&self) -> u8 {
        self.a
    }
    /// Advances the state and returns the next random `u8`.
    pub fn next_u8(&mut self) -> u8 {
        let t = self.x ^ (self.x << 4);
        self.x = self.y;
        self.y = self.z;
        self.z = self.a;
        self.a = self.z ^ t ^ (self.z >> 1) ^ (t << 1);
        self.a
    }

    /// Returns a copy of the next new random state.
    pub const fn peek_next_state(&self) -> Self {
        let mut new = *self;

        let t = new.x ^ (new.x << 4);
        new.x = new.y;
        new.y = new.z;
        new.z = new.a;
        new.a = new.z ^ t ^ (new.z >> 1) ^ (t << 1);
        new
    }

    /// Returns both the next random state and the `u8` value.
    pub const fn own_next_u8(self) -> Own<Self, u8> {
        let s = self.peek_next_state();
        let v = s.current_u8();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl Xyza8a {
    /// Returns a seeded `Xyza8a` generator from the given 32-bit seed.
    ///
    /// The seeds will be split in little endian order.
    pub const fn new1_u32(seed: u32) -> Self {
        Self::new(seed.to_le_bytes())
    }

    /// Returns a seeded `Xyza8a` generator from the given 2 × 16-bit seeds.
    ///
    /// The seeds will be split in little endian order.
    pub const fn new2_u16(seeds: [u16; 2]) -> Self {
        let [x, y] = seeds[0].to_le_bytes();
        let [z, a] = seeds[1].to_le_bytes();
        Self::new([x, y, z, a])
    }

    /// Returns a seeded `Xyza8b` generator from the given 4 × 8-bit seeds.
    /// This is an alias of [`new`][Self#method.new].
    pub const fn new4_u8(seeds: [u8; 4]) -> Self {
        Self::new(seeds)
    }
}

// -----------------------------------------------------------------------------

/// A simple 8-bit <abbr title="Pseudo-Random Number Generator">PRNG</abbr>
/// with 32-bit of state, based on the *XorShift* algorithm.
///
/// It has an almost optimal cycle so no real care is required
/// for seeding except avoiding all zeros, but it fails many of the die hard
/// random number tests.
///
/// Its longest cycle is 4,294,967,294.
#[doc = crate::doc_!(vendor: "8bit_rng")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Xyza8b {
    x: u8,
    y: u8,
    z: u8,
    a: u8,
}

impl Default for Xyza8b {
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl ConstDefault for Xyza8b {
    const DEFAULT: Self = Self::new(Self::DEFAULT_SEED);
}

// private associated items
impl Xyza8b {
    const DEFAULT_SEED: [u8; 4] = [0xDE, 0xFA, 0x00, 0x17];
}

impl Xyza8b {
    /// Returns a seeded `Xyza8b` generator from the given 4 × 8-bit seeds.
    /// This is the fastest constructor.
    pub const fn new(seeds: [u8; 4]) -> Self {
        Self { x: seeds[0], y: seeds[1], z: seeds[2], a: seeds[3] }
    }

    #[must_use]
    /// Returns the PRNG's inner state as a raw snapshot.
    pub const fn inner_state(self) -> [u8; 4] {
        [self.x, self.y, self.z, self.a]
    }
    /// Restores the PRNG from the given state.
    pub const fn from_state(state: [u8; 4]) -> Self {
        Self { x: state[0], y: state[1], z: state[2], a: state[3] }
    }

    /// Returns the current random `u8`.
    pub const fn current_u8(&self) -> u8 {
        self.a
    }

    /// Returns the next random `u8`.
    pub fn next_u8(&mut self) -> u8 {
        let t = self.x ^ (self.x >> 1);
        self.x = self.y;
        self.y = self.z;
        self.z = self.a;
        self.a = self.z ^ t ^ (self.z >> 3) ^ (t << 1);
        self.a
    }

    /// Returns a copy of the next new random state.
    pub const fn peek_next_state(&self) -> Self {
        let mut new = *self;

        let t = new.x ^ (new.x >> 1);
        new.x = new.y;
        new.y = new.z;
        new.z = new.a;
        new.a = new.z ^ t ^ (new.z >> 3) ^ (t << 1);
        new
    }

    /// Returns both the next random state and the `u8` value.
    pub const fn own_next_u8(self) -> Own<Self, u8> {
        let s = self.peek_next_state();
        let v = s.current_u8();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl Xyza8b {
    /// Returns a seeded `Xyza8a` generator from the given 32-bit seed.
    ///
    /// The seeds will be split in little endian order.
    pub const fn new1_u32(seed: u32) -> Self {
        Self::new(seed.to_le_bytes())
    }

    /// Returns a seeded `Xyza8a` generator from the given 2 × 16-bit seeds.
    ///
    /// The seeds will be split in little endian order.
    pub const fn new2_u16(seeds: [u16; 2]) -> Self {
        let [x, y] = seeds[0].to_le_bytes();
        let [z, b] = seeds[1].to_le_bytes();
        Self::new([x, y, z, b])
    }

    /// Returns a seeded `Xyza8b` generator from the given 4 × 8-bit seeds.
    /// This is an alias of [`new`][Self#method.new].
    pub const fn new4_u8(seeds: [u8; 4]) -> Self {
        Self::new(seeds)
    }
}

#[cfg(feature = "dep_rand_core")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_rand_core")))]
mod impl_rand {
    use super::{Xyza8a, Xyza8b};
    use crate::_dep::rand_core::{RngCore, SeedableRng};

    impl RngCore for Xyza8a {
        /// Returns the next 4 × random `u8` combined as a single `u32`.
        fn next_u32(&mut self) -> u32 {
            u32::from_le_bytes([self.next_u8(), self.next_u8(), self.next_u8(), self.next_u8()])
        }
        /// Returns the next 8 × random `u8` combined as a single `u64`.
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
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest {
                *byte = self.next_u8();
            }
        }
    }
    impl SeedableRng for Xyza8a {
        type Seed = [u8; 4];
        fn from_seed(seeds: Self::Seed) -> Self {
            Self::new(seeds)
        }
    }

    impl RngCore for Xyza8b {
        /// Returns the next 4 × random `u8` combined as a single `u32`.
        fn next_u32(&mut self) -> u32 {
            u32::from_le_bytes([self.next_u8(), self.next_u8(), self.next_u8(), self.next_u8()])
        }
        /// Returns the next 8 × random `u8` combined as a single `u64`.
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
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest {
                *byte = self.next_u8();
            }
        }
    }
    impl SeedableRng for Xyza8b {
        type Seed = [u8; 4];
        fn from_seed(seeds: Self::Seed) -> Self {
            Self::new(seeds)
        }
    }
}
