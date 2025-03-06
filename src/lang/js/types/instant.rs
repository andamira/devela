// devela::lang:js::types::instant

use crate::{impl_trait, Display};
#[allow(unused_imports)]
use crate::{Js, TimeDelta};

/// A high-resolution timestamp based on JavaScript's `performance.now()`.
///
/// The internal representation is a double-precision floating-point millisecond value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JsInstant {
    /// Milliseconds since `performance.timeOrigin`.
    pub ms: f64,
}
#[rustfmt::skip]
impl JsInstant {
    /// Returns the time in `f64` milliseconds.
    pub const fn as_millis_f64(self) -> f64 { self.ms }
    /// Returns a new `JsInstant` from a timestamp in milliseconds.
    pub const fn from_millis_f64(millis: f64) -> Self { Self { ms: millis } }
    /// Returns the time in `f64` seconds.
    pub const fn as_secs_f64(self) -> f64 { self.ms / 1_000.0 }
    /// Returns a new `JsInstant` from a timestamp in milliseconds.
    pub const fn from_secs_f64(secs: f64) -> Self { Self { ms: secs * 1_000.0 } }

    /// Returns the duration between this and an earlier `JsInstant`.
    pub const fn since(self, earlier: Self) -> Self { Self::from_millis_f64(self.ms - earlier.ms) }
    /// Returns the duration between this and an earlier instant as a `TimeDelta`.
    pub fn delta_since(self, earlier: Self) -> TimeDelta { TimeDelta::from_js(self.since(earlier)) }
    /// Returns the duration between this and an earlier instant as a `TimeDelta`.
    pub const fn const_delta_since(self, earlier: Self) -> TimeDelta {
        TimeDelta::const_from_js(self.since(earlier)) }
}

#[rustfmt::skip]
#[cfg(all(not(windows), feature = "unsafe_ffi"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(target_arch = "wasm32")))]
impl JsInstant {
    /// Returns the current instant using `performance.now()`.
    pub fn now() -> Self { Js::performance_now() }
    /// Resets this instant to the current time.
    pub fn reset(&mut self) { *self = Js::performance_now(); }
    /// Returns the elapsed time since this instant.
    pub fn elapsed(self) -> Self { Self::from_millis_f64(Js::performance_now().ms - self.ms) }
    /// Returns the elapsed time since this instant as a `TimeDelta`.
    pub fn delta_elapsed(self) -> TimeDelta { TimeDelta::from_js(self.elapsed()) }
}

impl_trait![fmt::Display for JsInstant |self, f| Display::fmt(&self.ms, f)];
