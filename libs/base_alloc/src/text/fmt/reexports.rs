// devela_base_alloc::text::fmt::reexports
//
//!
//

use crate::{_TAG_FMT, _reexport};

_reexport! { rust: alloc,
    tag: _TAG_FMT!(),
    doc: "Creates a String using interpolation of runtime expressions.",
    format
}
