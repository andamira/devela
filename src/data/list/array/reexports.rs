// devela::data::list::array::reexports
//
//! Reexported items.
//

use crate::{TAG_DATA_STRUCTURE, TAG_ITERATOR, reexport};

reexport! { rust: core::array,
    tag: TAG_ITERATOR!(),
    doc: "A by-value [array] iterator.",
    @IntoIter as ArrayIntoIter
}
reexport! { rust: core::array,
    tag: TAG_DATA_STRUCTURE!(),
    doc: "Creates an array `[T; N]`, where each `T` is returned from `cb` from its index.",
    @from_fn as array_from_fn
}
reexport! { rust: core::array,
    tag: TAG_DATA_STRUCTURE!(),
    doc: "Converts a mutable reference to `T` into `&mut [T; 1]` (without copying).",
    @from_mut as array_from_mut
}
reexport! { rust: core::array,
    tag: TAG_DATA_STRUCTURE!(),
    doc: "Converts a reference to `T` into `&[T; 1]` (without copying).",
    @from_ref as array_from_ref
}
