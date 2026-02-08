// devela::code::panic
//
#![doc = crate::_DOC_CODE_PANIC!()] // public
#![doc = crate::_doc!(modules: crate::code; panic)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: panic)]
//

mod _reexport_core; // SYMLINK to /crates/base/core/src/code/panic/_reexport.rs
mod _reexport_std; // SYMLINK to /crates/base/std/src/code/panic/_reexport.rs

mod set; // set_panic_handler!

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::set::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        pub use super::_reexport_std::*;
        #[cfg(feature = "std")]
        pub use devela_base_std::code::panic::Panic;
    }
}
