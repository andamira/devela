// devela::text::str:reexports
//
//! String related re-exports.
//

crate::mod_path!(+pub _c "../../../libs/base/src/text/str/reexports.rs");
crate::mod_path!(alloc +pub _a "../../../libs/base_alloc/src/text/str/reexports.rs");
crate::mod_path!(std +pub _s "../../../libs/base_std/src/text/str/reexports.rs");

pub use devela_base::text::str::{Str, StringU8, StringU16, StringU32, StringUsize};

/* from other modules */

pub use crate::CStr;
#[cfg(feature = "alloc")]
pub use crate::CString;
