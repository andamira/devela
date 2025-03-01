// devela::lang:js::namespace
//
//! Defines the [`Js`] struct namespace.
//!
//! This API has to be kept in sync with `./web_api.js`.
//
// TOC
// - struct Js
// - impl Web API
//   - canvas
//   - console
//   - time

use devela::js_reexport;

/// a Javascript namespace.
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_ffi")))]
pub struct Js;

/// # Web API canvas
#[rustfmt::skip]
impl Js {
    /* custom */
    /// Sets the active canvas by ID.
    pub fn set_canvas(id: &str) { unsafe { set_canvas(id.as_ptr(), id.len()); } }

    /* draw */

    /// Draws a filled rectangle.
    ///
    /// See: [fillRect].
    ///
    /// [fillRect]: https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect
    pub fn fill_rect(x: f64, y: f64, w: f64, h: f64) { fill_rect(x, y, w, h); }

    ///
    pub fn set_color(r: u8, g: u8, b: u8) { set_color(r, g, b); }

    ///
    pub fn draw_line(x1: f64, y1: f64, x2: f64, y2: f64) { draw_line(x1, y1, x2, y2); }

    ///
    pub fn draw_circle(x: f64, y: f64, radius: f64) { draw_circle(x, y, radius); }

    /* text */

    /// Draws a filled text string at the specified coordinates
    ///
    /// See [fillText].
    ///
    /// [fillText]: https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText
    // https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Drawing_text
    pub fn fill_text(text: &str, x: f64, y: f64) {
        unsafe { fill_text(text.as_ptr(), text.len(), x, y); }
    }
}
js_reexport! {
    [ module: "api_canvas" ]
    /* custom */
    unsafe fn set_canvas(str_ptr: *const u8, str_len: usize);

    /* draw */
    safe fn fill_rect(x: f64, y: f64, w: f64, h: f64);
    safe fn set_color(r: u8, g: u8, b: u8);
    safe fn draw_line(x1: f64, y1: f64, x2: f64, y2: f64);
    safe fn draw_circle(x: f64, y: f64, radius: f64);
    /* text */
    unsafe fn fill_text(str_ptr: *const u8, str_len: usize, x: f64, y: f64);
}

/// # Web API console
#[rustfmt::skip]
impl Js {
    ///
    pub fn console_debug(text: &str) { unsafe { console_debug(text.as_ptr(), text.len()); } }
    ///
    pub fn console_info(text: &str) { unsafe { console_info(text.as_ptr(), text.len()); } }
    ///
    pub fn console_log(text: &str) { unsafe { console_log(text.as_ptr(), text.len()); } }
    ///
    pub fn console_warn(text: &str) { unsafe { console_warn(text.as_ptr(), text.len()); } }
    ///
    pub fn console_error(text: &str) { unsafe { console_error(text.as_ptr(), text.len()); } }
    ///
    pub fn console_group(text: &str) { unsafe { console_group(text.as_ptr(), text.len()); } }
    ///
    pub fn console_group_end() { unsafe { console_group_end(); } }
}
// https://developer.mozilla.org/en-US/docs/Web/API/console
js_reexport! {
    [ module: "api_console" ]
    unsafe fn "console_debug" console_debug(str_ptr: *const u8, str_len: usize);
    unsafe fn "console_info" console_info(str_ptr: *const u8, str_len: usize);
    unsafe fn "console_log" console_log(str_ptr: *const u8, str_len: usize);
    unsafe fn "console_warn" console_warn(str_ptr: *const u8, str_len: usize);
    unsafe fn "console_error" console_error(str_ptr: *const u8, str_len: usize);
    unsafe fn "console_group" console_group(str_ptr: *const u8, str_len: usize);
    unsafe fn "console_group_end" console_group_end();
}

/// # Web API time
#[rustfmt::skip]
impl Js {
    ///
    pub fn get_time() -> f64 { get_time() }
}
js_reexport! {
    [ module: "api_timing" ]
    safe fn get_time() -> f64;
}
