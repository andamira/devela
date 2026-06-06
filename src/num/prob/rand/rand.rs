// devela::num::prob::rand::rand
//
//! Defines [`Rand`].
//

// use crate::Infallible;

#[doc = crate::_tags!(rand)]
/// Fallible source of raw random data.
#[doc = crate::_doc_meta!{location("num/prob/rand")}]
///
/// `RandTry` is the core trait for random sources that may fail,
/// such as OS calls, hardware RNGs, host APIs, finite buffers,
/// and fallible adapters.
///
/// Infallible sources use [`Infallible`] as their error type and
/// automatically implement [`Rand`].
pub trait RandTry {
    /*** required ***/

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

    /// Attempts to produce the next 64 bits of randomness.
    fn rand_try_next_u64(&mut self) -> Result<u64, Self::Error>;

    /*** provided ***/

    /* characteristics */

    /// Whether the source is deterministic.
    ///
    /// A deterministic source produces a stream determined by its current state.
    /// This does not imply that the state is public, seedable, or reproducible.
    const RAND_IS_DETERMINISTIC: bool = false;

    /// Whether the source can reproduce streams from explicit seed material.
    ///
    /// This is normally true for seedable PRNGs and false for OS, hardware,
    /// address-derived, and host-provided sources.
    const RAND_IS_REPRODUCIBLE: bool = false;

    /// Whether the source is intended for cryptographic use.
    ///
    /// This is a semantic claim by the implementor, not a proof.
    const RAND_IS_CRYPTOGRAPHIC: bool = false;

    /// Whether the source is known to be weak.
    ///
    /// Weak sources may still be useful for fallback seeding, tests, jitter,
    /// address diffusion, or non-security variation.
    const RAND_IS_WEAK: bool = false;

    /// Whether the source depends on state outside the value itself.
    ///
    /// OS, hardware, host, file-descriptor, and runtime-provided sources
    /// should normally set this to `true`.
    const RAND_IS_EXTERNAL: bool = false;

    /// Whether the source may block while producing random data.
    ///
    /// Sources that return an error instead of blocking should use `false`.
    /// Ordinary in-memory generators normally keep the default.
    const RAND_MAY_BLOCK: bool = false;

    /// Returns whether the source is deterministic.
    #[inline(always)]
    fn rand_is_deterministic(&self) -> bool {
        Self::RAND_IS_DETERMINISTIC
    }
    /// Returns whether the source can reproduce streams from explicit seed material.
    #[inline(always)]
    fn rand_is_reproducible(&self) -> bool {
        Self::RAND_IS_REPRODUCIBLE
    }
    /// Returns whether the source is intended for cryptographic use.
    #[inline(always)]
    fn rand_is_cryptographic(&self) -> bool {
        Self::RAND_IS_CRYPTOGRAPHIC
    }
    /// Returns whether the source is known to be weak.
    #[inline(always)]
    fn rand_is_weak(&self) -> bool {
        Self::RAND_IS_WEAK
    }
    /// Returns whether the source depends on state outside the value itself.
    #[inline(always)]
    fn rand_is_external(&self) -> bool {
        Self::RAND_IS_EXTERNAL
    }
    /// Returns whether the source may block while producing random data.
    #[inline(always)]
    fn rand_may_block(&self) -> bool {
        Self::RAND_MAY_BLOCK
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
