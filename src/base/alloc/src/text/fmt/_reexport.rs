// devela_base_alloc::text::fmt::_reexport

use crate::{_TAG_CONSTRUCTION, _TAG_FMT, _reexport};

_reexport! { rust: alloc, location: "text/fmt", tag: _TAG_FMT!() _TAG_CONSTRUCTION!(),
    doc: "Creates a String using interpolation of runtime expressions.",
    format
}
