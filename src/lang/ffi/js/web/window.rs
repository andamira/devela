// devela::lang::ffi::js::web::window
//
//! Defines [`WebWindow`].
//

#[cfg(feature = "unsafe_ffi")]
use crate::js_doc;
#[allow(unused_imports, reason = "not(windows)")]
use crate::{JsTimeout, js_uint32, js_reexport};

/// Handle to the global `window` object (singleton).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebWindow;

#[cfg(feature = "unsafe_ffi")] #[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
impl WebWindow {
    // #[doc = js_doc!("Window", "document")]
    // /// Returns the `document` object.
    // pub fn document(&self) -> WebDocument { WebDocument }

    // /// Returns the `localStorage` interface.
    // pub fn local_storage(&self) -> WebLocalStorage { WebLocalStorage }

    #[doc = js_doc!("Window", "setTimeout")]
    /// Calls a function after a delay in milliseconds.
    pub fn set_timeout(callback: extern "C" fn(), delay_ms: js_uint32) -> JsTimeout {
        JsTimeout { id: unsafe { window_set_timeout(callback as usize, delay_ms) } }
    }
    #[doc = js_doc!("Window", "setInterval")]
    /// Calls a function repeatedly at a fixed interval in milliseconds.
    pub fn set_interval(callback: extern "C" fn(), interval_ms: js_uint32) -> JsTimeout {
        JsTimeout { id: unsafe { window_set_interval(callback as usize, interval_ms) } }
    }
    #[doc = js_doc!("Window", "clearTimeout")]
    #[doc = js_doc!("Window", "clearInterval")]
    /// Cancels a timeout or interval.
    pub fn clear_timeout(id: JsTimeout) { window_clear_timeout(id.id); }

    /// Executes JavaScript code immediately.
    /// ## Security Warning
    /// - Avoid passing untrusted input, as this executes arbitrary JS.
    /// - Ensure all evaluated code is **safe and controlled**.
    pub fn eval(js_code: &str) { unsafe { window_eval(js_code.as_ptr(), js_code.len()); } }
    #[doc = js_doc!("Window", "setTimeout")]
    /// Executes JavaScript code after a delay in milliseconds.
    pub fn eval_timeout(js_code: &str, delay_ms: js_uint32) -> JsTimeout { JsTimeout {
        id: unsafe { window_eval_timeout(js_code.as_ptr(), js_code.len(), delay_ms) } } }
    #[doc = js_doc!("Window", "setInterval")]
    /// Executes JavaScript code repeatedly at a fixed interval in milliseconds.
    pub fn eval_interval(js_code: &str, interval_ms: js_uint32) -> JsTimeout { JsTimeout {
        id: unsafe { window_eval_interval(js_code.as_ptr(), js_code.len(), interval_ms) } } }

    #[doc = js_doc!("Window", "requestAnimationFrame")]
    /// Requests an animation frame, executing the given `callback`.
    pub fn request_animation_frame(callback: extern "C" fn()) -> js_uint32 {
        unsafe { window_request_animation_frame(callback as usize) } }
    /// Cancels a request for an animation frame.
    pub fn cancel_animation_frame(id: js_uint32) { window_cancel_animation_frame(id); }
}
js_reexport! { [module: "api_window"]
    unsafe fn window_set_timeout(callback_ptr: usize, delay_ms: js_uint32) -> js_uint32;
    unsafe fn window_set_interval(callback_ptr: usize, interval_ms: js_uint32) -> js_uint32;
    safe fn window_clear_timeout(timeout_id: js_uint32);
    //
    unsafe fn window_eval(js_code_ptr: *const u8, js_code_len: usize);
    unsafe fn window_eval_timeout(js_code_ptr: *const u8, js_code_len: usize, delay_ms: js_uint32)
        -> js_uint32;
    unsafe fn window_eval_interval(js_code_ptr: *const u8, js_code_len: usize,
        interval_ms: js_uint32) -> js_uint32;
    //
    unsafe fn window_request_animation_frame(callback_ptr: usize) -> js_uint32;
    safe fn window_cancel_animation_frame(requestId: js_uint32);
}
