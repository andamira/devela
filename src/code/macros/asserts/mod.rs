// devela::code::macros::asserts
//
//! Asserts.
//

mod dynamic; // assert_eq_all, assert_approx_eq_all
mod r#static;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{dynamic::*, r#static::all::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
