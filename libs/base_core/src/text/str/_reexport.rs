// devela_base_core::text::str::_reexport
//
//! String related re-exports.
//

#![allow(unused_imports)]

use crate::{_TAG_LIFETIME, _TAG_TEXT, _reexport};

_reexport! { rust: core::str,
    tag: _TAG_TEXT!() _TAG_LIFETIME!(),
    doc: "Parse a value from a string.",
    FromStr
}
