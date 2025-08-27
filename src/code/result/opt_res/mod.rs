// devela::code::result::option
//
//! Optional values.
//!
#![doc = crate::_doc!(extends: option, result)]
//

mod ext_option; // ExtOption
mod ext_result; // ExtResult
mod fmt; // OptionFmt, OptionFmtOr, OptionFmtOrElse
mod opt_res; // serr, sok, OptRes
mod unwrap; // unwrap!

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{
            ext_option::*, ext_result::*, fmt::*, opt_res::*, unwrap::*,
        };
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{ext_option::*, ext_result::*, fmt::*, opt_res::*, unwrap::*};
    }
}
