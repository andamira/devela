// devela_base_alloc::data::list::link::_reexport

use crate::{_TAG_DATA_STRUCTURE, _reexport};

_reexport! { rust: alloc::collections, location: "data/list", tag: _TAG_DATA_STRUCTURE!(),
    doc: "A doubly-linked list with owned nodes.",
    LinkedList
}
