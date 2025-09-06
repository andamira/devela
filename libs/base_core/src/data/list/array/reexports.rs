// devela_base_core::data::list::array::reexports

use crate::{_reexport, TAG_DATA_STRUCTURE, TAG_ITERATOR};

_reexport! { rust: core::array,
    tag: TAG_ITERATOR!(),
    doc: "A by-value [array] iterator.",
    @IntoIter as ArrayIntoIter
}
_reexport! { rust: core::array,
    tag: TAG_DATA_STRUCTURE!(),
    doc: "Creates an array `[T; N]`, where each `T` is returned from `cb` from its index.",
    @from_fn as array_from_fn
}
_reexport! { rust: core::array,
    tag: TAG_DATA_STRUCTURE!(),
    doc: "Converts a mutable reference to `T` into `&mut [T; 1]` (without copying).",
    @from_mut as array_from_mut
}
_reexport! { rust: core::array,
    tag: TAG_DATA_STRUCTURE!(),
    doc: "Converts a reference to `T` into `&[T; 1]` (without copying).",
    @from_ref as array_from_ref
}
