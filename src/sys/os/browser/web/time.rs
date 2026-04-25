// devela::sys::os::browser::web::time

use crate::{_js_doc, JsInstant, JsTimeout, js_uint32};
use crate::{TimeDelta, Web, WebWindow};

#[rustfmt::skip]
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
    pub fn delta_elapsed(self) -> TimeDelta { TimeDelta::from_js(self.elapsed()) }
}

#[rustfmt::skip]
impl JsTimeout {
    #[doc = _js_doc!("Window", "setTimeout")]
    /// Calls a function after a delay in milliseconds.
    pub fn timeout(callback: extern "C" fn(), delay_ms: js_uint32) -> Self {
        WebWindow::set_timeout(callback, delay_ms) }
    #[doc = _js_doc!("Window", "setInterval")]
    /// Calls a function repeatedly at a fixed interval in milliseconds.
    pub fn interval(callback: extern "C" fn(), interval_ms: js_uint32) -> Self {
        WebWindow::set_timeout(callback, interval_ms) }

    /// Executes JavaScript code immediately.
    /// ## Security Warning
    /// - Avoid passing untrusted input, as this executes arbitrary JS.
    /// - Ensure all evaluated code is **safe and controlled**.
    pub fn eval(js_code: &str) { WebWindow::eval(js_code) }
    #[doc = _js_doc!("Window", "setTimeout")]
    /// Executes JavaScript code after a delay in milliseconds.
    pub fn eval_timeout(js_code: &str, delay_ms: js_uint32) -> Self {
        WebWindow::eval_timeout(js_code, delay_ms) }
    #[doc = _js_doc!("Window", "setInterval")]
    /// Executes JavaScript code repeatedly at a fixed interval in milliseconds.
    pub fn eval_interval(js_code: &str, interval_ms: js_uint32) -> Self {
        WebWindow::eval_interval(js_code, interval_ms) }

    #[doc = _js_doc!("Window", "clearTimeout")]
    #[doc = _js_doc!("Window", "clearInterval")]
    /// Cancels a timeout or interval.
    pub fn clear(self) { WebWindow::clear_timeout(self); }
}
