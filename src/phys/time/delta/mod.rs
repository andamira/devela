// devela::phys::time::delta
//
//! Defines the [`TimeDelta`] struct.
//
// TOC
// - definitions
// - conversions
//   - duration
//   - system_instant

mod basic;
mod ops;

#[cfg(test)]
mod tests;

#[doc = crate::_TAG_TIME!()]
/// A signed duration of time, stored as an `(i64, i32)` pair of secs and nanos.
#[doc = crate::_doc_location!("phys/time")]
///
/// Supports negative values, allowing representation of both past and future offsets.
#[doc = crate::_doc!(vendor: "jiff")]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeDelta {
    secs: i64,
    nanos: i32,
}

const NANOS_PER_SEC: i32 = 1_000_000_000;
const NANOS_PER_MILLI: i32 = 1_000_000;
const NANOS_PER_MICRO: i32 = 1_000;
const MILLIS_PER_SEC: i64 = 1_000;
const MICROS_PER_SEC: i64 = 1_000_000;
const SECS_PER_MINUTE: i64 = 60;
const MINS_PER_HOUR: i64 = 60;

/* conversions */

#[rustfmt::skip]
mod impl_duration {
    use crate::{Duration, TimeDelta};

    /// # Additional APIs involving [`Duration`].
    impl TimeDelta {
        /// Converts a `Duration` to a `TimeDelta`.
        pub const fn from_duration(duration: Duration) -> Self {
            Self {
                secs: duration.as_secs() as i64,
                nanos: duration.subsec_nanos() as i32,
            }
        }

        /// Converts a `TimeDelta` to a `Duration` if it is non-negative.
        /// Returns `None` if the `TimeDelta` is negative.
        pub const fn to_duration(&self) -> Option<Duration> {
            if self.is_negative() { None }
            else { Some(Duration::new(self.secs as u64, self.nanos as u32)) }
        }

        /// Returns the absolute value of this time delta as a [`Duration`].
        ///
        /// This method cannot panic because the absolute value of `TimeDelta::MIN`
        /// is always representable in a `Duration`.
        pub const fn abs_duration(self) -> Duration {
            Duration::new(self.secs.unsigned_abs(), self.nanos.unsigned_abs())
        }

        /// Adds a `Duration` to this `TimeDelta`.
        /// Returns `None` if the result overflows.
        pub const fn checked_add_duration(&self, duration: Duration) -> Option<Self> {
            let duration_delta = Self::from_duration(duration);
            let Some(secs) = self.secs.checked_add(duration_delta.secs) else { return None; };
            let nanos = self.nanos + duration_delta.nanos;
            Some(Self::new(secs, nanos))
        }

        /// Subtracts a `Duration` from this `TimeDelta`.
        /// Returns `None` if the result underflows.
        pub const fn checked_sub_duration(&self, duration: Duration) -> Option<Self> {
            let duration_delta = Self::from_duration(duration);
            let Some(secs) = self.secs.checked_sub(duration_delta.secs) else { return None; };
            let nanos = self.nanos - duration_delta.nanos;
            Some(Self::new(secs, nanos))
        }
    }

    impl TryFrom<Duration> for TimeDelta {
        type Error = &'static str;

        fn try_from(d: Duration) -> Result<TimeDelta, Self::Error> {
            let secs = i64::try_from(d.as_secs())
                .map_err(|_| "seconds in unsigned duration {d:?} overflowed i64")?;
            // Guaranteed to succeed since 0<=nanos<=999,999,999.
            let nanos = i32::try_from(d.subsec_nanos()).unwrap();
            Ok(TimeDelta::new_unchecked(secs, nanos))
        }
    }
    impl TryFrom<TimeDelta> for Duration {
        type Error = &'static str;

        fn try_from(sd: TimeDelta) -> Result<Duration, Self::Error> {
            // This isn't needed, but improves error messages.
            if sd.is_negative() {
                return Err("cannot convert negative duration `{sd:?}` to \
                     unsigned `std::time::Duration`");
            }
            let secs = u64::try_from(sd.as_secs())
                .map_err(|_| "seconds in signed duration {sd:?} overflowed u64")?;
            // Guaranteed to succeed because the above only succeeds
            // when `sd` is non-negative. And when `sd` is non-negative,
            // we are guaranteed that 0<=nanos<=999,999,999.
            let nanos = u32::try_from(sd.subsec_nanos()).unwrap();
            Ok(Duration::new(secs, nanos))
        }
    }
}

#[rustfmt::skip]
#[cfg(all(feature = "js", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "js")))]
mod impl_js {
    use crate::{JsInstant, TimeDelta};
    impl TimeDelta {
        /// Converts a `JsInstant` into a `TimeDelta` relative to the time origin.
        pub const fn from_js(js: JsInstant) -> Self { Self::from_millis_f64(js.as_millis_f64()) }
        /// Converts a `TimeDelta` into a `JsInstant`, interpreting it as an absolute timestamp.
        pub const fn to_js(self) -> JsInstant { JsInstant::from_millis_f64(self.as_millis_f64()) }
    }
}

#[rustfmt::skip]
#[cfg(feature = "dep_jiff")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_jiff")))]
mod impl_jiff {
    use crate::{TimeDelta, _dep::jiff::SignedDuration};
    impl TimeDelta {
        /// Converts [`SignedDuration`] into [`TimeDelta`].
        pub const fn from_jiff(from: SignedDuration) -> TimeDelta {
            TimeDelta::new(from.as_secs(), from.subsec_nanos())
        }
        /// Converts [`TimeDelta`] into [`SignedDuration`].
        pub const fn to_jiff(self) -> SignedDuration { SignedDuration::new(self.secs, self.nanos) }
    }
    impl From<SignedDuration> for TimeDelta {
        fn from(from: SignedDuration) -> TimeDelta { Self::from_jiff(from) }
    }
    impl From<TimeDelta> for SignedDuration {
        fn from(from: TimeDelta) -> SignedDuration { from.to_jiff() }
    }
}
