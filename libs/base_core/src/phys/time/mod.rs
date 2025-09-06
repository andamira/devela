// devela_base_core::phys::time
//
#![doc = crate::_DOC_PHYS_TIME!()]
//

mod errors;
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{errors::*, reexports::*};
    }
}
