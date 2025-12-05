// devela_base_core::code::ops
//
#![doc = crate::_DOC_CODE_PANIC!()]
//

mod punroll; // punroll!
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            punroll::*,
            reexports::*,
        };
    }
}
