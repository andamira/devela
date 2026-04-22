// devela::sys::os::browser::web::api::canvas
// (in sync with ./web_api.js)
//
//! Implements the web canvas API.
//

use crate::Web;
use crate::{_js_doc, _js_extern, JsTextMetrics, JsTextMetricsFull, js_number};

/// # Web API canvas
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D>
/// - <https://html.spec.whatwg.org/multipage/canvas.html>
#[rustfmt::skip]
impl Web {
    /* misc. */
    /// Sets the active canvas using a CSS `selector`.
    pub fn set_canvas(selector: &str) { unsafe { set_canvas(selector.as_ptr(), selector.len()); } }

    /* color settings */
    #[doc = _js_doc!(canvas "fillStyle")]
    /// Sets the color or style for filling shapes.
    pub fn fill_style(r: u8, g: u8, b: u8) { fill_style(r, g, b); }
    #[doc = _js_doc!(canvas "strokeStyle")]
    /// Sets the color or style for lines.
    pub fn stroke_style(r: u8, g: u8, b: u8) { stroke_style(r, g, b); }

    /* drawing rectangles */
    #[doc = _js_doc!(canvas "fillRect")]
    /// Draws a filled rectangle.
    pub fn fill_rect(x: js_number, y: js_number, w: js_number, h: js_number)
        { fill_rect(x, y, w, h); }
    #[doc = _js_doc!(canvas "strokeRect")]
    /// Draws a rectangular outline.
    pub fn stroke_rect(x: js_number, y: js_number, w: js_number, h: js_number)
        { stroke_rect(x, y, w, h); }
    #[doc = _js_doc!(canvas "clearRect")]
    /// Clears the specified rectangular area, making it fully transparent.
    pub fn clear_rect(x: js_number, y: js_number, w: js_number, h: js_number)
        { clear_rect(x, y, w, h); }

    /* drawing shapes */
    /// Draws a line.
    pub fn draw_line(x1: js_number, y1: js_number, x2: js_number, y2: js_number)
        { draw_line(x1, y1, x2, y2); }
    /// Draws a circle.
    pub fn draw_circle(x: js_number, y: js_number, radius: js_number)
        { draw_circle(x, y, radius); }

    /* drawing text */
    #[doc = _js_doc!(canvas "fillText")]
    /// Draws filled text at the specified position.
    pub fn fill_text(text: &str, x: js_number, y: js_number) {
        unsafe { fill_text(text.as_ptr(), text.len(), x, y); }
    }
    #[doc = _js_doc!(canvas "strokeText")]
    /// Draws text outline at the specified position.
    pub fn stroke_text(text: &str, x: js_number, y: js_number) {
        unsafe { stroke_text(text.as_ptr(), text.len(), x, y); }
    }
    #[doc = _js_doc!(canvas "measureText")]
    /// Measures the essential properties of text.
    pub fn measure_text(text: &str) -> JsTextMetrics {
        let (mut metrics, ptr, len) = (JsTextMetrics::default(), text.as_ptr(), text.len());
        unsafe { measure_text(ptr, len, &mut metrics as *mut JsTextMetrics); }
        metrics
    }
    #[doc = _js_doc!(canvas "measureTextFull")]
    /// Measures all available text metrics.
    pub fn measure_text_full(text: &str) -> JsTextMetricsFull {
        let (mut metrics, ptr, len) = (JsTextMetricsFull::default(), text.as_ptr(), text.len());
        unsafe { measure_text_full(ptr, len, &mut metrics as *mut JsTextMetricsFull); }
        metrics
    }
}
_js_extern! {
    [ module: "api_canvas" ]
    /* misc. */
    unsafe fn set_canvas(str_ptr: *const u8, str_len: usize);
    /* color settings */
    safe fn "fillStyle" fill_style(r: u8, g: u8, b: u8);
    safe fn "strokeStyle" stroke_style(r: u8, g: u8, b: u8);
    /* drawing rectangles */
    safe fn "fillRect" fill_rect(x: js_number, y: js_number, w: js_number, h: js_number);
    safe fn "strokeRect" stroke_rect(x: js_number, y: js_number, w: js_number, h: js_number);
    safe fn "clearRect" clear_rect(x: js_number, y: js_number, w: js_number, h: js_number);
    /* drawing paths */
    //
    /* drawing shapes */
    safe fn draw_line(x1: js_number, y1: js_number, x2: js_number, y2: js_number);
    safe fn draw_circle(x: js_number, y: js_number, radius: js_number);

    /* drawing text */
    unsafe fn "fillText" fill_text(str_ptr: *const u8, str_len: usize, x: js_number, y: js_number);
    unsafe fn "strokeText" stroke_text(str_ptr: *const u8, str_len: usize,
        x: js_number, y: js_number);
    unsafe fn "measureText" measure_text(text_ptr: *const u8, text_len: usize,
        out_metrics: *mut JsTextMetrics);
    unsafe fn "measureTextFull" measure_text_full(text_ptr: *const u8, text_len: usize,
        out_metrics: *mut JsTextMetricsFull);
}
