// devela_base_core::num::ord::_reexport
//
//!
//

use crate::{_TAG_NUM, _reexport, _tags};

_reexport! { rust: core::cmp, location: "num/ord", tag: _TAG_NUM!(),
    doc: "A helper struct for reverse ordering.",
    Reverse
}
_reexport! { rust: core::cmp, location: "num/ord", tag: _tags!(num result),
    doc: "The result of a comparison between two values.",
    Ordering
}

_reexport! { rust: core::cmp, location: "num/ord", tag: _TAG_NUM!(),
    doc: "Trait for comparisons corresponding to
        [equivalence relations](https://en.wikipedia.org/wiki/Equivalence_relation). (Derivable)",
    Eq
}
_reexport! { rust: core::cmp, location: "num/ord", tag: _TAG_NUM!(),
    doc: "Trait for comparisons using the equality operator. (Derivable)",
    PartialEq
}

_reexport! { rust: core::cmp, location: "num/ord", tag: _TAG_NUM!(),
    doc: "Trait for types that form a
        [total order](https://en.wikipedia.org/wiki/Total_order). (Derivable)",
    Ord
}
_reexport! { rust: core::cmp, location: "num/ord", tag: _TAG_NUM!(),
    doc: "Trait for types that form a
        [partial order](https://en.wikipedia.org/wiki/Partial_order). (Derivable)",
    PartialOrd
}
