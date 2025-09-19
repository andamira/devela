// devela::code::result::option
//
//! Optional values.
//!
#![doc = crate::_doc!(extends: option, result)]
//

mod ext_option; // ExtOption
mod ext_result; // ExtResult
mod fmt; // OptionFmt, OptionFmtOr, OptionFmtOrElse

crate::structural_mods! { // _mods
    _mods {
        pub use super::{ext_option::*, ext_result::*, fmt::*};
    }
}
