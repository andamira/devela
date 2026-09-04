// devela/src/num/grain/_.rs
//
#![doc = crate::_DOC_NUM_GRAIN!()] // public
#![doc = crate::_doc!(modules: crate::num; grain: niche, wide)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: num)]
//

crate::mods_in! {
        // mod_ big; // TODO
        mod_ cast; // Cast, PrimCast, PrimJoin, PrimSplit, cast!
        // mod_ float; // Compact binary floating-point formats and representations
        mod_ lim; // Boundary-aware integer representations and arithmetic
    pub mod_ niche; // MaybeNiche, NonNiche*, NonZero*, Non<Max|Min|Value>*, nm!, nv!, nz!
        mod prim; // PrimFloat, PrimInt, PrimScalar, PrimSint, PrimUint
    pub mod_ wide; // lane!
}
crate::mods_out! { // _mods, _pub_mods, _crate_internals, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            // big::_all::*,
            cast::_all::*,
            // float::_all::*, // RELOCATE
            lim::_all::*,
            prim::*,
        };
    }
    _pub_mods {
        pub use super::{
            niche::_all::*,
            wide::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            lim::_crate_internals::*,
        };
    }
    _hidden {
        pub use super::{
            lim::_hidden::*,
            niche::_hidden::*,
            wide::_hidden::*,
        };
    }
}
