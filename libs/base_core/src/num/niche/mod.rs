// devela_base_core::num::niche
//
#![doc = crate::_DOC_NUM_NICHE!()]
//

mod _reexport; // SYMLINK from /src/num/niche/_reexport_core.rs

mod absence; // MaybeNiche, NonNiche
mod macros; // niche_prim!, ne!, nv!, nz!, (NicheNew)
mod mem; // NonExtreme*, NonValue*
// mod norm; // Norm WIP

crate::structural_mods! { // _mods, _reexports, _workspace_internals
    _mods {
        #[doc(inline)]
        pub use super::{
            absence::*,
            macros::{niche_prim, ne, nv, nz},
            mem::*,
            // norm::*, // WIP
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
    _workspace_internals {
        pub use super::macros::NicheNew;
    }
}
