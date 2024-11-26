// devela::num::float::reexports
//
//! Reexported items from `core`.
//

use crate::reexport;

reexport! { rust: core::num,
    doc: "A classification of floating point numbers.",
    @FpCategory as FloatCategory
}
