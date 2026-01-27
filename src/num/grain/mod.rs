// devela::num::grain
//
#![doc = crate::_DOC_NUM_GRAIN!()]
//

mod cast; // (Cast), PrimitiveCast, PrimitiveJoin, PrimitiveSplit
pub mod niche; // (MaybeNiche, NonNiche*, NonZero*, NonValue*|NonExtreme*, ne!, nz!)
pub mod wide; // (define_lane!)

crate::structural_mods! { // _mods, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            cast::_all::*,
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
