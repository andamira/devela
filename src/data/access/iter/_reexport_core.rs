// devela::data::access::iter::_reexport_core
//
// - WAIT: [iter_map_windows](https://github.com/rust-lang/rust/issues/87155)
// - WAIT: [iter_array_chunks](https://github.com/rust-lang/rust/issues/100450)
// - WAIT: [iter_intersperse](https://github.com/rust-lang/rust/issues/79524)
// - WAIT: [step_trait](https://github.com/rust-lang/rust/issues/42168)
// - WAIT: [trusted_len](https://github.com/rust-lang/rust/issues/37572)
// - WAIT: [trusted_step](https://github.com/rust-lang/rust/issues/85731)

use crate::{_reexport, _tags};

// NOTE: interator implementation structs are not re-exported

/* core traits */

_reexport! { rust: core::iter, location: "data/access/iter", tag: _tags!(iterator),
    doc: "An iterator able to yield elements from both ends.",
    @DoubleEndedIterator as IteratorDoubleEnded
}
_reexport! { rust: core::iter, location: "data/access/iter", tag: _tags!(iterator),
    doc: "An iterator that knows its exact length.",
    @ExactSizeIterator as IteratorExactSize
}
_reexport! { rust: core::iter, location: "data/access/iter", tag: _tags!(iterator),
    doc: "Extend a collection with the contents of an iterator.",
    @Extend as IteratorExtend
}
_reexport! { rust: core::iter, location: "data/access/iter", tag: _tags!(iterator),
    doc: "Conversion from an [`Iterator`].",
    @FromIterator as IteratorFrom
}
_reexport! { rust: core::iter, location: "data/access/iter", tag: _tags!(iterator),
    doc: "An iterator that always continues to yield `None` when exhausted.",
    @FusedIterator as IteratorFused
}
_reexport! { rust: core::iter, location: "data/access/iter", tag: _tags!(iterator),
    doc: "Conversion into an [`Iterator`].",
    @IntoIterator as IteratorInto
}
_reexport! { rust: core::iter, location: "data/access/iter", tag: _tags!(iterator),
    doc: "A trait for dealing with iterators.",
    Iterator
}
_reexport! { rust: core::iter, location: "data/access/iter", tag: _tags!(iterator num),
    doc: "Represents types that can be created by multiplying elements of an iterator.",
    @Product as IteratorProduct
}
_reexport! { rust: core::iter, location: "data/access/iter", tag: _tags!(iterator num),
    doc: "Represents types that can be created by summing up an iterator.",
    @Sum as IteratorSum
}

/* core functions */

// These are re-exported as methods of the Iter namespace.

_reexport! { rust: core::iter, extra_flags:(nightly_coro),
    location: "data/access/iter", tag: _tags!(iterator runtime),
    doc: "Creates an iterator where each iteration calls the provided coroutine.",
    @from_coroutine as iter_from_coroutine
}
_reexport! { rust: core::iter, extra_flags:(nightly_coro),
    location: "data/access/iter", tag: _tags!(iterator runtime),
    doc: "Creates an iterator where each iteration calls the provided coroutine.",
    @FromCoroutine as IterFromCoroutine
}
