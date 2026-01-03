// devela_base_std::data::codec::hash::_reexport
//
//!
//

use crate::{_TAG_HASH, _reexport};

_reexport! { rust: std::hash, location: "data/hash", tag: _TAG_HASH!(),
    doc: "The default state for [`HashMapStd`][crate::HashMapStd].",
    RandomState
}
