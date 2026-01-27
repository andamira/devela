// devela_base_core::num::grain
//
#![doc = crate::_DOC_NUM_GRAIN!()]
//

mod cast; // Cast
mod wide; // define_lane!

pub mod niche; // NonZero*, NonZero*, NonValue*|NonExtreme*, ne!, nz!

crate::structural_mods! { // _mods, _pub_mods, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            cast::_all::*,
            wide::_all::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            niche::_all::*,
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
