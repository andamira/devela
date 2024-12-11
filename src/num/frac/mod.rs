// devela::num::frac
//
//! Fractional functionality.
//

// mod r#trait; // TODO

#[cfg(_int_·)]
mod wrapper;

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        #[cfg(_int_·)]
        pub use super::wrapper::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        #[allow(unused, reason = "feature-gated")]
        pub use super::doc_inline::*;
    }
}
