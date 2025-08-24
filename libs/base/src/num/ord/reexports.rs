// devela_base::num::ord::reexports
//
//!
//

use crate::_reexport;

_reexport! { rust: core::cmp, doc: "A helper struct for reverse ordering.",
    Reverse
}
_reexport! { rust: core::cmp,
    tag: crate::TAG_RESULT!(),
    doc: "The result of a comparison between two values.",
    Ordering
}

/// <span class='stab portability'
/// title='re-exported from rust&#39;s `core`'>`core`</span>
#[doc(inline)]
pub use ::core::cmp::{Eq, Ord, PartialEq, PartialOrd};
