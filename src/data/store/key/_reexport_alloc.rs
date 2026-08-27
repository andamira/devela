// devela/src/data/store/key/_reexport_alloc.rs

use crate::{_reexport, _tags};

/* from `alloc` */

_reexport! { rust: alloc::collections,
    location: "data/store/key" => struct BTreeSet, tag: _tags!(data_structure ord set),
    doc: "An ordered set based on a B-Tree.",
    BTreeSet
}
