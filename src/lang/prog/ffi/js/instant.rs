// devela/src/lang/prog/ffi/js/instant.rs
//
//! Defines [`JsInstant`], [`JsTimeout`].
//

#[cfg(feature = "time")]
use crate::TimeDelta;
use crate::{_impl_init, Display, impl_trait, js_number, js_uint32};

#[doc = crate::_tags!(runtime time)]
/// A high-resolution timestamp based on JavaScript's `performance.now()`.
#[doc = crate::_doc_meta!{
    location("lang/prog/ffi/js"),
    test_size_of(JsInstant = 8|64),
}]
///
/// The internal representation is a double-precision floating-point millisecond value.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JsInstant {
    /// Milliseconds since `performance.timeOrigin`.
    pub ms: js_number,
}
_impl_init![Self::ZERO => JsInstant];

#[rustfmt::skip]
impl JsInstant {
    /// The instant at `performance.timeOrigin` (`0.0` milliseconds).
    pub const ZERO: Self = Self { ms: 0.0 };

    /// Returns a new `JsInstant` from a timestamp in milliseconds.
    pub const fn from_millis_f64(millis: js_number) -> Self { Self { ms: millis } }
    /// Returns the time in milliseconds.
    pub const fn as_millis_f64(self) -> js_number { self.ms }

    /// Returns a new `JsInstant` from a timestamp in seconds.
    pub const fn from_secs_f64(secs: js_number) -> Self { Self { ms: secs * 1_000.0 } }
    /// Returns the time in `f64` seconds.
    pub const fn as_secs_f64(self) -> js_number { self.ms / 1_000.0 }

    /// Returns the duration between this and an earlier `JsInstant`.
    pub const fn since(self, earlier: Self) -> Self { Self::from_millis_f64(self.ms - earlier.ms) }

    /// Returns the duration between this and an earlier instant as a `TimeDelta`.
    #[cfg(feature = "time")]
    pub const fn delta_since(self, earlier: Self) -> TimeDelta { TimeDelta::from_js(self.since(earlier)) }
}

impl_trait![fmt::Display for JsInstant |self, f| Display::fmt(&self.ms, f)];

#[rustfmt::skip]
#[cfg(feature = "event")]
mod impl_event {
    pub use crate::{EventTimestamp, JsInstant};

    impl EventTimestamp {
        /// Converts a `JsInstant` to an `EventTimestamp`, ensuring a valid value.
        pub const fn from_js(from: JsInstant) -> EventTimestamp {
            EventTimestamp::from_millis_f32(from.as_millis_f64() as f32)
        }
        /// Converts an `EventTimestamp` to a `JsInstant`.
        pub const fn to_js(self) -> JsInstant {
            JsInstant::from_millis_f64(self.as_millis_f32() as f64)
        }
    }
    impl From<JsInstant> for EventTimestamp {
        fn from(from: JsInstant) -> Self { EventTimestamp::from_js(from) }
    }
    impl From<EventTimestamp> for JsInstant {
        fn from(from: EventTimestamp) -> Self { from.to_js() }
    }
}
#[rustfmt::skip]
#[cfg(any(target_arch = "wasm32", doc))]
#[cfg(all(feature = "time", feature = "web", not(windows)))]
mod impl_web {
    use crate::{JsInstant, TimeScale, TimeSource};

    /// Relative `u64` projection of [`JsInstant`] in milliseconds.
    ///
    /// This is the canonical wide numeric projection for the browser high-resolution
    /// time origin. Milliseconds are the native scale exposed by the type's public API.
    ///
    /// At millisecond resolution, `u64` spans about 584 million years.
    impl TimeSource<u64> for JsInstant {
        fn time_is_monotonic() -> bool { true }
        fn time_is_absolute() -> bool { false }
        fn time_scale() -> TimeScale { TimeScale::Millis }
        fn time_now() -> u64 { JsInstant::now().as_millis_f64() as u64 }
        fn time_point_value(point: u64) -> u64 { point }
        fn time_elapsed_value(elapsed: u64) -> u64 { elapsed }
        fn time_now_millis_f64() -> f64 { JsInstant::now().as_millis_f64() }
    }
    /// Relative `u32` projection of [`JsInstant`] in milliseconds.
    ///
    /// This compact projection favors storage size while keeping a practical range
    /// for browser sessions and medium-lived applications.
    ///
    /// At millisecond resolution, `u32` spans about 49.7 days.
    impl TimeSource<u32> for JsInstant {
        fn time_is_monotonic() -> bool { true }
        fn time_is_absolute() -> bool { false }
        fn time_scale() -> TimeScale { TimeScale::Millis }
        fn time_now() -> u32 {
            u32::try_from(JsInstant::now().as_millis_f64() as u64)
                .expect("JsInstant u32 millisecond projection overflow")
        }
        fn time_point_value(point: u32) -> u64 { point.into() }
        fn time_elapsed_value(elapsed: u32) -> u64 { elapsed.into() }
        fn time_now_millis_f64() -> f64 { JsInstant::now().as_millis_f64() }
    }
}

#[doc = crate::_tags!(runtime time uid)]
/// A handle to a JavaScript timeout.
#[doc = crate::_doc_meta!{
    location("lang/prog/ffi/js"),
    test_size_of(JsTimeout = 4|32),
}]
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout#return_value>.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsTimeout {
    pub(crate) id: js_uint32,
}

impl JsTimeout {
    /// Returns a new invalid handle.
    pub const fn invalid() -> Self {
        JsTimeout { id: 0 }
    }
    /// Returns the numeric ID of the handle.
    pub const fn id(self) -> js_uint32 {
        self.id
    }
}
