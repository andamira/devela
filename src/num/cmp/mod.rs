// devela::num::cmp
//
//! Comparing and ordering values.
// #![doc = crate::doc_!(extends: cmp)]
// #![doc = crate::doc_!(modules: crate::num; cmp)]
// #![doc = crate::doc_!(newline)]
//

mod reexports;

#[cfg(_cmp_·)]
mod compare; // `Compare`

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::reexports::*;

        #[cfg(_cmp_·)]
        pub use super::compare::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
