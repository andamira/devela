// devela::lang::ffi::js::time
//
//! Defines [`JsInstant`] & [`JsTimeout`].
//

#[cfg(feature = "time")]
use crate::TimeDelta;
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
use crate::{js_doc, WebWindow};
use crate::{Display, impl_trait};
#[allow(unused_imports)]
use crate::{Web, js_number, js_uint32};

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
    pub fn now() -> Self { Web::performance_now() }
    /// Returns the time origin using `performance.timeOrigin()`.
    pub fn origin() -> Self { Web::performance_time_origin() }

    /// Resets this instant to the current time.
    pub fn reset(&mut self) { *self = Web::performance_now(); }
    /// Returns the elapsed time since this instant.
    pub fn elapsed(self) -> Self { Self::from_millis_f64(Web::performance_now().ms - self.ms) }
    /// Returns the elapsed time since this instant as a `TimeDelta`.
    #[cfg(feature = "time")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
    pub fn delta_elapsed(self) -> TimeDelta { TimeDelta::from_js(self.elapsed()) }
}

impl_trait![fmt::Display for JsInstant |self, f| Display::fmt(&self.ms, f)];

/// A handle to a JavaScript timeout.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout#return_value>.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsTimeout {
    pub(in crate::lang::ffi::js) id: js_uint32,
}

#[rustfmt::skip]
impl JsTimeout {
    /// Returns a new invalid handle.
    pub const fn invalid() -> Self { JsTimeout { id: 0 } }
    /// Returns the numeric ID of the handle.
    pub const fn id(self) -> js_uint32 { self.id }
}

#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
impl JsTimeout {
    #[doc = js_doc!("Window", "setTimeout")]
    /// Calls a function after a delay in milliseconds.
    pub fn timeout(callback: extern "C" fn(), delay_ms: js_uint32) -> Self {
        WebWindow::set_timeout(callback, delay_ms) }
    #[doc = js_doc!("Window", "setInterval")]
    /// Calls a function repeatedly at a fixed interval in milliseconds.
    pub fn interval(callback: extern "C" fn(), interval_ms: js_uint32) -> Self {
        WebWindow::set_timeout(callback, interval_ms) }

    /// Executes JavaScript code immediately.
    /// ## Security Warning
    /// - Avoid passing untrusted input, as this executes arbitrary JS.
    /// - Ensure all evaluated code is **safe and controlled**.
    pub fn eval(js_code: &str) { WebWindow::eval(js_code) }
    #[doc = js_doc!("Window", "setTimeout")]
    /// Executes JavaScript code after a delay in milliseconds.
    pub fn eval_timeout(js_code: &str, delay_ms: js_uint32) -> Self {
        WebWindow::eval_timeout(js_code, delay_ms) }
    #[doc = js_doc!("Window", "setInterval")]
    /// Executes JavaScript code repeatedly at a fixed interval in milliseconds.
    pub fn eval_interval(js_code: &str, interval_ms: js_uint32) -> Self {
        WebWindow::eval_interval(js_code, interval_ms) }

    #[doc = js_doc!("Window", "clearTimeout")]
    #[doc = js_doc!("Window", "clearInterval")]
    /// Cancels a timeout or interval.
    pub fn clear(self) { WebWindow::clear_timeout(self); }
}
