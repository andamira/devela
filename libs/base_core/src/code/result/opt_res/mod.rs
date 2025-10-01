// devela_base_core::code::result::opt_res
//
//! Optional values.
//

mod ext_option; // ExtOption
mod ext_result; // ExtResult
mod fmt; // OptionFmt, OptionFmtOr, OptionFmtOrElse
mod opt_res; // serr, sok, OptRes
mod unwrap; // unwrap!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            ext_option::*, ext_result::*, fmt::*, opt_res::*, unwrap::*,
        };
    }
}
