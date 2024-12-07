// devela::num::frac
//
//! Fractional functionality.
//

// mod r#trait; // TODO
mod wrapper;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::wrapper::*;
    }
    pub(crate) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
