// devela_base_core::num::prob::rand::rand
//
//! Defines [`Rand`].
//

#[doc = crate::_tags!(rand)]
/// Core random number generator behavior.
#[doc = crate::_doc_location!("num/prob/rand")]
///
/// `Rand` represents a deterministic source of raw randomness.
/// It defines a minimal required primitive and provides a broad
/// set of derived, allocation-free convenience methods.
///
/// The provided methods favor clarity and generic correctness.
/// Concrete generators may override them for efficiency.
pub trait Rand {
    /* required */
    /* -----------------------------------------------------------------------*/

    /// Number of bits produced per output step.
    ///
    /// Adapters or composite generators may report a conservative value.
    const RAND_OUTPUT_BITS: u32;

    /// Size of the internal state, in bits.
    ///
    /// This value describes the entropy capacity of the generator state.
    const RAND_STATE_BITS: u32;

    /// Advances the generator and returns the next 64 bits of randomness.
    fn rand_next_u64(&mut self) -> u64;

    /* provided */
    /* -----------------------------------------------------------------------*/

    /* derived primitives */

    /// Returns the next random `u32`.
    #[inline(always)]
    fn rand_next_u32(&mut self) -> u32 {
        self.rand_next_u64() as u32
    }
    /// Returns the next random `u16`.
    #[inline(always)]
    fn rand_next_u16(&mut self) -> u16 {
        self.rand_next_u64() as u16
    }

    /// Returns a random boolean with equal probability.
    #[inline(always)]
    fn rand_next_bool(&mut self) -> bool {
        (self.rand_next_u64() & 1) != 0
    }

    /* byte access */

    /// Fill `buf` with random data.
    fn rand_fill_bytes(&mut self, buf: &mut [u8]) {
        let mut i = 0;
        let len = buf.len();
        while i + 8 <= len {
            buf[i..i + 8].copy_from_slice(&self.rand_next_u64().to_ne_bytes());
            i += 8;
        }
        if i < len {
            let tail = self.rand_next_u64().to_ne_bytes();
            buf[i..].copy_from_slice(&tail[..len - i]);
        }
    }

    /* bounded sampling */

    /// Returns a uniformly random integer in `0..upper`.
    ///
    /// Uses rejection sampling to avoid modulo bias.
    fn rand_below(&mut self, upper: u64) -> u64 {
        assert!(upper > 0);
        let zone = u64::MAX - (u64::MAX % upper);
        loop {
            let v = self.rand_next_u64();
            if v < zone {
                return v % upper;
            }
        }
    }

    /// Returns a uniformly random integer in `low..high`.
    ///
    /// Panics if `low >= high`.
    #[inline(always)]
    fn rand_range(&mut self, low: u64, high: u64) -> u64 {
        assert!(low < high);
        low + self.rand_below(high - low)
    }

    /* common discrete helpers */

    /// Simulates rolling a fair die with the given number of sides.
    ///
    /// Returns a value in `1..=sides`.
    #[inline(always)]
    fn rand_roll(&mut self, sides: u64) -> u64 {
        self.rand_below(sides) + 1
    }

    /// Randomly shuffles the elements of the slice in place.
    ///
    /// Uses the Fisher–Yates algorithm.
    fn rand_shuffle<T>(&mut self, slice: &mut [T]) {
        let mut i = slice.len();
        while i > 1 {
            i -= 1;
            let j = self.rand_below((i + 1) as u64) as usize;
            slice.swap(i, j);
        }
    }
}
