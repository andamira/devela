// devela_base_alloc::text::fmt::reexports
//
//!
//

use crate::{_reexport, TAG_FMT};

_reexport! { rust: alloc,
    tag: TAG_FMT!(),
    doc: "Creates a String using interpolation of runtime expressions.",
    format
}
