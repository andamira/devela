// devela_base_core::num::prob::rand::pcg::u32
//
//! Defines [`Pcg32`].
//

use crate::{Cast, ConstInitCore, Own, Rand, Slice, slice};

#[doc = crate::_tags!(rand)]
#[doc = concat!["A 32-bit ", crate::_ABBR_PCG!(), " ", crate::_ABBR_PRNG!(), "."]]
#[doc = crate::_doc_location!("num/prob/rand")]
/// It has a 128-bit internal state and generates 32-bit values.
///
/// This is the canonical PCG-XSH-RR variant, combining a linear
/// congruential generator with a high-quality output permutation.
///
/// Compared to classic xorshift generators, PCG provides stronger
/// statistical properties while remaining small and fast.
///
/// This generator is deterministic and non-cryptographic.
///
/// This implementation follows the canonical PCG XSH RR 64→32 variant
/// as described by Melissa O'Neill.
///
/// Reference: https://www.pcg-random.org
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pcg32 {
    state: u64,
    inc: u64, // must always be odd
}
/// Creates a new PRNG initialized with the default fixed seed.
impl Default for Pcg32 {
    fn default() -> Self {
        Self::INIT
    }
}
/// Creates a new PRNG initialized with the default fixed seed.
impl ConstInitCore for Pcg32 {
    const INIT: Self = Self::new(Self::DEFAULT_SEED, Self::DEFAULT_INC);
}

// Private associated item
impl Pcg32 {
    #[doc(hidden)]
    pub const DEFAULT_SEED: u64 = 0xDEFA_0017_DEFA_0017;
    #[doc(hidden)]
    pub const DEFAULT_INC: u64 = super::PCG_INC_64;

    #[inline(always)]
    const fn output_xsh_rr_64_32(state: u64) -> u32 {
        let xorshifted = (((state >> 18) ^ state) >> 27) as u32;
        let rot = (state >> 59) as u32;
        xorshifted.rotate_right(rot)
    }

    #[inline]
    const fn advance_lcg_64(
        state: u64,
        mut delta: u64,
        mut cur_mult: u64,
        mut cur_plus: u64,
    ) -> u64 {
        let mut acc_mult = 1u64;
        let mut acc_plus = 0u64;
        while delta > 0 {
            if (delta & 1) != 0 {
                acc_mult = acc_mult.wrapping_mul(cur_mult);
                acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
            }
            cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
            cur_mult = cur_mult.wrapping_mul(cur_mult);
            delta >>= 1;
        }
        acc_mult.wrapping_mul(state).wrapping_add(acc_plus)
    }
}

impl Pcg32 {
    /// Creates a new PCG32 using the canonical PCG seeding scheme.
    ///
    /// This performs a small warm-up so that closely related seeds produce
    /// well-scrambled initial outputs.
    pub const fn new(seed: u64, stream: u64) -> Self {
        let mut rng = Self::new_fast(0, stream);
        let _ = rng.next_u32();
        rng.state = rng.state.wrapping_add(seed);
        let _ = rng.next_u32();
        rng
    }

    /// Creates a new PCG32 with the given initial state and stream.
    ///
    /// The stream value selects an independent random sequence.
    pub const fn new_fast(state: u64, stream: u64) -> Self {
        Self { state, inc: (stream << 1) | 1 }
    }

    /// Creates a new PCG32 without enforcing invariants.
    ///
    /// # Safety / Invariants
    /// `inc` must be odd.
    pub const fn new_unchecked(state: u64, inc: u64) -> Self {
        debug_assert![!inc.is_multiple_of(2), "`inc` must be odd"];
        Self { state, inc }
    }

    /// Returns the PRNG's inner state as a raw snapshot.
    #[must_use]
    pub const fn inner_state(self) -> (u64, u64) {
        (self.state, self.inc)
    }

    /// Restores the PRNG from the given state.
    #[must_use]
    pub const fn from_state(state: (u64, u64)) -> Self {
        Self { state: state.0, inc: state.1 }
    }

    /// Returns the current `u32` value.
    #[must_use]
    pub const fn current_u32(&self) -> u32 {
        Self::output_xsh_rr_64_32(self.state)
    }

    /// Advances to the next random `u32` value.
    #[must_use]
    pub const fn next_u32(&mut self) -> u32 {
        let old = self.state;
        self.state = old.wrapping_mul(super::PCG_MUL_64).wrapping_add(self.inc | 1);
        Self::output_xsh_rr_64_32(old)
    }

    /// Returns a copy of the next PRNG state.
    #[must_use]
    pub const fn peek_next_state(&self) -> Self {
        let next_state = self.state.wrapping_mul(super::PCG_MUL_64).wrapping_add(self.inc | 1);
        Self { state: next_state, inc: self.inc }
    }
    /// Returns the next random `u32` without advancing the state.
    #[must_use]
    pub const fn peek_next_u32(&self) -> u32 {
        Self::output_xsh_rr_64_32(self.state)
    }

