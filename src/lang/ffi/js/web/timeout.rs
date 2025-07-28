// devela::lang::ffi::js::types::timeout
//

#[cfg(all(feature = "unsafe_ffi", not(windows)))]
use crate::docweb;
#[allow(unused_imports)]
use crate::{Web, js_uint32};

/// A handle to a JavaScript timeout.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout#return_value>.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebTimeout {
    pub(in crate::lang::ffi::js) id: js_uint32,
}

#[rustfmt::skip]
impl WebTimeout {
    /// Returns a new invalid handle.
    pub const fn invalid() -> Self { WebTimeout { id: 0 } }
    /// Returns the numeric ID of the handle.
    pub const fn id(self) -> js_uint32 { self.id }
}

#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
impl WebTimeout {
    #[doc = docweb!("Window", "setTimeout")]
    /// Calls a function after a delay in milliseconds.
    pub fn timeout(callback: extern "C" fn(), delay_ms: js_uint32) -> Self {
        Web::window_set_timeout(callback, delay_ms) }
    #[doc = docweb!("Window", "setInterval")]
    /// Calls a function repeatedly at a fixed interval in milliseconds.
    pub fn interval(callback: extern "C" fn(), interval_ms: js_uint32) -> Self {
        Web::window_set_timeout(callback, interval_ms) }

    /// Executes JavaScript code immediately.
    /// ## Security Warning
    /// - Avoid passing untrusted input, as this executes arbitrary JS.
    /// - Ensure all evaluated code is **safe and controlled**.
    pub fn eval(js_code: &str) { Web::window_eval(js_code) }
    #[doc = docweb!("Window", "setTimeout")]
    /// Executes JavaScript code after a delay in milliseconds.
    pub fn eval_timeout(js_code: &str, delay_ms: js_uint32) -> Self {
        Web::window_eval_timeout(js_code, delay_ms) }
    #[doc = docweb!("Window", "setInterval")]
    /// Executes JavaScript code repeatedly at a fixed interval in milliseconds.
    pub fn eval_interval(js_code: &str, interval_ms: js_uint32) -> Self {
        Web::window_eval_interval(js_code, interval_ms) }

    #[doc = docweb!("Window", "clearTimeout")]
    #[doc = docweb!("Window", "clearInterval")]
    /// Cancels a timeout or interval.
    pub fn clear(self) { Web::window_clear_timeout(self); }
}
