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

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{
            ext_option::*, ext_result::*, fmt::*, opt_res::*, unwrap::*,
        };
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
