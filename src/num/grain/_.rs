// devela/src/num/grain/_.rs
//
#![doc = crate::_DOC_NUM_GRAIN!()] // public
#![doc = crate::_doc!(modules: crate::num; grain: niche, prim, wide)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: num)]
//

crate::mods_in! {
        // mod_ big; // TODO
        // mod_ float; // Compact binary floating-point formats and representations
        mod_ lim; // Boundary-aware integer representations and arithmetic
    pub mod_ niche; // Specialized numeric types and behaviors
    pub mod_ prim; // Primitive numeric types, families, and representations
    pub mod_ wide; // Wide numeric types and parallel arithmetic
}
crate::mods_out! { // _mods, _pub_mods, _crate_internals, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            // big::_all::*,
            // float::_all::*,
            lim::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            niche::_all::*,
            prim::_all::*,
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
