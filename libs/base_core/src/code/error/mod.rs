// devela_base_core::code::error
//
#![doc = crate::_DOC_CODE_ERROR!()]
//

mod _reexport;

mod define_error; // define_error!
mod errors; // general errors

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            define_error::*,
            errors::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
