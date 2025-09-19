// devela::code::error
//
#![doc = crate::_DOC_CODE_ERROR!()]
//!
//! It re-exports the error and result types defined in other modules.
//!
#![doc = crate::_doc!(extends: backtrace, error)]
//

mod reexports;

// WIPZONE
// mod num; // numeric-related errors

crate::structural_mods! { // _mods
    _mods {
        pub use super::reexports::*;
        // WIPZONE
        // pub use super::num::*;
    }
}
