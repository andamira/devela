// devela/src/data/codec/hash/_reexport_std.rs

use crate::{_reexport, _tags};

_reexport! { rust: std::hash,
    location: "data/codec/hash" => struct RandomState, tag: _tags!(hash),
    doc: "The default state for [`HashMapStd`][crate::HashMapStd].",
    RandomState
}
