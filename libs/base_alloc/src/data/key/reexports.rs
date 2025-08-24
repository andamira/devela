// devela_alloc::data::key::reexports

use crate::_reexport;

/* from `alloc` */

_reexport! { rust: alloc::collections,
    tag: crate::TAG_DATA_STRUCTURE!(),
    doc: "An ordered map based on a B-Tree.",
    BTreeMap
}
_reexport! { rust: alloc::collections::btree_map,
    doc: "An entry of an ordered map based on a B-Tree.",
    @Entry as BTreeMapEntry
}
_reexport! { rust: alloc::collections,
    tag: crate::TAG_DATA_STRUCTURE!(),
    doc: "An ordered set based on a B-Tree.",
    BTreeSet
}

