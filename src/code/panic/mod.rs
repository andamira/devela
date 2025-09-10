// devela::code::panic
//
#![doc = crate::_DOC_CODE_PANIC!()]
//!
#![doc = crate::_doc!(extends: panic)]
//

mod set; // set_panic_handler!

// re-exports
crate::mod_path!(_c "../../../libs/base_core/src/code/panic/reexports.rs");
crate::mod_path!(std _s "../../../libs/base_std/src/code/panic/reexports.rs");

crate::structural_mods! { // _mods
    _mods {
        pub use super::set::*;

        // re-exports
        pub use super::_c::*;
        #[cfg(feature = "std")]
        pub use {
            devela_base_std::code::panic::Panic,
            super::_s::*,
        };
    }
}
