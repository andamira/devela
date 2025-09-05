// devela_base::num::niche
//
#![doc = crate::_DOC_NUM_NICHE!()]
//

mod mem; // NonExtreme*, NonValue*
mod macros; // ne!, nz! (NicheNew)
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            mem::*,
            macros::{ne, nz},
            reexports::*,
        };
    }
    _workspace_internals {
        pub use super::macros::NicheNew;
    }
}
