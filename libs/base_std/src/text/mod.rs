// devela_base_std::text
//
#![doc = crate::_DOC_TEXT!()]
//

pub mod str;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            // fmt::_all::*,
            // parse::_all::*,
            str::_all::*,
        };
    }
}
