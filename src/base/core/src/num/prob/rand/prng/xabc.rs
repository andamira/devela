// devela_base_core::num::prob::rand::xabc
//
//!
//

use crate::{ConstInitCore, Own, Rand};

#[doc = crate::_tags!(rand)]
/// X ABC <abbr title="Pseudo-Random Number Generator">PRNG</abbr> for 8-bit devices.
#[doc = crate::_doc_location!("num/prob/rand")]
///
/// It has a 32-bit state and generates 8-bit numbers.
///
/// This is a small PRNG, experimentally verified to have at least a 50 million
/// byte period by generating 50 million bytes and observing that there were no
/// overapping sequences and repeats.
///
/// This generator passes serial correlation, entropy, Monte Carlo Pi value,
/// arithmetic mean, and many other statistical tests. This generator may have a
/// period of up to 2^32, but this has not been verified.
///
/// By XORing 3 bytes into the a, b, and c registers, you can add in entropy
/// from an external source easily.
///
/// This generator is free to use, but is not suitable for cryptography due to
/// its short period (by cryptographic standards) and simple construction.
/// No attempt was made to make this generator suitable for cryptographic use.
///
/// Due to the use of a constant counter, the generator should be resistant to
/// latching up. A significant performance gain is had in that the x variable is
/// only ever incremented.
///
/// Only 4 bytes of ram are needed for the internal state, and generating a byte
/// requires 3 XORs, 2 ADDs, one bit shift right, and one increment. Difficult
/// or slow operations like multiply, etc were avoided for maximum speed on
/// ultra low power devices.
///
/// It has a period of 487,780,609 from a zeroed state.
///
#[doc = crate::_doc!(vendor: "Xabc")]
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Xabc {
    a: u8,
    b: u8,
    c: u8,
    x: u8,
}

/// Creates a new PRNG initialized with the default fixed seed.
impl Default for Xabc {
    fn default() -> Self {
        Self::INIT
    }
}
/// Creates a new PRNG initialized with the default fixed seed.
impl ConstInitCore for Xabc {
    const INIT: Self = Self::new(Self::DEFAULT_SEED);
}

// private associated items
impl Xabc {
    #[doc(hidden)]
    pub const DEFAULT_SEED: [u8; 3] = [0xDE, 0xFA, 0x17];
}

impl Xabc {
    /// Returns a seeded `Xabc` generator from the given 3 × 8-bit seeds.
    pub const fn new(seeds: [u8; 3]) -> Self {
        let a = seeds[0];
        let b = seeds[1];
        let c = seeds[2];
        let x = 1;
        let a = a ^ c ^ x;
        let b = b.wrapping_add(a);
        let c = c.wrapping_add(b >> 1) ^ a;
        Self { a, b, c, x }
    }

    /// Reseeds the generator from the given 3 × 8-bit seeds.
    pub const fn reseed(&mut self, seeds: [u8; 3]) {
        // XOR new entropy into key state
        self.a ^= seeds[0];
        self.b ^= seeds[1];
        self.c ^= seeds[2];

        self.x += 1;
        self.a = self.a ^ self.c ^ self.x;
        self.b = self.b.wrapping_add(self.a);
        self.c = self.c.wrapping_add(self.b >> 1) ^ self.a;
    }

    #[must_use]
    /// Returns the PRNG's inner state as a raw snapshot.
    pub const fn inner_state(self) -> [u8; 4] {
        [self.a, self.b, self.c, self.x]
    }
    /// Restores the PRNG from the given state.
    pub const fn from_state(state: [u8; 4]) -> Self {
        Self { a: state[0], b: state[1], c: state[2], x: state[3] }
    }

    /// Returns the current random `u8`.
    #[must_use]
    pub const fn current_u8(&self) -> u8 {
        self.c
    }
    /// Advances the state and returns the next random `u8`.
    #[must_use]
    pub const fn next_u8(&mut self) -> u8 {
        // x is incremented every round and is not affected by any other variable
        self.x = self.x.wrapping_add(1);
        // note the mix of addition and XOR
        self.a = self.a ^ self.c ^ self.x;
        // And the use of very few instructions
        self.b = self.b.wrapping_add(self.a);
        // the right shift is to ensure that high-order bits from b can affect
        // low order bits of other variables
        self.c = self.c.wrapping_add(self.b >> 1) ^ self.a;
        self.c
    }

    /// Returns a copy of the next new random state.
    pub const fn peek_next_state(&self) -> Self {
        let [mut a, mut b, mut c, mut x] = [self.a, self.b, self.c, self.x];
        x += 1;
        a = a ^ c ^ x;
        b = b.wrapping_add(a);
        c = c.wrapping_add(b >> 1) ^ a;
        Self { a, b, c, x }
    }

    /// Returns both the next random state and the `u8` value.
    pub const fn own_next_u8(self) -> Own<Self, u8> {
        let s = self.peek_next_state();
        let v = s.current_u8();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl Xabc {
    /// Returns a seeded `Xabc` generator from the given 3 × 8-bit seeds.
    ///
    /// This is an alias of [`new`][Self#method.new].
    pub const fn new3_u8(seeds: [u8; 3]) -> Self {
        Self::new(seeds)
    }
}

impl Rand for Xabc {
    const RAND_OUTPUT_BITS: u32 = 8;
    const RAND_STATE_BITS: u32 = 32;
    /// Returns the next 8 × random `u8` combined as a single `u64`.
    fn rand_next_u64(&mut self) -> u64 {
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
    /// Returns the next 4 × random `u8` combined as a single `u32`.
    fn rand_next_u32(&mut self) -> u32 {
        u32::from_le_bytes([self.next_u8(), self.next_u8(), self.next_u8(), self.next_u8()])
    }
    fn rand_fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest {
            *byte = self.next_u8();
        }
    }
}

#[cfg(feature = "dep_rand_core")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_rand_core")))]
mod impl_rand {
    use super::{Rand, Xabc};
    use crate::_dep::rand_core::{RngCore, SeedableRng};

    impl RngCore for Xabc {
        /// Returns the next 4 × random `u8` combined as a single `u32`.
        fn next_u32(&mut self) -> u32 {
            self.rand_next_u32()
        }
        /// Returns the next 8 × random `u8` combined as a single `u64`.
        fn next_u64(&mut self) -> u64 {
            self.rand_next_u64()
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            self.rand_fill_bytes(dest)
        }
    }
    impl SeedableRng for Xabc {
        type Seed = [u8; 3];
        fn from_seed(seed: Self::Seed) -> Self {
            Self::new(seed)
        }
    }
}
