// devela::num::frac
//
//! Fractional functionality.
//

// mod r#trait; // TODO

#[cfg(_int_·)]
mod wrapper;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        #[cfg(_int_·)]
        pub use super::wrapper::*;
    }
    pub(crate) mod all { #[doc(inline)]
        #[allow(unused_imports, reason = "feature-gated")]
        pub use super::doc_inline::*;
    }
}
