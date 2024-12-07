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
    pub(crate) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
