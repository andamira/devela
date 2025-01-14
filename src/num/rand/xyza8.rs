// devela::num::rand::xyza8
//
//!

// Original license file:
// https://github.com/edrosten/8bit_rng/blob/master/LICENSE
/*
Copyright Edward Rosten 2008--2013.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
1. Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND OTHER CONTRIBUTORS ``AS IS''
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR OTHER CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
*/

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
///
/// # License
/// This algorithm was ported from [8bit_rng](https://github.com/edrosten/8bit_rng).
/// Copyright (c) 2008-2013 Edward Rosten.
/// Licensed under the [BSD 2-Clause "Simplified" License][license]
///
/// [license]: https://github.com/edrosten/8bit_rng/blob/master/LICENSE
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Xyza8a {
    x: u8,
    y: u8,
    z: u8,
    a: u8,
}

impl Default for Xyza8a {
    fn default() -> Self {
        Self::DEFAULT
    }
}
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

    /// Returns the current random `u8`.
    #[must_use]
    pub const fn current_u8(&self) -> u8 {
        self.a
    }

    /// Updates the state and returns the next random `u8`.
    pub fn next_u8(&mut self) -> u8 {
        let t = self.x ^ (self.x << 4);
        self.x = self.y;
        self.y = self.z;
        self.z = self.a;
        self.a = self.z ^ t ^ (self.z >> 1) ^ (t << 1);
        self.a
    }

    /// Returns a copy of the next new random state.
    #[must_use]
    pub const fn next_state(&self) -> Self {
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
        let s = self.next_state();
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
    pub const fn next_state(&self) -> Self {
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
        let s = self.next_state();
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
    use crate::_dep::rand_core::{Error, RngCore, SeedableRng};

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

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
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

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl SeedableRng for Xyza8b {
        type Seed = [u8; 4];

        fn from_seed(seeds: Self::Seed) -> Self {
            Self::new(seeds)
        }
    }
}
