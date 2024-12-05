// devela::num::frac
//
//! Fractional functionality.
//

// mod r#trait;
mod wrapper;

// structural access
crate::items! {
    mod doc_inline {
        pub use super::wrapper::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(crate) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
