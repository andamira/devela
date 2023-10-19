// devela::num::reexports
//
//! Reexported items from `core`.
//

use crate::codegen::reexport;

reexport! { rust: core::num, local_module: "num",
    doc: "A signed integer that is known not to equal zero.",
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize
}

reexport! { rust: core::num, local_module: "num",
    doc: "An unsigned integer that is known not to equal zero.",
    NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize
}
