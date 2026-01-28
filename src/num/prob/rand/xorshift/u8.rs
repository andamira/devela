// devela::num::prob::rand::xorshift::u8
//
//! 8-bit versions of XorShift.
//

use crate::{ConstInit, Own, xorshift_basis};

#[doc = crate::_tags!(rand)]
/// The `XorShift8` <abbr title="Pseudo-Random Number Generator">PRNG</abbr>.
#[doc = crate::_doc_location!("num/rand")]
///
/// It has an 8-bit state and generates 8-bit numbers.
/// It has poor statistical quality and limited entropy.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift8<const A: usize = 3, const B: usize = 4, const C: usize = 2>(u8);

/// Creates a new PRNG initialized with the default fixed seed.
impl<const A: usize, const B: usize, const C: usize> Default for XorShift8<A, B, C> {
    fn default() -> Self {
        Self::new_unchecked(Self::DEFAULT_SEED)
    }
}
/// Creates a new PRNG initialized with the default fixed seed.
impl<const A: usize, const B: usize, const C: usize> ConstInit for XorShift8<A, B, C> {
    const INIT: Self = Self::new_unchecked(Self::DEFAULT_SEED);
}

// private associated items
impl<const A: usize, const B: usize, const C: usize> XorShift8<A, B, C> {
    const DEFAULT_SEED: u8 = 0xDE;

    #[cold] #[allow(dead_code)] #[rustfmt::skip]
    const fn cold_path_default() -> Self { Self::new_unchecked(Self::DEFAULT_SEED) }
}

impl<const A: usize, const B: usize, const C: usize> XorShift8<A, B, C> {
    /// Returns a seeded `XorShift8` generator from the given 8-bit seed.
    ///
    /// If the seed is `0`, the default seed is used instead.
    ///
    /// # Panics
    /// Panics in debug if either `A`, `B` or `C` are < 1 or > 7.
    pub const fn new(seed: u8) -> Self {
        debug_assert![A > 0 && A <= 7];
        debug_assert![B > 0 && A <= 7];
        debug_assert![C > 0 && A <= 7];
        if seed == 0 { Self::cold_path_default() } else { Self(seed) }
    }

    /// Returns a seeded `XorShift8` generator from the given 8-bit seed,
    /// unchecked.
    ///
    /// The seed must not be `0`, otherwise every result will also be `0`.
    ///
    /// # Panics
    /// Panics in debug if either `A`, `B` or `C` are < 1 or > 7,
    /// or if the seed is `0`.
    pub const fn new_unchecked(seed: u8) -> Self {
        debug_assert![A > 0 && A <= 7];
        debug_assert![B > 0 && A <= 7];
        debug_assert![C > 0 && A <= 7];
        debug_assert![seed != 0, "Seed must be non-zero"];
        Self(seed)
    }

    #[must_use]
    /// Returns the PRNG's inner state as a raw snapshot.
    pub const fn inner_state(self) -> u8 {
        self.0
    }
    /// Restores the PRNG from the given state.
    pub const fn from_state(state: u8) -> Self {
        Self(state)
    }

    /// Returns the current random `u8`.
    #[must_use]
    pub const fn current_u8(&self) -> u8 {
        self.0
    }

    /// Updates the state and returns the next random `u8`.
    ///
    pub const fn next_u8(&mut self) -> u8 {
        let mut x = self.0;
        xorshift_basis!(x, 0, (A, B, C));
        self.0 = x;
        x
    }

    /// Returns a copy of the next new random state.
    pub const fn peek_next_state(&self) -> Self {
        let mut x = self.0;
        xorshift_basis!(x, 0, (A, B, C));
        Self(x)
    }

    /// Returns both the next random state and the `u8` value.
    pub const fn own_next_u8(self) -> Own<Self, u8> {
        let s = self.peek_next_state();
        let v = s.current_u8();
        Own::new(s, v)
    }
}

/// # Extra constructors
impl<const A: usize, const B: usize, const C: usize> XorShift8<A, B, C> {
    /// Returns a seeded `XorShift8` generator from the given 8-bit seed.
    ///
    /// This is an alias of [`new`][Self#method.new].
    pub const fn new1_u8(seed: u8) -> Self {
        Self::new(seed)
    }
}

#[cfg(feature = "dep_rand_core")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_rand_core")))]
mod impl_rand {
    use super::XorShift8;
    use crate::_dep::rand_core::{RngCore, SeedableRng};

    impl<const A: usize, const B: usize, const C: usize> RngCore for XorShift8<A, B, C> {
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

    impl<const A: usize, const B: usize, const C: usize> SeedableRng for XorShift8<A, B, C> {
        type Seed = [u8; 1];

        /// When seeded with zero this implementation uses the default seed
        /// value as the cold path.
        fn from_seed(seed: Self::Seed) -> Self {
            if seed[0] == 0 {
                Self::cold_path_default()
            } else {
                Self::new_unchecked(seed[0])
            }
        }
    }
}
