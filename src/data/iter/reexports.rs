// devela::data::iter::reexports

use crate::_reexport;

crate::mod_path!(+pub _c "../../../libs/base/src/data/iter/reexports.rs");

/* itertool items */

_reexport! { "dep_itertools", "itertools", itertools,
    tag: crate::TAG_ITERATOR!(),
    doc: "An [`Iterator`] blanket impl providing extra adaptors and methods.",
    Itertools
}
_reexport! { "dep_itertools", "itertools", itertools,
    tag: crate::TAG_ITERATOR!(),
    doc: "An iterator that can be unzipped into multiple collections.",
    @MultiUnzip as IteratorMultiUnzip
}
_reexport! { "dep_itertools", "itertools", itertools,
    tag: crate::TAG_ITERATOR!(),
    doc: "An iterator that allows peeking at an element before deciding to accept it.",
    @PeekingNext as IteratorPeekingNext
}
