// devela_base_core::num::niche
//
#![doc = crate::_DOC_NUM_NICHE!()]
//

mod absence; // NonNiche
mod macros; // ne!, nv!, nz!, (NicheNew)
mod mem; // NonExtreme*, NonValue*
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            absence::*,
            macros::*,
            mem::*,
            reexports::*,
        };
    }
    _workspace_internals {
        pub use super::macros::NicheNew;
    }
}
