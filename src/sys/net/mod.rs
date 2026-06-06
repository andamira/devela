// devela::sys::net
//
#![doc = crate::_DOC_SYS_NET!()] // public
#![doc = crate::_doc!(modules: crate::sys; net)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: net)]
//

mod _reexport_core;
#[cfg(feature = "std")]
mod _reexport_std;

// #[cfg(feature = "std")]
// mod http; // WIP
// mod imap; // WIP
// mod fpt; // WIP
// mod smtp; // WIP
// mod telegram; // WIP

crate::structural_mods! { // _mods, _reexports
    _mods {
        // #[cfg(feature = "std")]
        // pub use super::http::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
