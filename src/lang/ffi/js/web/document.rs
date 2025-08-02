// devela::lang::ffi::js::web::document
//
//! Defines [`WebDocument`], [`WebElement`], [`WebElementIter`].
//

#[cfg(feature = "alloc")]
use crate::String;
use crate::js_doc;
#[allow(unused_imports, reason = "not(windows)")]
use crate::{Js, js_bool, js_int32, js_reexport, js_uint32};

/// Handle to the brower's global [Document] associated APIs.
///
/// [Document]: https://developer.mozilla.org/en-US/docs/Web/API/Document
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WebDocument;

#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
impl WebDocument {
    /* flags */

    #[doc = js_doc!("Document", "compatMode")]
    /// Returns `true` if the browser is in no-quirks mode, ("CSS1Compat") or `false` otherwise.
    ///
    /// - <https://developer.mozilla.org/en-US/docs/Web/HTML/Guides/Quirks_mode_and_standards_mode>
    pub fn is_compat_mode() -> js_bool { document_is_compat_mode() }

    #[doc = js_doc!("Document", "hidden")]
    /// Whether the current document is hidden.
    pub fn is_hidden() -> js_bool { document_is_hidden() }

    /* */

    #[doc = js_doc!("Document", "contentType")]
    /// Returns the document's content type.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn content_type() -> String {
        Js::read_string(|ptr, len| unsafe { document_content_type(ptr, len) })
    }

    #[doc = js_doc!("Document", "contentType")]
    /// Returns the document's content type using the provided buffer.
    pub fn content_type_buf(buffer: &mut [u8]) -> &str {
        Js::read_str(buffer, |ptr, len| unsafe { document_content_type(ptr, len) })
    }
}
js_reexport! {
    [module: "api_document"]
    safe fn document_is_compat_mode() -> js_bool;
    safe fn document_is_hidden() -> js_bool;
    unsafe fn document_content_type(ptr: *mut u8, max_len: js_uint32) -> js_int32;
}
