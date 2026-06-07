// devela::num::prob::rand::from
//
//! Defines [`FromRand`] and [`FromRandTry`].
//

use crate::{Cast, Rand, RandTry, Result};

#[doc = crate::_tags!(rand construction)]
/// Fallible construction from a source of randomness.
#[doc = crate::_doc_meta!{location("num/prob/rand")}]
///
/// `FromRandTry` defines a type's canonical random construction.
///
/// Implementations should document their distribution and constraints.
/// Types without a clear canonical distribution should use explicit
/// constructors or distribution types instead.
pub trait FromRandTry: Sized {
    /// Attempts to construct a value using `rng`.
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error>;
}

#[doc = crate::_tags!(rand construction)]
/// Infallible construction from a source of randomness.
#[doc = crate::_doc_meta!{location("num/prob/rand")}]
///
/// `FromRand` is implemented automatically for types implementing
/// [`FromRandTry`] when used with an infallible [`Rand`] source.
pub trait FromRand: FromRandTry {
    /// Constructs a value using `rng`.
    #[must_use]
    #[inline(always)]
    fn from_rand<R: Rand + ?Sized>(rng: &mut R) -> Self {
        match Self::from_rand_try(rng) {
            Ok(value) => value,
            Err(error) => match error {},
        }
    }
}

/* impls for primitives */

impl FromRandTry for bool {
    /// Returns `true` or `false` with equal probability.
    #[inline(always)]
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        rng.rand_try_next_bool()
    }
}
impl FromRandTry for char {
    /// Returns a value uniformly distributed across all Unicode scalar values.
    ///
    /// This includes unassigned, private-use, control, and non-printing scalars.
    #[inline]
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        const SCALAR_COUNT: u64 = 0x10_F800;
        const SURROGATE_START: u32 = 0xD800;
        const SURROGATE_COUNT: u32 = 0x800;
        let index = rng.rand_try_below(SCALAR_COUNT)? as u32;
        let scalar = if index < SURROGATE_START { index } else { index + SURROGATE_COUNT };
        match char::from_u32(scalar) {
            Some(ch) => Ok(ch),
            None => unreachable!("mapped value is always a valid Unicode scalar"),
        }
    }
}

/* integer primitives */

impl FromRandTry for u8 {
    /// Returns a value uniformly distributed across all `u8` values.
    #[inline(always)]
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        rng.rand_try_next_u16().map(|n| n as u8)
    }
}
impl FromRandTry for u16 {
    /// Returns a value uniformly distributed across all `u16` values.
    #[inline(always)]
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        rng.rand_try_next_u16()
    }
}
impl FromRandTry for u32 {
    /// Returns a value uniformly distributed across all `u32` values.
    #[inline(always)]
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        rng.rand_try_next_u32()
    }
}
impl FromRandTry for u64 {
    /// Returns a value uniformly distributed across all `u64` values.
    #[inline(always)]
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        rng.rand_try_next_u64()
    }
}
impl FromRandTry for u128 {
    /// Returns a value uniformly distributed across all `u128` values.
    #[inline(always)]
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        let pair = [rng.rand_try_next_u64()?, rng.rand_try_next_u64()?];
        Ok(Cast::<u128>::from_u64_le(pair))
    }
}
impl FromRandTry for usize {
    /// Returns a value uniformly distributed across all `usize` values.
    #[inline(always)]
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        rng.rand_try_next_u64().map(|n| n as usize)
    }
}
impl FromRandTry for i8 {
    /// Returns a value uniformly distributed across all `i16` values.
    #[inline(always)]
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        rng.rand_try_next_u16().map(|n| n as i8)
    }
}
impl FromRandTry for i16 {
    /// Returns a value uniformly distributed across all `i16` values.
    #[inline(always)]
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        rng.rand_try_next_u16().map(|n| n as i16)
    }
}
impl FromRandTry for i32 {
    /// Returns a value uniformly distributed across all `i32` values.
    #[inline(always)]
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        rng.rand_try_next_u32().map(|n| n as i32)
    }
}
impl FromRandTry for i64 {
    /// Returns a value uniformly distributed across all `i64` values.
    #[inline(always)]
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        rng.rand_try_next_u64().map(|n| n as i64)
    }
}
impl FromRandTry for i128 {
    /// Returns a value uniformly distributed across all `i128` values.
    #[inline(always)]
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        let pair = [rng.rand_try_next_u64()?, rng.rand_try_next_u64()?];
        Ok(Cast::<u128>::from_u64_le(pair) as i128)
    }
}
impl FromRandTry for isize {
    /// Returns a value uniformly distributed across all `isize` values.
    #[inline(always)]
    fn from_rand_try<R: RandTry + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        rng.rand_try_next_u64().map(|n| n as isize)
    }
}
