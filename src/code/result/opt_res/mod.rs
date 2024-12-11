// devela::code::result::option
//
//! Optional values.
#![doc = crate::doc_!(extends: option)]
#![doc = crate::doc_!(modules: crate::code::result; option)]
#![doc = crate::doc_!(newline)]
//!
//

mod ext_option; // ExtOption
mod ext_result; // ExtResult
mod fmt; // OptionFmt, OptionFmtOr, OptionFmtOrElse
mod opt_res; // serr, sok, OptRes
mod unwrap; // unwrap!

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{
            ext_option::*, ext_result::*, fmt::*, opt_res::*, unwrap::*,
        };
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::{ext_option::*, ext_result::*, fmt::*, opt_res::*, unwrap::*};
    }
}
