// devela/src/data/id/uuid/generator.rs
//
//! Defines [`UuidV7Generator`].
//

use crate::{ConstInit, Pcg32, RandTry, is, read_at, unwrap, write_at};
#[cfg(feature = "time")]
use crate::{TimePoint, TimeSource, TimeSourceCfg};
use crate::{Uuid, UuidNonNil, UuidVersion};

#[doc = crate::_tags!(uid state)]
/// A stateful monotonic UUID version 7 generator.
#[doc = crate::_doc_meta!{
    location("data/id/uuid", struct UuidV7Generator),
    test_size_of(UuidV7Generator = 16|128; niche !Option),
}]
/// The generator retains its preceding UUID as state. Each successfully
/// generated UUID is strictly greater than the preceding one.
///
/// When the supplied timestamp advances, a new UUID is initialized from that
/// timestamp and fresh random material. When the timestamp is unchanged or
/// moves backwards, the previous timestamp is retained and its `rand_b` field
/// is advanced by a positive increment derived from the supplied random material.
///
/// This preserves UUID ordering across repeated millisecond timestamps
/// and clock rollback within one generator state.
///
/// Generation returns `None` if the timestamp exceeds the UUID version 7 range
/// or if `rand_b` cannot be advanced without overflow. The generator state
/// is unchanged when generation fails.
///
/// The ordering guarantee applies to a single state lineage.
/// Independently restored generators are not coordinated.
#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub struct UuidV7Generator {
    last: Option<UuidNonNil>,
}

impl UuidV7Generator {
    /* constructors */

    /// Creates an empty UUIDv7 generator.
    pub const fn new() -> Self {
        Self { last: None }
    }
    /// Restores a generator using a UUID version 7 value as its preceding state.
    ///
    /// Returns `None` if `last` is not a recognized IETF version 7 UUID.
    pub const fn from_last(last: UuidNonNil) -> Option<Self> {
        unwrap![some_or last.version(), UuidVersion::V7 => Some(Self { last: Some(last) }), None]
    }

    /* state */

    /// Returns the UUID currently retained as generator state.
    pub const fn last(&self) -> Option<UuidNonNil> {
        self.last
    }
    /// Returns the UUID currently retained as generator state,
    /// consuming the generator.
    pub const fn into_last(self) -> Option<UuidNonNil> {
        self.last
    }

    /* generation */

