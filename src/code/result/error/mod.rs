// devela::code::result::error
//
//! Error-related types and traits.
//!
//! It re-exports the error and result types defined in other modules.
//!
#![doc = crate::doc_!(extends: error)]
//

mod ext; // ExtError
mod all_error; // AllError, modular errors
mod reexports;

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{all_error::*, ext::*, reexports::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::{all_error::*, ext::*, reexports::*};
    }
}
