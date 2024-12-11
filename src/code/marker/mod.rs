// devela::code::marker
//
//! Marker types, traits and macros.
//!
#![doc = crate::doc_!(extends: marker)]
//

mod reexports; // core::marker re-exports
mod marker; // zero-cost generic type markers
mod res; // zero-cost type-safe resource markers

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{marker::*, reexports::*, res::*};
    }
    pub(super) mod all { #![allow(unused)]
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::{marker::*, reexports::*, res::*};
    }
}
