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

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::{
            ext_option::*, ext_result::*, fmt::*, opt_res::*, unwrap::*,
        };
    }
    _always {
        pub use super::{ext_option::*, ext_result::*, fmt::*, opt_res::*, unwrap::*};
    }
}
