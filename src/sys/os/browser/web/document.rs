// devela::sys::os::browser::web::document
//
//! Defines [`WebDocument`].
//

use devela::_js_doc;
#[cfg(feature = "alloc")]
use devela::String;
// #[allow(unused_imports, reason = "not(windows)")]
use devela::{_js_extern, _js_method_str_alloc, WebElement, js_bool, js_int32, js_uint32};

#[doc = crate::_TAG_WEB!()]
/// Handle to the brower's global [Document] associated APIs.
#[doc = crate::_doc_location!("sys/os/browser/web")]
///
/// [Document]: https://developer.mozilla.org/en-US/docs/Web/API/Document
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WebDocument;

#[rustfmt::skip]
#[cfg(not(feature = "safe_lang"))]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
impl WebDocument {
    /* flags */

    #[doc = _js_doc!("Document", "compatMode")]
    /// Returns `true` if the browser is in no-quirks mode, ("CSS1Compat") or `false` otherwise.
    ///
    /// - <https://developer.mozilla.org/en-US/docs/Web/HTML/Guides/Quirks_mode_and_standards_mode>
    pub fn is_compat_mode() -> js_bool { document_is_compat_mode() }

    #[doc = _js_doc!("Document", "hidden")]
    /// Whether the current document is hidden.
    pub fn is_hidden() -> js_bool { document_is_hidden() }

    /* */

    _js_method_str_alloc! {
        #[doc = _js_doc!("Document", "contentType")]
        /// Returns the document's content type.
        content_type, document_content_type
    }
}
_js_extern! {
    [module: "api_document"]
    safe fn document_is_compat_mode() -> js_bool;
    safe fn document_is_hidden() -> js_bool;
    unsafe fn document_content_type(ptr: *mut u8, max_len: js_uint32) -> js_int32;
}
