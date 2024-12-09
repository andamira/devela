// devela::num::logic
//
//! Logic related types and functionality.
//

mod bool;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::bool::*;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
