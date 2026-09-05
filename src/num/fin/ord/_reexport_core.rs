// devela/src/num/fin/ord/_reexport_core.rs

use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: core::cmp,
    location: "num/fin/ord" => struct Reverse, tag: _tags!(num ord),
    doc: "A helper struct for reverse ordering.",
    Reverse
}

/* enums */

_reexport! { rust: core::cmp,
    location: "num/fin/ord" => enum Ordering, tag: _tags!(num ord result),
    doc: "The result of a comparison between two values.",
    Ordering
}

/* traits */

// NOTE: the following traits and the corresponding derive macros have the same name:

_reexport! { rust: core::cmp,
    location: "num/fin/ord" => trait Eq, tag: _tags!(num),
    doc: "Trait for comparisons corresponding to
        [equivalence relations](https://en.wikipedia.org/wiki/Equivalence_relation). (Derivable)",
    Eq
}
_reexport! { rust: core::cmp,
    location: "num/fin/ord" => trait PartialEq, tag: _tags!(num),
    doc: "Trait for comparisons using the equality operator. (Derivable)",
    PartialEq
}

_reexport! { rust: core::cmp,
    location: "num/fin/ord" => trait Ord, tag: _tags!(num ord),
    doc: "Trait for types that form a
        [total order](https://en.wikipedia.org/wiki/Total_order). (Derivable)",
    Ord
}
_reexport! { rust: core::cmp,
    location: "num/fin/ord" => trait PartialOrd, tag: _tags!(num ord),
    doc: "Trait for types that form a
        [partial order](https://en.wikipedia.org/wiki/Partial_order). (Derivable)",
    PartialOrd
}
