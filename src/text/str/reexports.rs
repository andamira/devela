// devela::text::str:reexports
//
//! String related re-exports.
//

use crate::{_reexport_from, impl_cdef};

// from workspace base
_reexport_from!("../../../libs/base/src/text/str/reexports.rs", _c);
_reexport_from!(alloc "../../../libs/base_alloc/src/text/str/reexports.rs", _a);
_reexport_from!(std "../../../libs/base_std/src/text/str/reexports.rs", _s);

/* from other modules */

pub use crate::CStr;
#[cfg(feature = "alloc")]
pub use crate::CString;

/* impl ConstDefault */

impl_cdef!["" => &str];
#[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
impl crate::ConstDefault for &mut str {
    // SAFETY: The empty string is valid UTF-8.
    const DEFAULT: Self = unsafe { ::core::str::from_utf8_unchecked_mut(&mut []) };
}
#[cfg(feature = "alloc")]
impl_cdef![Self::new() => String];