    /// Returns both the next random state and the `u32` value.
    pub const fn own_next_u32(self) -> Own<Self, u32> {
        let value = Self::output_xsh_rr_64_32(self.state);
        let next = self.peek_next_state();
        Own::new(next, value)
    }

    /// Fills the given buffer
    pub const fn fill_bytes(&mut self, buffer: &mut [u8]) {
        let mut i = 0;
        while i < buffer.len() {
            let r = self.next_u32();
            let bytes = r.to_le_bytes();
            let remaining = buffer.len() - i;
            if remaining >= 4 {
                // dest[i..i + 4].copy_from_slice(&bytes);
                Slice::copy(slice!(mut buffer, i, ..i + 4), &bytes);
                i += 4;
            } else {
                // dest[i..].copy_from_slice(&bytes[..remaining]);
                Slice::copy(slice!(mut buffer, i, ..), slice!(&bytes, ..remaining));
                break;
            }
        }
    }

    /// Advances the generator by `delta` steps.
    pub const fn advance(&mut self, delta: u64) {
        self.state = Self::advance_lcg_64(self.state, delta, super::PCG_MUL_64, self.inc | 1);
    }

    /// Returns a copy of the state advanced by `delta` steps.
    pub const fn copy_advance(self, delta: u64) -> Self {
        let mut s = self;
        s.advance(delta);
        s
    }

    /// Returns a uniformly distributed value in `0..bound`.
    ///
    /// Uses rejection sampling to avoid modulo bias.
    #[must_use]
    pub fn next_bounded(&mut self, bound: u32) -> u32 {
        debug_assert!(bound > 0);
        let threshold = bound.wrapping_neg() % bound;
        loop {
            let r = self.next_u32();
            if r >= threshold {
                return r % bound;
            }
        }
    }
}

impl Rand for Pcg32 {
    const RAND_OUTPUT_BITS: u32 = 32;
    const RAND_STATE_BITS: u32 = 128;

    /// Returns the next random `u32`,
    /// from the first 32-bits of `next_u64`.
    fn rand_next_u32(&mut self) -> u32 {
        self.next_u32()
    }
    /// Returns the next random `u64`.
    fn rand_next_u64(&mut self) -> u64 {
        Cast::<u64>::from_u32_le([self.next_u32(), self.next_u32()])
    }
    fn rand_fill_bytes(&mut self, dest: &mut [u8]) {
        self.fill_bytes(dest)
    }
    fn rand_below(&mut self, upper: u64) -> u64 {
        debug_assert!(upper > 0);
        if upper <= u32::MAX as u64 {
            self.next_bounded(upper as u32) as u64
        } else {
            // fallback to generic rejection on u64
            let zone = u64::MAX - (u64::MAX % upper);
            loop {
                let v = self.rand_next_u64();
                if v < zone {
                    return v % upper;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pcg32_deterministic() {
        let mut a = Pcg32::new(42, 54);
        let mut b = Pcg32::new(42, 54);
        for _ in 0..1000 {
            assert_eq!(a.next_u32(), b.next_u32());
        }
    }
    #[test]
    fn pcg32_independent_streams() {
        let mut a = Pcg32::new(42, 1);
        let mut b = Pcg32::new(42, 2);
        let mut equal = 0;
        for _ in 0..1000 {
            if a.next_u32() == b.next_u32() {
                equal += 1;
            }
        }
        assert!(equal < 5); // extremely unlikely
    }
    #[test]
    fn pcg32_peek_matches_next() {
        let mut rng = Pcg32::new(123, 7);
        let peeked = rng.peek_next_u32();
        let next = rng.next_u32();
        assert_eq!(peeked, next);
    }
    #[test]
    fn pcg32_advance_one_step() {
        let mut a = Pcg32::new(999, 3);
        let mut b = a;
        let _ = a.next_u32();
        b.advance(1);
        assert_eq!(a.inner_state(), b.inner_state());
    }
    #[test]
    fn pcg32_bounded_range() {
        let mut rng = Pcg32::new(1, 1);
        for _ in 0..10_000 {
            let v = rng.next_bounded(7);
            assert!(v < 7);
        }
    }
    #[test]
    fn pcg32_exact_sequence() {
        let mut rng = Pcg32::new(42, 54);
        let expected = [2707161783, 2068313097, 3122475824, 2211639955, 3215226955];
        for &v in &expected {
            assert_eq!(rng.next_u32(), v);
        }
    }
    #[test]
    fn pcg32_fill_bytes_matches_next_u32() {
        let mut rng1 = Pcg32::new(42, 54);
        let mut rng2 = Pcg32::new(42, 54);
        let mut buf = [0u8; 16];
        rng1.fill_bytes(&mut buf);
        let expected = [
            rng2.next_u32().to_le_bytes(),
            rng2.next_u32().to_le_bytes(),
            rng2.next_u32().to_le_bytes(),
            rng2.next_u32().to_le_bytes(),
        ]
        .concat();
        assert_eq!(&buf[..], &expected[..]);
    }
}
