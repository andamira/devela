// devela::num::rand::xorshift::u16
//
//! 16-bit version of XorShift.
//

use crate::{ConstInit, Own, xorshift_basis};

#[doc = crate::_TAG_RAND!()]
/// The `XorShift16` <abbr title="Pseudo-Random Number Generator">PRNG</abbr>.
///
/// It has a 16-bit state and generates 16-bit numbers.
///
/// This is [John Metcalf's 16-bit] (7, 9, 8) version of George Marsaglia's
/// original [`XorShift32`][super::XorShift32].
///
/// [John Metcalf's 16-bit]: https://github.com/impomatic/xorshift798
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift16<
    const BASIS: usize = 0,
    const A: usize = 7,
    const B: usize = 9,
    const C: usize = 8,
>(u16);

/// Creates a new PRNG initialized with the default fixed seed.
impl Default for XorShift16 {
    fn default() -> Self {
        Self::INIT
    }
}
/// Creates a new PRNG initialized with the default fixed seed.
impl ConstInit for XorShift16 {
    const INIT: Self = Self::new_unchecked(Self::DEFAULT_SEED);
}

// private associated items
impl<const BASIS: usize, const A: usize, const B: usize, const C: usize>
    XorShift16<BASIS, A, B, C>
{
    const DEFAULT_SEED: u16 = 0xDEFA;

    #[cold] #[allow(dead_code)] #[rustfmt::skip]
    const fn cold_path_default() -> Self { Self::new_unchecked(Self::DEFAULT_SEED) }
}

impl<const BASIS: usize, const A: usize, const B: usize, const C: usize>
    XorShift16<BASIS, A, B, C>
{
    /// Returns a seeded `XorShift16` generator from the given 16-bit seed.
    ///
    /// If the seed is `0`, the default seed is used instead.
    pub const fn new(seed: u16) -> Self {
        if seed == 0 { Self::cold_path_default() } else { Self(seed) }
    }

    /// Returns a seeded `XorShift16` generator from the given 8-bit seed, unchecked.
    ///
    /// The seed must not be `0`, otherwise every result will also be `0`.
    pub const fn new_unchecked(seed: u16) -> Self {
        debug_assert![seed != 0, "Seed must be non-zero"];
        Self(seed)
    }

    #[must_use]
    /// Returns the PRNG's inner state as a raw snapshot.
    pub const fn inner_state(self) -> u16 {
        self.0
    }
    /// Restores the PRNG from the given state.
    pub const fn from_state(state: u16) -> Self {
        Self(state)
    }

    /// Returns the current random `u16`.
    #[must_use]
    pub const fn current_u16(&self) -> u16 {
        self.0
    }

    /// Returns the next random `u16`.
    ///
    #[must_use]
    pub fn next_u16(&mut self) -> u16 {
        let mut x = self.0;
        xorshift_basis!(x, BASIS, (A, B, C));
        self.0 = x;
        x
    }

    /// Returns a copy of the next new random state.
    pub const fn peek_next_state(&self) -> Self {
        let mut x = self.0;
        xorshift_basis!(x, BASIS, (A, B, C));
        Self(x)
    }

    /// Returns both the next random state and the `u16` value.
    pub const fn own_next_u16(self) -> Own<Self, u16> {
        let s = self.peek_next_state();
        let v = s.current_u16();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl<const BASIS: usize, const A: usize, const B: usize, const C: usize>
    XorShift16<BASIS, A, B, C>
{
    /// Returns a seeded `XorShift16` generator from the given 16-bit seed.
    ///
    /// This is an alias of [`new`][Self#method.new].
    pub const fn new1_u16(seed: u16) -> Self {
        Self::new(seed)
    }

    /// Returns a seeded `XorShift16` generator from the given 2 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    pub const fn new2_u8(seeds: [u8; 2]) -> Self {
        Self::new(u16::from_le_bytes(seeds))
    }
}

#[cfg(feature = "dep_rand_core")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_rand_core")))]
mod impl_rand {
    use crate::_dep::rand_core::{RngCore, SeedableRng};
    use crate::{Cast, XorShift16};

    impl<const BASIS: usize, const A: usize, const B: usize, const C: usize> RngCore
        for XorShift16<BASIS, A, B, C>
    {
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
    }

    impl<const BASIS: usize, const A: usize, const B: usize, const C: usize> SeedableRng
        for XorShift16<BASIS, A, B, C>
    {
        type Seed = [u8; 2];

        /// When seeded with zero this implementation uses the default seed
        /// value as the cold path.
        fn from_seed(seed: Self::Seed) -> Self {
            if seed == [0; 2] {
                Self::cold_path_default()
            } else {
                Self::new_unchecked(u16::from_le_bytes(seed))
            }
        }
    }
}

/// 4 × good triplets for 16-bit xorshift. (243 Bytes)
///
/// There are 60 shift triplets with the maximum period 2^16-1. 4 triplets pass
/// a series of lightweight randomness tests including randomly plotting various
/// n × n matrices using the high bits, low bits, reversed bits, etc. These are:
#[doc(hidden)]
#[rustfmt::skip]
#[allow(dead_code)]
pub const XOROSHIFT_16_TRIPLETS: [(u8, u8, u8); 4] = [
    (6, 7, 13), (7, 9, 8), (7, 9, 13), (9, 7, 13)
];
