// devela_base::code::result
//
#![doc = crate::_DOC_CODE_RESULT!()]
//

mod mismatch; // Mismatch
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{mismatch::*, reexports::*};
    }
}
