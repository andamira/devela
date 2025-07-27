// devela::lang::ffi::js::types::instant

#[cfg(feature = "time")]
use crate::TimeDelta;
use crate::{Display, impl_trait};
#[allow(unused_imports)]
use crate::{Js, js_number};

/// A high-resolution timestamp based on JavaScript's `performance.now()`.
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
    #[cfg(feature = "time")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
    pub fn delta_since(self, earlier: Self) -> TimeDelta { TimeDelta::from_js(self.since(earlier)) }
    /// Returns the duration between this and an earlier instant as a `TimeDelta`.
    #[cfg(feature = "time")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
    pub const fn const_delta_since(self, earlier: Self) -> TimeDelta {
        TimeDelta::const_from_js(self.since(earlier)) }
}

#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
impl JsInstant {
    /// Returns the current instant using `performance.now()`.
    pub fn now() -> Self { Js::performance_now() }
    /// Returns the time origin using `performance.timeOrigin()`.
    pub fn origin() -> Self { Js::performance_time_origin() }

    /// Resets this instant to the current time.
    pub fn reset(&mut self) { *self = Js::performance_now(); }
    /// Returns the elapsed time since this instant.
    pub fn elapsed(self) -> Self { Self::from_millis_f64(Js::performance_now().ms - self.ms) }
    /// Returns the elapsed time since this instant as a `TimeDelta`.
    #[cfg(feature = "time")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
    pub fn delta_elapsed(self) -> TimeDelta { TimeDelta::from_js(self.elapsed()) }
}

impl_trait![fmt::Display for JsInstant |self, f| Display::fmt(&self.ms, f)];
