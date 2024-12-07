// devela::code::any
//
//! Dynamic typing and reflection.
// #![doc = crate::doc_!(extends: any)]
// #![doc = crate::doc_!(modules: crate::code; any)]
// #![doc = crate::doc_!(newline)]
//

mod ext;
mod reexports;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{ext::*, reexports::*};
    }
    pub(crate) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
