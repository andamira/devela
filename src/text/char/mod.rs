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

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{definitions::*, namespace::*, reexports::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused_imports)]
        pub use super::reexports::*;
    }
}
