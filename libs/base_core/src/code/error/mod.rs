// devela_base_core::code::error
//
#![doc = crate::_DOC_CODE_ERROR!()]
//

mod define_error; // define_error!
mod errors; // general errors
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{define_error::*, errors::*, reexports::*};
    }
}
