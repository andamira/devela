// devela::data::collections::stack
//
//! A type that can be used as a single-ended stack.
//

// no items defined
mod impl_traits;
mod methods;

mod definitions; // Stack, StackIter, …

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::definitions::*;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
