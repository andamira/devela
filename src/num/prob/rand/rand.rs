// devela::num::prob::rand::rand
//
//! Defines [`Rand`].
//

use crate::RandQualities;

#[doc = crate::_tags!(rand)]
/// Fallible source of raw random data.
#[doc = crate::_doc_meta!{location("num/prob/rand")}]
///
/// `RandTry` is the core trait for random sources that may fail, such as
/// OS calls, hardware RNGs, host APIs, finite buffers, and fallible adapters.
///
/// Source behavior and suitability are described by [`RandQualities`].
///
/// Infallible sources use [`Infallible`][crate::Infallible] as their error type.
// and automatically implement [`Rand`]. TODO
pub trait RandTry {
    /* required */

    /// Error returned when complete random data could not be produced.
    type Error;

    /// Number of bits produced per output step.
    ///
    /// Adapters or composite sources may report a conservative value.
    const RAND_OUTPUT_BITS: u32;

    /// Size of the source state owned by this value, in bits.
    ///
    /// External entropy sources that do not store generator state
    /// in the value itself should use `0`.
    const RAND_STATE_BITS: u32;

    /// Semantic qualities of this random source.
    const RAND_QUALITIES: RandQualities;

    /// Attempts to produce the next 64 bits of randomness.
    fn rand_try_next_u64(&mut self) -> Result<u64, Self::Error>;

    /*** provided ***/

    /// Returns the semantic qualities of this random source.
    #[must_use]
    #[inline(always)]
    fn rand_qualities(&self) -> RandQualities {
        Self::RAND_QUALITIES
    }

    /* derived primitives and byte access */

    /// Attempts to produce the next random `u32`.
    #[inline(always)]
    fn rand_try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.rand_try_next_u64()? as u32)
    }
    /// Attempts to produce the next random `u16`.
    #[inline(always)]
    fn rand_try_next_u16(&mut self) -> Result<u16, Self::Error> {
        Ok(self.rand_try_next_u64()? as u16)
    }
    /// Attempts to produce a random boolean.
    #[inline(always)]
    fn rand_try_next_bool(&mut self) -> Result<bool, Self::Error> {
        Ok((self.rand_try_next_u64()? & 1) != 0)
    }

    /// Attempts to fill `buf` with random data.
    ///
    /// This method must either fill the whole buffer or return an error.
    fn rand_try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        let mut i = 0;
        let len = buf.len();
        while i + 8 <= len {
            buf[i..i + 8].copy_from_slice(&self.rand_try_next_u64()?.to_ne_bytes());
            i += 8;
        }
        if i < len {
            let tail = self.rand_try_next_u64()?.to_ne_bytes();
            buf[i..].copy_from_slice(&tail[..len - i]);
        }
        Ok(())
    }

    /* bounded sampling */

    /// Attempts to return a uniformly random integer in `0..upper`.
    fn rand_try_below(&mut self, upper: u64) -> Result<u64, Self::Error> {
        assert!(upper > 0);
        let zone = u64::MAX - (u64::MAX % upper);
        loop {
            let v = self.rand_try_next_u64()?;
            if v < zone {
                return Ok(v % upper);
            }
        }
    }
    /// Attempts to return a uniformly random integer in `low..high`.
    #[inline(always)]
    fn rand_try_range(&mut self, low: u64, high: u64) -> Result<u64, Self::Error> {
        assert!(low < high);
        Ok(low + self.rand_try_below(high - low)?)
    }

    /* common discrete helpers */

    /// Attempts to roll a fair die with a number of `sides`.
    #[inline(always)]
    fn rand_try_roll(&mut self, sides: u64) -> Result<u64, Self::Error> {
        Ok(self.rand_try_below(sides)? + 1)
    }
    /// Attempts to shuffle the elements of the slice in place.
    fn rand_try_shuffle<T>(&mut self, slice: &mut [T]) -> Result<(), Self::Error> {
        let mut i = slice.len();
        while i > 1 {
            i -= 1;
            let j = self.rand_try_below((i + 1) as u64)? as usize;
            slice.swap(i, j);
        }
        Ok(())
    }
}

#[doc = crate::_tags!(rand)]
/// Core random number generator behavior.
#[doc = crate::_doc_meta!{location("num/prob/rand")}]
///
/// `Rand` represents a deterministic source of raw randomness.
/// It defines a minimal required primitive and provides a broad
/// set of derived, allocation-free convenience methods.
///
/// The provided methods favor clarity and generic correctness.
/// Concrete generators may override them for efficiency.
pub trait Rand {
    /*** required ***/

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

    /*** provided ***/

    /* derived primitives and byte access */

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
