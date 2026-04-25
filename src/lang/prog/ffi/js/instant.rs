// devela::lang::prog::ffi::js::instant
//
//! Defines [`JsInstant`], [`JsTimeout`].
//

use crate::{Display, TimeDelta, impl_trait};
use crate::{js_number, js_uint32};

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

#[doc = crate::_tags!(runtime time uid)]
/// A handle to a JavaScript timeout.
#[doc = crate::_doc_location!("lang/prog/ffi/js")]
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
