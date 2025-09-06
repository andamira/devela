// devela_base_core::text::str:reexports
//
//! String related re-exports.
//

#![allow(unused_imports)]

use crate::{_reexport, TAG_TEXT};

_reexport! { rust: core::str,
    tag: TAG_TEXT!(),
    doc: "Parse a value from a string.",
    FromStr
}
