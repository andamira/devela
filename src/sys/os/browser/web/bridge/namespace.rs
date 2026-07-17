// devela/src/sys/os/browser/web/bridge/namespace.rs
// In sync with js/*.js
//
//! Defines the [`Web`] namespace.
//

use crate::{_js_doc, WebDocument, WebWindow};

#[doc = crate::_tags!(web namespace)]
/// A Web API namespace.
#[doc = crate::_doc_meta!{location("sys/os/browser/web")}]
///
/// # Methods
/// - core APis
//   - [console](#web-api-console)
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
///
/// See also: [`JsConsole`][crate::JsConsole].
#[derive(Clone, Copy, Debug)]
pub struct Web;

#[rustfmt::skip]
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
