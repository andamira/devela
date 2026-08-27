// devela/src/data/store/key/map/_reexport_alloc.rs

use crate::{_reexport, _tags};

/* from `alloc` */

_reexport! { rust: alloc::collections,
    location: "data/store/key/map" => struct BTreeMap, tag: _tags!(data_structure ord),
    doc: "An ordered map based on a B-Tree.",
    BTreeMap
}
_reexport! { rust: alloc::collections::btree_map,
    location: "data/store/key/map" => struct BTreeMapEntry, tag: _tags!(data_structure ord),
    doc: "An entry of an ordered map based on a B-Tree.",
    @Entry as BTreeMapEntry
}
