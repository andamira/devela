// devela/src/code/panic/_.rs
//
#![doc = crate::_DOC_CODE_PANIC!()] // public
#![doc = crate::_doc!(modules: crate::code; panic)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: panic)]
//

crate::mods_in! {
    mod _reexport_core;
    #[cfg(feature = "std")]
    mod _reexport_std;

    mod handler; // set_panic_handler!

    #[cfg(feature = "std")]
    mod namespace; // Panic
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::handler::set_panic_handler;

        #[cfg(feature = "std")]
        pub use super::namespace::Panic;
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
