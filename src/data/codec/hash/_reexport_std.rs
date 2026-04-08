// devela::data::codec::hash::_reexport_std

use crate::{_reexport, _tags};

_reexport! { rust: std::hash, location: "data/hash", tag: _tags!(hash),
    doc: "The default state for [`HashMapStd`][crate::HashMapStd].",
    RandomState
}
