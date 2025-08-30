// devela_base::code::marker
//
#![doc = crate::_DOC_CODE_MARKER!()]
//

mod reexports;
mod type_marker; // zero-cost generic type markers

crate::structural_mods! { // _mods
    _mods {
        pub use super::{type_marker::*, reexports::*};
    }
}
