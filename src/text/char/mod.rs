// devela::text::char
//
//! Unicode scalars.
// #![doc = crate::doc_!(extends: char)]
// #![doc = crate::doc_!(modules: crate::text; char)]
// #![doc = crate::doc_!(newline)]
//

// without re-exports
mod core_impls;
mod impls;
#[cfg(test)]
mod tests;

// with re-exports
mod definitions;
mod namespace;
mod reexports;

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{definitions::*, namespace::*, reexports::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
