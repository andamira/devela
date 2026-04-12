// devela::num::grain
//
#![doc = crate::_DOC_NUM_GRAIN!()] // public
#![doc = crate::_doc!(modules: crate::num; grain: niche, wide)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: num)]
//

mod cast; // Cast, PrimitiveCast, PrimitiveJoin, PrimitiveSplit, cast!
pub mod niche; // MaybeNiche, NonNiche*, NonZero*, NonValue*|NonExtreme*, ne!, nz!
mod prim; // PrimFloat, PrimInt, PrimScalar, PrimSint, PrimUint
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
        pub use super::{
            niche::_all::*,
            wide::_all::*,
        };
    }
    _hidden {
        pub use super::{
            niche::_hidden::*,
            wide::_hidden::*,
        };
    }
}
