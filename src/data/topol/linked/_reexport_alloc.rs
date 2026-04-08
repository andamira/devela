// devela::data::topol::linked::_reexport_alloc

use crate::{_reexport, _tags};

_reexport! { rust: alloc::collections, location: "data/topol", tag: _tags!(data_structure),
    doc: "A doubly-linked list with owned nodes.",
    LinkedList
}
