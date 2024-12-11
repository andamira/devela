// devela::sys::sound
//
//! Sound I/O functionality.
//

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {}

    pub(super) mod all { #![allow(unused)]
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
