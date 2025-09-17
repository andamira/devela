// devela_base_core::text::str:reexports
//
//! String related re-exports.
//

#![allow(unused_imports)]

use crate::{_TAG_TEXT, _reexport};

_reexport! { rust: core::str,
    tag: _TAG_TEXT!(),
    doc: "Parse a value from a string.",
    FromStr
}
