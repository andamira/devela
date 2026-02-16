// devela_base_core::data::layout::array::_reexport

use crate::{_TAG_DATA_STRUCTURE, _TAG_ITERATOR, _reexport};

_reexport! { rust: core::array, location: "data/layout/array", tag: _TAG_ITERATOR!(),
    doc: "A by-value [array] iterator.",
    @IntoIter as ArrayIntoIter
}
_reexport! { rust: core::array, location: "data/layout/array", tag: _TAG_DATA_STRUCTURE!(),
    doc: "Creates an array `[T; N]`, where each `T` is returned from `cb` from its index.",
    @from_fn as array_from_fn
}
_reexport! { rust: core::array, location: "data/layout/array", tag: _TAG_DATA_STRUCTURE!(),
    doc: "Converts a mutable reference to `T` into `&mut [T; 1]` (without copying).",
    @from_mut as array_from_mut
}
_reexport! { rust: core::array, location: "data/layout/array", tag: _TAG_DATA_STRUCTURE!(),
    doc: "Converts a reference to `T` into `&[T; 1]` (without copying).",
    @from_ref as array_from_ref
}
