// devela::sys::os::browser::web::api::namespace
// (in sync with ./web_api.js)
//
//! Defines the [`Web`] namespace.
//

use crate::_js_extern;
#[cfg(feature = "unsafe_ffi")]
#[cfg(not(feature = "safe_lang"))]
crate::items! {
    use crate::{WebDocument, WebPermission, WebPermissionState, WebWindow};
    use crate::{_js_doc, js_int32, js_number, JsInstant};
    #[cfg(feature = "event")]
    use crate::{WebEventKind, js_uint32};
}

#[doc = crate::_tags!(web namespace)]
/// A Web API namespace.
#[doc = crate::_doc_location!("sys/os/browser/web")]
///
/// # Features
/// All methods depend on the `unsafe_ffi` feature and the `wasm32` architecture.
///
/// # Methods
/// - core APis
///   - [console](#web-api-console)
//    - document
///   - [events](#web-api-events)
///   - [history & location](#web-api-history--location)
///   - [permissions](#web-api-permissions)
//   - [url](#web-api-url--urlsearchparams)
///   - [window](#web-api-window)
/// - extended APis
///   - media & graphics
///     - [canvas](#web-api-canvas)
///  - performance & optimization
///    - [performance](#web-api-performance)
///  - advanced & experimental
///    - [workers](#web-api-workers)
#[derive(Clone, Copy, Debug)]
pub struct Web;

#[rustfmt::skip]
#[cfg(not(feature = "safe_lang"))]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
impl Web {
    #[doc = _js_doc!("Window")]
    #[doc = _js_doc!("Screen")]
    /// Returns the handle to the browser's global [Window] and [Screen] associated APIs.
    ///
    /// [Window]: https://developer.mozilla.org/en-US/docs/Web/API/Window
    /// [Screen]: https://developer.mozilla.org/en-US/docs/Web/API/Window/screen
    pub fn window() -> WebWindow { WebWindow }

    #[doc = _js_doc!("Document")]
    /// Handle to the brower's global [Document] associated APIs.
    ///
    /// [Document]: https://developer.mozilla.org/en-US/docs/Web/API/Document
    pub fn document() -> WebDocument { WebDocument }
}

/// # Web API permissions
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API>
#[rustfmt::skip]
#[cfg(not(feature = "safe_lang"))]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
impl Web {
    #[doc = _js_doc!("Permissions", "query")]
    /// Queries the status of a given permission.
    ///
    /// Returns `Granted`, `Denied`, `Prompt`, or `Unknown` if unsupported.
    pub fn permissions_query(permission: WebPermission) -> WebPermissionState {
        unsafe { permissions_query(permission.as_str().as_ptr(), permission.as_str().len()) }
        .into()
    }
}
_js_extern! {
    [ module: "api_permissions" ]
    unsafe fn permissions_query(name_ptr: *const u8, name_len: usize) -> js_int32;
}

/* methods: extended APIs */

/// # Web API performance
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Performance>
#[rustfmt::skip]
#[cfg(not(feature = "safe_lang"))]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
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
