// devela::lang::ffi::js::web::window
//
//! Defines [`WebWindow`].
//

#[cfg(feature = "metric")]
use crate::Extent2d;
#[cfg(feature = "unsafe_ffi")]
use crate::js_doc;
#[allow(unused_imports, reason = "not(windows)")]
use crate::{Js, JsTimeout, js_bool, js_int32, js_number, js_reexport, js_uint32};
#[cfg(feature = "alloc")]
use devela::String;

/// Handle to the global `window` object (singleton).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebWindow;

#[cfg(feature = "unsafe_ffi")] #[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
impl WebWindow {
    #[doc = js_doc!("Window", "closed")]
    /// Indicates whether the current window is closed or not.
    pub fn is_closed() -> js_bool { window_is_closed() }

    #[doc = js_doc!("Window", "crossOriginIsolated")]
    /// Indicates whether the website is in a cross-origin isolation state.
    pub fn is_cross_origin_isolated() -> js_bool { window_is_cross_origin_isolated() }

    #[doc = js_doc!("Window", "isSecureContext")]
    /// Indicates whether the current [context is secure][0].
    ///
    /// [0]: https://developer.mozilla.org/en-US/docs/Web/Security/Secure_Contexts
    pub fn is_secure_context() -> js_bool { window_is_secure_context() }

    /* texts */

    #[doc = js_doc!("Window", "name")]
    /// Returns the window name as a `String`.
    #[cfg(feature = "alloc")] #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn name() -> String { Js::read_string(|ptr, len| unsafe { window_name(ptr, len) }) }
    #[doc = js_doc!("Window", "name")]
    /// Writes the current window name into the provided buffer and returns it as a string slice.
    pub fn name_buf(buffer: &mut [u8]) -> &str {
        Js::read_str(buffer, |ptr, len| unsafe { window_name(ptr, len) }) }
    #[doc = js_doc!("Window", "name")]
    /// Sets the current window `name`.
    pub fn set_name(name: &str) { unsafe { window_set_name(name.as_ptr(), name.len() as u32); } }

    /* metrics */

    #[doc = js_doc!("Window", "devicePixelRatio")]
    /// Returns the ratio of the resolution in physical pixels to the resolution in CSS pixels.
    pub fn device_pixel_ratio() -> js_number { window_device_pixel_ratio() }
    #[doc = js_doc!("Window", "innerWidth")]
    #[doc = js_doc!("Window", "innerHeight")]
    /// The extent of the content area of the browser window including any rendered scrollbars.
    #[cfg(feature = "metric")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "metric")))]
    pub fn inner_size() -> Extent2d<js_number> {
        let mut extent = Extent2d { size: [0.0; 2] };
        unsafe { window_inner_size(extent.size.as_mut_ptr()) };
        extent
    }
    #[doc = js_doc!("Window", "innerWidth")]
    /// The width of the content area of the browser window including the vertical scrollbar.
    pub fn inner_width() -> js_number { window_inner_width() }
    #[doc = js_doc!("Window", "innerHeight")]
    /// The height of the content area of the browser window including the horizontal scrollbar.
    pub fn inner_height() -> js_number { window_inner_height() }

    /* timeout */

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

    /* eval */

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

    /* animation */

    #[doc = js_doc!("Window", "requestAnimationFrame")]
    /// Requests an animation frame, executing the given `callback`.
    pub fn request_animation_frame(callback: extern "C" fn()) -> js_uint32 {
        unsafe { window_request_animation_frame(callback as usize) } }
    /// Cancels a request for an animation frame.
    pub fn cancel_animation_frame(id: js_uint32) { window_cancel_animation_frame(id); }
}
js_reexport! {
    [module: "api_window"]
    // misc
    safe fn window_is_closed() -> js_bool;
    safe fn window_is_cross_origin_isolated() -> js_bool;
    safe fn window_is_secure_context() -> js_bool;
    // texts
    unsafe fn window_name(buf_ptr: *mut u8, max_len: js_uint32) -> js_int32;
    unsafe fn window_set_name(str_ptr: *const u8, str_len: js_uint32);
    // metrics
    safe fn window_device_pixel_ratio() -> js_number;
    #[cfg(feature = "metric")]
    unsafe fn window_inner_size(data: *mut js_number);
    safe fn window_inner_width() -> js_number;
    safe fn window_inner_height() -> js_number;
    // timeout
    unsafe fn window_set_timeout(callback_ptr: usize, delay_ms: js_uint32) -> js_uint32;
    unsafe fn window_set_interval(callback_ptr: usize, interval_ms: js_uint32) -> js_uint32;
    safe fn window_clear_timeout(timeout_id: js_uint32);
    // eval
    unsafe fn window_eval(js_code_ptr: *const u8, js_code_len: usize);
    unsafe fn window_eval_timeout(js_code_ptr: *const u8, js_code_len: usize, delay_ms: js_uint32)
        -> js_uint32;
    unsafe fn window_eval_interval(js_code_ptr: *const u8, js_code_len: usize,
        interval_ms: js_uint32) -> js_uint32;
    // animation
    unsafe fn window_request_animation_frame(callback_ptr: usize) -> js_uint32;
    safe fn window_cancel_animation_frame(requestId: js_uint32);
}
