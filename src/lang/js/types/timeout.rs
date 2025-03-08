// devela::lang:js::types::timeout
//

#[cfg(all(feature = "unsafe_ffi", not(windows)))]
use crate::web_api;
#[allow(unused_imports)]
use crate::Js;

/// A handle to a JavaScript timeout.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout#return_value>.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsTimeout {
    pub(in crate::lang::js) id: u32,
}

#[rustfmt::skip]
impl JsTimeout {
    /// Returns a new invalid handle.
    pub const fn invalid() -> Self { JsTimeout { id: 0 } }
    /// Returns the numeric ID of the handle.
    pub const fn id(self) -> u32 { self.id }
}

#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
impl JsTimeout {
    #[doc = web_api!("Window", "setTimeout")]
    /// Calls a function after a delay in milliseconds.
    pub fn timeout(callback: extern "C" fn(), delay_ms: u32) -> Self {
        Js::window_set_timeout(callback, delay_ms) }
    #[doc = web_api!("Window", "setInterval")]
    /// Calls a function repeatedly at a fixed interval in milliseconds.
    pub fn interval(callback: extern "C" fn(), interval_ms: u32) -> Self {
        Js::window_set_timeout(callback, interval_ms) }

    /// Executes JavaScript code immediately.
    /// ## Security Warning
    /// - Avoid passing untrusted input, as this executes arbitrary JS.
    /// - Ensure all evaluated code is **safe and controlled**.
    pub fn eval(js_code: &str) { Js::window_eval(js_code) }
    #[doc = web_api!("Window", "setTimeout")]
    /// Executes JavaScript code after a delay in milliseconds.
    pub fn eval_timeout(js_code: &str, delay_ms: u32) -> Self {
        Js::window_eval_timeout(js_code, delay_ms) }
    #[doc = web_api!("Window", "setInterval")]
    /// Executes JavaScript code repeatedly at a fixed interval in milliseconds.
    pub fn eval_interval(js_code: &str, interval_ms: u32) -> Self {
        Js::window_eval_interval(js_code, interval_ms) }

    #[doc = web_api!("Window", "clearTimeout")]
    #[doc = web_api!("Window", "clearInterval")]
    /// Cancels a timeout or interval.
    pub fn clear(self) { Js::window_clear_timeout(self); }
}
