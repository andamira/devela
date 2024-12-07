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

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::reexports::*;
        #[cfg(_cmp_·)]
        pub use super::compare::*;
    }
    pub(crate) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
