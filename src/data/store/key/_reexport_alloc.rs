// devela::data::store::key::_reexport_alloc

use crate::{_reexport, _tags};

/* from `alloc` */

_reexport! { rust: alloc::collections, location: "data/id", tag: _tags!(data_structure),
    doc: "An ordered map based on a B-Tree.",
    BTreeMap
}
_reexport! { rust: alloc::collections::btree_map, location: "data/id",
    doc: "An entry of an ordered map based on a B-Tree.",
    @Entry as BTreeMapEntry
}
_reexport! { rust: alloc::collections, location: "data/id", tag: _tags!(data_structure),
    doc: "An ordered set based on a B-Tree.",
    BTreeSet
}
