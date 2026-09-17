use crate::{_reexport, _tags};

/* from `alloc` */

_reexport! { rust: alloc::collections,
    location: "data/store/key/set" => struct BTreeSet, tag: _tags!(data_structure ord set),
    doc: "An ordered set based on a B-Tree.",
    BTreeSet
}
