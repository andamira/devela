// devela::text::fmt::_reexport_alloc

use crate::{_reexport, _tags};

_reexport! { rust: alloc, location: "text/fmt", tag: _tags!(fmt construction),
    doc: "Creates a String using interpolation of runtime expressions.",
    format
}
