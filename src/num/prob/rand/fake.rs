// devela::num::prob::rand::fake
//
//! Defines [`RandFake`].
//

use crate::{Infallible, InfallibleResult, NotEnoughElements, RandQualities, RandTry};

#[doc = crate::_tags!(rand fake)]
/// A test-friendly random source backed by a fixed sequence.
#[doc = crate::_doc_meta!{
    location("num/prob/rand"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: RandFake<4> = 36),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: RandFake<4> = 40),
}]
///
/// `RandFake` replays caller-provided `u64` values so
/// random-dependent code can be tested with a known stream.
///
/// It is not a statistical generator.
///
/// By default, `RandFake<N>` is fallible: exhaustion is reported
/// through [`RandTry`] using [`NotEnoughElements`].
///
/// `RandFake<N, true>` is infallible at the type level: exhaustion panics,
/// so it also automatically implements [`Rand`][crate::Rand].
///
/// # Const parameters
/// - `N`: number of scripted `u64` values.
/// - `PANIC`: whether exhaustion panics instead of returning an error.
#[derive(Clone, Copy, Debug)]
pub struct RandFake<const N: usize, const PANIC: bool = false> {
    values: [u64; N],
    index: usize,
}

impl<const N: usize, const PANIC: bool> RandFake<N, PANIC> {
    /// Creates a fake random source from a fixed output sequence.
    pub const fn new(values: [u64; N]) -> Self {
        Self { values, index: 0 }
    }
    /// Returns the number of values already consumed.
    pub const fn index(&self) -> usize {
        self.index
    }
    /// Returns the number of values not yet consumed.
    pub const fn remaining(&self) -> usize {
        N - self.index
    }
    /// Returns whether no values remain.
    pub const fn is_empty(&self) -> bool {
        self.index == N
    }
    /// Resets the source to the beginning of the sequence.
    pub const fn reset(&mut self) {
        self.index = 0;
    }
}
impl<const N: usize> RandFake<N, false> {
    /// Converts this source into panicking mode.
    ///
    /// In panicking mode, exhaustion panics instead of returning
    /// [`NotEnoughElements`]. This makes the source implement [`Rand`][crate::Rand].
    pub const fn panicking(self) -> RandFake<N, true> {
        RandFake { values: self.values, index: self.index }
    }
}
impl<const N: usize> RandFake<N, true> {
    /// Converts this source into fallible mode.
    ///
    /// In fallible mode, exhaustion is reported through [`RandTry`]
    /// as [`NotEnoughElements`].
    pub const fn fallible(self) -> RandFake<N, false> {
        RandFake { values: self.values, index: self.index }
    }
}

/// # Fallible implementation
impl<const N: usize> RandTry for RandFake<N, false> {
    type Error = NotEnoughElements;

    const RAND_OUTPUT_BITS: u32 = 64;
    const RAND_STATE_BITS: u32 = usize::BITS;
    const RAND_QUALITIES: RandQualities = RandQualities::WEAK_PRNG;

    /// Returns the next scripted value.
    ///
    /// Returns [`NotEnoughElements`] if the sequence is exhausted.
    fn rand_try_next_u64(&mut self) -> Result<u64, Self::Error> {
        if self.index < N {
            let value = self.values[self.index];
            self.index += 1;
            Ok(value)
        } else {
            Err(NotEnoughElements(Some(1)))
        }
    }
}

/// # Panicking implementation
impl<const N: usize> RandTry for RandFake<N, true> {
    type Error = Infallible;

    const RAND_OUTPUT_BITS: u32 = 64;
    const RAND_STATE_BITS: u32 = usize::BITS;
    const RAND_QUALITIES: RandQualities = RandQualities::WEAK_PRNG;

    /// Returns the next scripted value.
    ///
    /// # Panics
    /// Panics if the sequence is exhausted.
    #[track_caller]
    fn rand_try_next_u64(&mut self) -> InfallibleResult<u64> {
        if self.index < N {
            let value = self.values[self.index];
            self.index += 1;
            Ok(value)
        } else {
            panic!("RandFake exhausted")
        }
    }
}
