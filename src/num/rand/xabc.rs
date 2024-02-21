// devela::num::rand::xabc
//
//!
//

/// X ABC Algorithm Random Number Generator for 8-bit Devices.
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
/// # License
/// This algorithm was originally openly published in December 2011 by user
/// *EternityForest* in [Electro-Tech-Online.com][link].
///
/// [link]: https://www.electro-tech-online.com/threads/ultra-fast-pseudorandom-number-generator-for-8-bit.124249/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Xabc {
    a: u8,
    b: u8,
    c: u8,
    x: u8,
}

impl Default for Xabc {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

// private associated items
impl Xabc {
    const DEFAULT_SEED: [u8; 3] = [0xDE, 0xFA, 0x17];
}

impl Xabc {
    /// Returns a seeded `Xabc` generator from the given 3 × 8-bit seeds.
    #[inline]
    #[must_use]
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
    #[inline]
    pub fn reseed(&mut self, seeds: [u8; 3]) {
        // XOR new entropy into key state
        self.a ^= seeds[0];
        self.b ^= seeds[1];
        self.c ^= seeds[2];

        self.x += 1;
        self.a = self.a ^ self.c ^ self.x;
        self.b = self.b.wrapping_add(self.a);
        self.c = self.c.wrapping_add(self.b >> 1) ^ self.a;
    }

    /// Returns the current random `u8`.
    #[inline(always)]
    #[must_use]
    pub const fn current_u8(&self) -> u8 {
        self.c
    }

    /// Returns the next random `u8`.
    #[inline]
    #[must_use]
    pub fn next_u8(&mut self) -> u8 {
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
    #[inline]
    #[must_use]
    pub const fn next_new(&self) -> Self {
        let [mut a, mut b, mut c, mut x] = [self.a, self.b, self.c, self.x];
        x += 1;
        a = a ^ c ^ x;
        b = b.wrapping_add(a);
        c = c.wrapping_add(b >> 1) ^ a;
        Self { a, b, c, x }
    }
}

/// # Extra constructors
impl Xabc {
    /// Returns a seeded `Xabc` generator from the given 3 × 8-bit seeds.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    pub const fn new3_u8(seeds: [u8; 3]) -> Self {
        Self::new(seeds)
    }
}

#[cfg(feature = "rand_core")]
// #[cfg(any(feature = "rand_core", all(feature = "dep", feature = "num")))] // MAYBE
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rand_core")))]
mod impl_rand {
    use super::Xabc;
    use crate::_deps::rand_core::{Error, RngCore, SeedableRng};

    impl RngCore for Xabc {
        /// Returns the next 4 × random `u8` combined as a single `u32`.
        fn next_u32(&mut self) -> u32 {
            u32::from_le_bytes([
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
                self.next_u8(),
            ])
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

    impl SeedableRng for Xabc {
        type Seed = [u8; 3];

        fn from_seed(seed: Self::Seed) -> Self {
            Self::new(seed)
        }
    }
}
