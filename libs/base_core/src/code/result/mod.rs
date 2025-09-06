// devela_base_core::code::result
//
#![doc = crate::_DOC_CODE_RESULT!()]
//

mod chain_hook; // Chain, Hook
mod mismatch; // Mismatch
mod opt_res; // OptRes, sok, serr
mod own; // Own
mod reexports;
mod value_quant; // ValueQuant

// WIPZONE
// mod enumatch; // enumatch!
// #[cfg(feature = "_tuple")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "_tuple")))]
// mod menu;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            chain_hook::*, mismatch::*, opt_res::_all::*, own::*, reexports::*, value_quant::*,
        };

        // WIPZONE
        // pub use super::enumatch::*;
        // #[cfg(feature = "_tuple")]
        // pub use super::menu::*;
    }
}