    /// Generates the next monotonic UUID version 7
    /// from a Unix millisecond timestamp and random material.
    ///
    /// If `unix_ts_ms` is greater than the timestamp retained by the generator,
    /// it begins a new timestamp using `random`. Otherwise the retained timestamp
    /// is preserved and `random` is used to derive a positive `rand_b` increment.
    ///
    /// Returns `None` if `unix_ts_ms` exceeds the UUID version 7 timestamp range,
    /// or if the monotonic `rand_b` field would overflow.
    ///
    /// The generator state is unchanged on failure.
    pub const fn next_random(&mut self, unix_ts_ms: u64, random: [u8; 10]) -> Option<UuidNonNil> {
        is! { unix_ts_ms > Uuid::V7_UNIX_TS_MS_MAX, return None }
        let last = unwrap![some_or self.last, return self._start_timestamp(unix_ts_ms, random)];
        let last_ts = unwrap![some? last.into_uuid().unix_ts_ms_v7()];
        is! { unix_ts_ms > last_ts,
            self._start_timestamp(unix_ts_ms, random), self._advance(last, random)
        }
    }
    /// Generates the next monotonic UUID version 7 using a Unix millisecond timestamp
    /// and deterministic randomness from [`Pcg32`].
    ///
    /// This is suitable when reproducibility is desired;
    /// it does not provide cryptographic unpredictability.
    ///
    /// Returns `None` if `unix_ts_ms` exceeds the UUID version 7 timestamp range,
    /// or if the monotonic `rand_b` field would overflow.
    ///
    /// An out-of-range timestamp is rejected without advancing `rng`.
    pub const fn next_pcg32(&mut self, unix_ts_ms: u64, rng: &mut Pcg32) -> Option<UuidNonNil> {
        is! { unix_ts_ms > Uuid::V7_UNIX_TS_MS_MAX, return None }
        let mut random = [0; 10];
        rng.fill_bytes(&mut random);
        self.next_random(unix_ts_ms, random)
    }
    /// Generates the next monotonic UUID version 7 using the current time from
    /// an absolute [`TimeSource`] and deterministic randomness from [`Pcg32`].
    ///
    /// Returns `None` if the source is not absolute, if its current Unix
    /// millisecond timestamp exceeds the UUID version 7 range,
    /// or if the monotonic `rand_b` field would overflow.
    #[cfg(feature = "time")]
    pub fn next_pcg32_now<T, P>(&mut self, rng: &mut Pcg32) -> Option<UuidNonNil>
    where
        T: TimeSource<P>,
        P: TimePoint,
    {
        is! { !T::time_is_absolute(), return None }
        self.next_pcg32(T::time_now_millis(), rng)
    }
    /// Generates the next monotonic UUID version 7 using the current time from
    /// an absolute configured [`TimeSourceCfg`] and deterministic randomness
    /// from [`Pcg32`].
    ///
    /// Returns `None` if the source is not absolute, if its current Unix
    /// millisecond timestamp exceeds the UUID version 7 range,
    /// or if the monotonic `rand_b` field would overflow.
    #[cfg(feature = "time")]
    pub fn next_pcg32_now_cfg<T, P>(
        &mut self,
        cfg: T::Config,
        rng: &mut Pcg32,
    ) -> Option<UuidNonNil>
    where
        T: TimeSourceCfg<P>,
        P: TimePoint,
    {
        is! { !T::time_is_absolute(cfg), return None }
        self.next_pcg32(T::time_now_millis(cfg), rng)
    }
    /// Generates the next monotonic UUID version 7 using a Unix millisecond timestamp
    /// and a fallible random source.
    ///
    /// Returns:
    /// - `Ok(Some(uuid))` on successful generation,
    /// - `Ok(None)` if the timestamp is outside the UUID version 7 range
    ///   or the monotonic `rand_b` field would overflow,
    /// - `Err(error)` if the random source fails.
    ///
    /// The generator state is unchanged on `Ok(None)` or `Err`.
    pub fn next_rand_try<R: RandTry + ?Sized>(
        &mut self,
        unix_ts_ms: u64,
        rng: &mut R,
    ) -> Result<Option<UuidNonNil>, R::Error> {
        is! { unix_ts_ms > Uuid::V7_UNIX_TS_MS_MAX, return Ok(None) }
        let mut random = [0; 10];
        rng.rand_try_fill_bytes(&mut random)?;
        Ok(self.next_random(unix_ts_ms, random))
    }

    /* helpers */

    /// Maximum value of the 62-bit UUIDv7 `rand_b` field.
    const RAND_B_MAX: u64 = 0x3FFF_FFFF_FFFF_FFFF;

    const fn _start_timestamp(
        &mut self,
        unix_ts_ms: u64,
        mut random: [u8; 10],
    ) -> Option<UuidNonNil> {
        // rand_b begins at random[2].
        // Reserve its most-significant bit as a rollover guard.
        random[2] &= 0x1F;
        let uuid = unwrap![some? Uuid::from_random_v7(unix_ts_ms, random)];
        let uuid = unwrap![some_guaranteed_or_ub UuidNonNil::from_uuid(uuid)]; // safe for UUIDv7 
        self.last = Some(uuid);
        Some(uuid)
    }
    const fn _advance(&mut self, last: UuidNonNil, random: [u8; 10]) -> Option<UuidNonNil> {
        let mut bytes = last.into_bytes();
        let mut rand_b = read_at!(bytes, 8, @8);
        rand_b[0] &= 0x3F;
        let rand_b = u64::from_be_bytes(rand_b);
        let increment = (u16::from_be_bytes(read_at!(random, 0, @2)) & 0x0FFF) as u64 + 1;
        let next = unwrap![some_if? rand_b.checked_add(increment), |next| next <= Self::RAND_B_MAX];
        let mut next = next.to_be_bytes();
        next[0] = 0x80 | (next[0] & 0x3F);
        write_at!(bytes, 8, @8 next);
        let uuid = unwrap![some? UuidNonNil::from_bytes(bytes)];
        self.last = Some(uuid);
        Some(uuid)
    }
}

/* trait impls */

impl ConstInit for UuidV7Generator {
    const INIT: Self = Self::new();
}
impl Default for UuidV7Generator {
    fn default() -> Self {
        Self::new()
    }
}
