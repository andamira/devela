// devela::code::macros::asserts::static
//
//! Static assertions.
//

mod r#const;
mod r#impl;

crate::items! { // structural access: doc_inline, all
    #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        #[allow(unused_imports, reason = "WIP impl")]
        pub use super::{r#const::*, r#impl::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
