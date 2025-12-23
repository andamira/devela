// devela_base_core::num::niche
//
#![doc = crate::_DOC_NUM_NICHE!()]
//

mod absence; // MaybeNiche, NonNiche
mod macros; // niche_prim!, ne!, nv!, nz!, (NicheNew)
mod mem; // NonExtreme*, NonValue*
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            absence::*,
            macros::{niche_prim, ne, nv, nz},
            mem::*,
            reexports::*,
        };
    }
    _workspace_internals {
        pub use super::macros::NicheNew;
    }
}
