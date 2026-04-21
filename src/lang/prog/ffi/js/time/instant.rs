// devela::lang::prog::ffi::js::time::instant
//
//! Defines [`JsInstant`].
//

use crate::{Display, TimeDelta, impl_trait};
#[allow(unused_imports)]
use crate::{Web, js_number, js_uint32};

#[doc = crate::_tags!(runtime time)]
/// A high-resolution timestamp based on JavaScript's `performance.now()`.
#[doc = crate::_doc_location!("lang/prog/ffi/js")]
///
/// The internal representation is a double-precision floating-point millisecond value.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JsInstant {
    /// Milliseconds since `performance.timeOrigin`.
    pub ms: js_number,
}
#[rustfmt::skip]
impl JsInstant {
    /// Returns the time in milliseconds.
    pub const fn as_millis_f64(self) -> js_number { self.ms }
    /// Returns a new `JsInstant` from a timestamp in milliseconds.
    pub const fn from_millis_f64(millis: js_number) -> Self { Self { ms: millis } }
    /// Returns the time in `f64` seconds.
    pub const fn as_secs_f64(self) -> js_number { self.ms / 1_000.0 }
    /// Returns a new `JsInstant` from a timestamp in milliseconds.
    pub const fn from_secs_f64(secs: js_number) -> Self { Self { ms: secs * 1_000.0 } }

    /// Returns the duration between this and an earlier `JsInstant`.
    pub const fn since(self, earlier: Self) -> Self { Self::from_millis_f64(self.ms - earlier.ms) }

    /// Returns the duration between this and an earlier instant as a `TimeDelta`.
    pub const fn delta_since(self, earlier: Self) -> TimeDelta { TimeDelta::from_js(self.since(earlier)) }
}

#[rustfmt::skip]
#[cfg(not(feature = "safe_lang"))]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
impl JsInstant {
    /// Returns the current instant using `performance.now()`.
    pub fn now() -> Self { Web::performance_now() }
    /// Returns the time origin using `performance.timeOrigin()`.
    pub fn origin() -> Self { Web::performance_time_origin() }

    /// Resets this instant to the current time.
    pub fn reset(&mut self) { *self = Web::performance_now(); }
    /// Returns the elapsed time since this instant.
    pub fn elapsed(self) -> Self { Self::from_millis_f64(Web::performance_now().ms - self.ms) }
    /// Returns the elapsed time since this instant as a `TimeDelta`.
    pub fn delta_elapsed(self) -> TimeDelta { TimeDelta::from_js(self.elapsed()) }
}

impl_trait![fmt::Display for JsInstant |self, f| Display::fmt(&self.ms, f)];

#[rustfmt::skip]
#[cfg(feature = "event")]
mod impls {
    pub use super::JsInstant;
    pub use crate::EventTimestamp;

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
