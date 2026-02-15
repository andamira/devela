// devela_alloc::data::id::key::_reexport

use crate::{_TAG_DATA_STRUCTURE, _reexport};

/* from `alloc` */

_reexport! { rust: alloc::collections, location: "data/id", tag: _TAG_DATA_STRUCTURE!(),
    doc: "An ordered map based on a B-Tree.",
    BTreeMap
}
_reexport! { rust: alloc::collections::btree_map, location: "data/id",
    doc: "An entry of an ordered map based on a B-Tree.",
    @Entry as BTreeMapEntry
}
_reexport! { rust: alloc::collections, location: "data/id", tag: _TAG_DATA_STRUCTURE!(),
    doc: "An ordered set based on a B-Tree.",
    BTreeSet
}
