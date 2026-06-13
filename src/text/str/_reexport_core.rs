// devela/src/text/str/_reexport_core.rs

#![allow(unused_imports)]

use crate::{_reexport, _tags};

_reexport! { rust: core::str, location: "text/str", tag: _tags!(text lifetime),
    doc: "Parse a value from a string.",
    FromStr
}
