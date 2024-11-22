// devela::num::cmp::reexports
//
//! Reexported items from `core`.
//

use crate::reexport;

reexport! { rust: core::cmp, doc: "A helper struct for reverse ordering.",
    Reverse
}

reexport! { rust: core::cmp,
    doc: "The result of a comparison between two values.",
    Ordering
}

/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
#[doc(inline)]
pub use core::cmp::{Eq, Ord, PartialEq, PartialOrd};
