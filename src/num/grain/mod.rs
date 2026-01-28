// devela::num::grain
//
#![doc = crate::_DOC_NUM_GRAIN!()]
#![doc = crate::_doc!(modules: crate::num; grain: niche, wide)]
#![doc = crate::_doc!(flat:"num")]
//!
#![doc = crate::_doc!(extends: num)]
//

mod cast; // (Cast), PrimitiveCast, PrimitiveJoin, PrimitiveSplit
pub mod niche; // (MaybeNiche, NonNiche*, NonZero*, NonValue*|NonExtreme*, ne!, nz!)
pub mod wide; // (define_lane!)

crate::structural_mods! { // _mods, _pub_mods _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            cast::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            niche::_all::*,
            wide::_all::*,
        };
    }
    _hidden {
        pub use super::{
            wide::_hidden::*,
        };
    }
}
