// devela_base::text::char
//
#![doc = crate::_DOC_TEXT_CHAR!()]
//

mod namespace; // Char
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            namespace::*,
            reexports::*,
        };
    }
}
