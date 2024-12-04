// devela::code::result::option
//
//! Optional values.
#![doc = crate::doc_!(extends: option)]
#![doc = crate::doc_!(modules: crate::code::result; option)]
#![doc = crate::doc_!(newline)]
//!
//

mod ext_option;
mod ext_result;
mod fmt;
mod opt_res;
mod unwrap;

// re-export private sub-modules
#[allow(unused_imports)]
pub use {
    ext_option::ExtOption,
    ext_result::ExtResult,
    fmt::{OptionFmt, OptionFmtOr, OptionFmtOrElse},
    opt_res::{serr, sok, OptRes},
    unwrap::unwrap,
};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{ext_option::*, ext_result::*, fmt::*, opt_res::*, unwrap::*};
}
