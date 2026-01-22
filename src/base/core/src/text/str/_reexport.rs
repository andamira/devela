// devela_base_core::text::str::_reexport

#![allow(unused_imports)]

use crate::{_TAG_LIFETIME, _TAG_TEXT, _reexport};

_reexport! { rust: core::str, location: "text/str", tag: _TAG_TEXT!() _TAG_LIFETIME!(),
    doc: "Parse a value from a string.",
    FromStr
}
