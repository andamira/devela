// devela::sys::os::browser::web::element
//
//! Defines [`WebElement`], [`WebElementIter`].
//

// use devela::_js_doc;
// #[cfg(feature = "alloc")]
// use devela::String;
#[allow(unused_imports, reason = "not(windows)")]
use devela::{_js_extern, _js_method_str_alloc, Js, WebDocument, js_bool, js_int32, js_uint32};

#[doc = crate::_TAG_WEB!()]
#[doc = crate::_TAG_UID!()]
/// Handle to a DOM [Element].
#[doc = crate::_doc_location!("sys/os/browser/web")]
///
/// [Element]: https://developer.mozilla.org/en-US/docs/Web/API/Element
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebElement {
    pub(crate) id: js_uint32
}

#[rustfmt::skip]
#[cfg(not(feature = "safe_lang"))]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
impl WebElement {
}
_js_extern! {
    [module: "api_element"]
}
