// devela/src/text/fmt/_reexport_alloc.rs

use crate::{_reexport, _tags};

_reexport! { rust: alloc, location: "text/fmt", tag: _tags!(fmt construction),
    doc: "Creates a String using interpolation of runtime expressions.",
    format
}
