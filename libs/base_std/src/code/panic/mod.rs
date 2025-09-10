// devela_base_std::code::panic
//
#![doc = crate::_DOC_CODE_PANIC!()]
//

mod namespace; // Panic
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            namespace::*,
            reexports::*,
        };
    }
}
