// devela_base_std::data::codec::hash::reexports
//
//!
//

use crate::{_reexport, TAG_HASH};

_reexport! { rust: std::hash,
    tag: TAG_HASH!(),
    doc: "The default state for [`HashMapStd`][crate::HashMapStd].",
    RandomState
}
