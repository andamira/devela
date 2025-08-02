// devela::lang::ffi::js::web::document
//
//! Defines [`WebDocument`], [`WebElement`], [`WebElementIter`].
//

#[cfg(feature = "alloc")]
use devela::String;
use devela::js_doc;
use devela::{Backing, MaybeOwned, Ownership};
#[allow(unused_imports, reason = "not(windows)")]
use devela::{Js, js_bool, js_int32, js_reexport, js_uint32};

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
    pub fn content_type<'a>(buffer: Backing<'a>) -> MaybeOwned<'a, str> {
        match buffer {
            Backing::Buf(b) => {
                let s = Js::read_str(b, |ptr, len| unsafe { document_content_type(ptr, len) });
                MaybeOwned::Borrowed(s)
            }
            #[cfg(feature = "alloc")]
            Backing::Alloc => {
                let s = Js::read_string(|ptr, len| unsafe { document_content_type(ptr, len)});
                MaybeOwned::Owned(s)
            }
        }
    }
}
js_reexport! {
    [module: "api_document"]
    safe fn document_is_compat_mode() -> js_bool;
    safe fn document_is_hidden() -> js_bool;
    unsafe fn document_content_type(ptr: *mut u8, max_len: js_uint32) -> js_int32;
}
