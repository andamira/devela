// devela/src/sys/os/browser/web/bridge/performance.rs
// In sync with js/performance.js
//
//! Implements the web performance API.
//

use crate::{_js_doc, _js_extern, JsInstant, Web, js_number};
#[cfg(feature = "event")]
use crate::{WebEventKind, js_uint32};

/// # Web API performance
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Performance>
#[rustfmt::skip]
impl Web {
    #[doc = _js_doc!("Performance", "now")]
    /// Retrieves a high-resolution timestamp in milliseconds.
    pub fn performance_now() -> JsInstant { JsInstant::from_millis_f64(performance_now()) }
    #[doc = _js_doc!("Performance", "timeOrigin")]
    /// Retrieves the time origin in milliseconds.
    pub fn performance_time_origin() -> JsInstant {
        JsInstant::from_millis_f64(performance_time_origin()) }
    #[doc = _js_doc!("Performance", "eventCounts")]
    /// Retrieves the count of recorded events.
    #[cfg(feature = "event")]
    pub fn performance_event_count(event: WebEventKind) -> js_uint32 {
        let name = event.as_str();
        unsafe { performance_event_count(name.as_ptr(), name.len()) }
    }
}
_js_extern! {
    [ module: "api_performance" ]
    safe fn "now" performance_now() -> js_number;
    safe fn "timeOrigin" performance_time_origin() -> js_number;
    #[cfg(feature = "event")]
    unsafe fn "eventCounts" performance_event_count(event_ptr: *const u8, event_len: usize)
        -> js_uint32;
}
