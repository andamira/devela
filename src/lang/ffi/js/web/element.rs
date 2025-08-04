// devela::lang::ffi::js::web::element
//
//! Defines [`WebElement`], [`WebElementIter`].
//

use devela::_js_doc;
#[cfg(feature = "alloc")]
use devela::String;
#[allow(unused_imports, reason = "not(windows)")]
use devela::{_js_extern, _js_method_str_alloc, Js, js_bool, js_int32, js_uint32};

/// Handle to a DOM [`Element`].
///
/// [Element]: https://developer.mozilla.org/en-US/docs/Web/API/Element
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebElement {
    pub(in crate::lang::ffi::js) id: js_uint32,
}

impl WebElement {
}
_js_extern! {
    [module: "api_element"]
}
