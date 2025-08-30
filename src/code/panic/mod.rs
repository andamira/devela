// devela::code::panic
//
#![doc = crate::_DOC_CODE_PANIC!()]
//!
#![doc = crate::_doc!(extends: panic)]
//

mod namespace; // Panic
mod reexports; // ::core::panic::*
mod set; // set_panic_handler!

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::{namespace::*, reexports::*, set::*};
    }
    _always {
        pub use super::reexports::*;
    }
}
