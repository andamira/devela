// devela_base_core::num::grain
//
#![doc = crate::_DOC_NUM_GRAIN!()] // public
#![doc = crate::_doc!(modules: crate::num; grain: niche, wide)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: num)]
//

mod cast; // Cast
mod prim; // Prim* traits

pub mod niche; // NonZero*, NonZero*, NonValue*|NonExtreme*, ne!, nz!
pub mod wide; // define_lane!

crate::structural_mods! { // _mods, _pub_mods, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            cast::_all::*,
            prim::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            niche::_all::*,
            wide::_all::*,
        };
    }
    _workspace_internals {
        pub use super::{
            niche::_workspace_internals::*,
        };
    }
    _hidden {
        pub use super::{
            wide::_hidden::*,
        };
    }
}
