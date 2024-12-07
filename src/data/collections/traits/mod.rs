// devela::data::collections::traits
//
//! Abstract data types
//

mod array;
mod collection;
mod queues;
mod stacks;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{array::*, collection::*, queues::*, stacks::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
