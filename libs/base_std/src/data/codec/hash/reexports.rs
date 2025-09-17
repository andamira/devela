// devela_base_std::data::codec::hash::reexports
//
//!
//

use crate::{_TAG_HASH, _reexport};

_reexport! { rust: std::hash,
    tag: _TAG_HASH!(),
    doc: "The default state for [`HashMapStd`][crate::HashMapStd].",
    RandomState
}
