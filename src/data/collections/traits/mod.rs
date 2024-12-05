// devela::data::collections::traits
//
//! Abstract data types
//

mod array;
mod collection;
mod queues;
mod stacks;

// structural access
crate::items! {
    mod doc_inline {
        pub use super::{array::*, collection::*, queues::*, stacks::*};
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
