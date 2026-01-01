// devela_base_core::code::result
//
#![doc = crate::_DOC_CODE_RESULT!()]
//

mod _reexport;

mod chain_hook; // Chain, Hook
mod mismatch; // Mismatch
mod opt_res; // OptRes, sok, serr
mod own; // Own

// WIPZONE
// mod enumatch; // enumatch!
// #[cfg(feature = "_tuple")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "_tuple")))]
// mod menu;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            chain_hook::*,
            mismatch::*,
            opt_res::_all::*,
            own::*,
        };

        // WIPZONE
        // pub use super::enumatch::*;
        // #[cfg(feature = "_tuple")]
        // pub use super::menu::*;
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
