// devela::num::rand::xorshift::u32
//
//! 32-bit version of XorShift.
//

use crate::{ConstDefault, Own, xorshift_basis};

#[doc = crate::TAG_RAND!()]
/// The `XorShift32` <abbr title="Pseudo-Random Number Generator">PRNG</abbr>.
///
/// It has a 32-bit state and generates 32-bit numbers.
///
/// This is the classic 32-bit *XorShift* algorithm by George Marsaglia.
///
/// The `BASIS` and triplet (`A`, `B`, `C`) values default to the canonical example.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift32<
    const BASIS: usize = 1,
    const A: usize = 5,
    const B: usize = 17,
    const C: usize = 13,
>(u32);

/// Creates a new PRNG initialized with the default fixed seed.
impl Default for XorShift32 {
    fn default() -> Self {
        Self::DEFAULT
    }
}
/// Creates a new PRNG initialized with the default fixed seed.
impl ConstDefault for XorShift32 {
    const DEFAULT: Self = Self::new_unchecked(Self::DEFAULT_SEED);
}

// private associated items
impl<const BASIS: usize, const A: usize, const B: usize, const C: usize>
    XorShift32<BASIS, A, B, C>
{
    const DEFAULT_SEED: u32 = 0xDEFA_0017;

    #[cold] #[allow(dead_code)] #[rustfmt::skip]
    const fn cold_path_default() -> Self { Self::new_unchecked(Self::DEFAULT_SEED) }
}

impl<const BASIS: usize, const A: usize, const B: usize, const C: usize>
    XorShift32<BASIS, A, B, C>
{
    /// Returns a seeded `XorShift32` generator from the given 32-bit seed.
    ///
    /// If the seed is `0`, the default seed is used instead.
    pub const fn new(seed: u32) -> Self {
        if seed == 0 { Self::cold_path_default() } else { Self(seed) }
    }

    /// Returns a seeded `XorShift32` generator from the given 8-bit seed, unchecked.
    ///
    /// The seed must not be `0`, otherwise every result will also be `0`.
    pub const fn new_unchecked(seed: u32) -> Self {
        debug_assert![seed != 0, "Seed must be non-zero"];
        Self(seed)
    }

    #[must_use]
    /// Returns the PRNG's inner state as a raw snapshot.
    pub const fn inner_state(self) -> u32 {
        self.0
    }
    /// Restores the PRNG from the given state.
    pub const fn from_state(state: u32) -> Self {
        Self(state)
    }

    /// Returns the current random `u32`.
    #[must_use]
    pub const fn current_u32(&self) -> u32 {
        self.0
    }

    /// Returns the next random `u32`.
    //
    // Algorithm "xor" from p. 4 of Marsaglia, "Xorshift RNGs"
    #[must_use]
    pub fn next_u32(&mut self) -> u32 {
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

    /// Returns both the next random state and the `u32` value.
    pub const fn own_next_u32(self) -> Own<Self, u32> {
        let s = self.peek_next_state();
        let v = s.current_u32();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl<const BASIS: usize, const A: usize, const B: usize, const C: usize>
    XorShift32<BASIS, A, B, C>
{
    /// Returns a seeded `XorShift32` generator from the given 32-bit seed.
    ///
    /// This is an alias of [`new`][Self#method.new].
    pub const fn new1_u32(seed: u32) -> Self {
        Self::new(seed)
    }

    /// Returns a seeded `XorShift32` generator from the given 2 × 16-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[cfg(feature = "join")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "join")))]
    pub const fn new2_u16(seeds: [u16; 2]) -> Self {
        Self::new(crate::Cast::<u32>::from_u16_le(seeds))
    }

    /// Returns a seeded `XorShift32` generator from the given 4 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    pub const fn new4_u8(seeds: [u8; 4]) -> Self {
        Self::new(u32::from_le_bytes(seeds))
    }
}

#[cfg(all(feature = "dep_rand_core", feature = "join"))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "dep_rand_core", feature = "join"))))]
mod impl_rand {
    use crate::_dep::rand_core::{RngCore, SeedableRng};
    use crate::{Cast, XorShift32};

    impl<const BASIS: usize, const A: usize, const B: usize, const C: usize> RngCore
        for XorShift32<BASIS, A, B, C>
    {
        /// Returns the next random `u32`.
        fn next_u32(&mut self) -> u32 {
            self.next_u32()
        }

        /// Returns the next 2 × random `u32` combined as a single `u64`.
        fn next_u64(&mut self) -> u64 {
            Cast::<u64>::from_u32_le([self.next_u32(), self.next_u32()])
        }

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
    }

    impl<const BASIS: usize, const A: usize, const B: usize, const C: usize> SeedableRng
        for XorShift32<BASIS, A, B, C>
    {
        type Seed = [u8; 4];

        /// When seeded with zero this implementation uses the default seed
        /// value as the cold path.
        fn from_seed(seed: Self::Seed) -> Self {
            if seed == [0; 4] {
                Self::cold_path_default()
            } else {
                Self::new_unchecked(u32::from_le_bytes(seed))
            }
        }
    }
}

/// 81 × good triplets for 32-bit xorshift. (243 Bytes)
#[doc(hidden)]
#[rustfmt::skip]
#[allow(dead_code)]
pub const XOROSHIFT_32_TRIPLETS: [(u8, u8, u8); 81] = [
    ( 1, 3,10), ( 1, 5,16), ( 1, 5,19), ( 1, 9,29), ( 1,11, 6), ( 1,11,16),
    ( 1,19, 3), ( 1,21,20), ( 1,27,27), ( 2, 5,15), ( 2, 5,21), ( 2, 7, 7),
    ( 2, 7, 9), ( 2, 7,25), ( 2, 9,15), ( 2,15,17), ( 2,15,25), ( 2,21, 9),
    ( 3, 1,14), ( 3, 3,26), ( 3, 3,28), ( 3, 3,29), ( 3, 5,20), ( 3, 5,22),
    ( 3, 5,25), ( 3, 7,29), ( 3,13, 7), ( 3,23,25), ( 3,25,24), ( 3,27,11),
    ( 4, 3,17), ( 4, 3,27), ( 4, 5,15), ( 5, 3,21), ( 5, 7,22), ( 5, 9, 7),
    ( 5, 9,28), ( 5, 9,31), ( 5,13, 6), ( 5,15,17), ( 5,17,13), ( 5,21,12),
    ( 5,27, 8), ( 5,27,21), ( 5,27,25), ( 5,27,28), ( 6, 1,11), ( 6, 3,17),
    ( 6,17, 9), ( 6,21, 7), ( 6,21,13), ( 7, 1, 9), ( 7, 1,18), ( 7, 1,25),
    ( 7,13,25), ( 7,17,21), ( 7,25,12), ( 7,25,20), ( 8, 7,23), ( 8, 9,23),
    ( 9, 5, 1), ( 9, 5,25), ( 9,11,19), ( 9,21,16), (10, 9,21), (10, 9,25),
    (11, 7,12), (11,7, 16), (11,17,13), (11,21,13), (12, 9,23), (13, 3,17),
    (13, 3,27), (13,5, 19), (13,17,15), (14, 1,15), (14,13,15), (15, 1,29),
    (17,15,20), (17,15,23), (17,15,26)
];
