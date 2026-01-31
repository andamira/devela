// devela_base_core::num::dom::real::float
//
#![doc = crate::_DOC_NUM_DOM_REAL_FLOAT!()] // private
#![doc = crate::_doc!(modules: crate::num::dom::real; float)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

pub(crate) mod _consts; // PI, TAU, SQRT2, …
pub(crate) mod _docs; // _FLOAT_[ALGORITHM|FORMULA|NOTATION|PIECEWISE]_*!()
mod _internals; // _FloatInternals
mod _reexport; // SYMLINK from /src/num/dom/realfloat/_reexport_core.rs

mod alias; // fsize
mod bits; // f32bits, f64bits
mod float_const; // FloatConst
mod wrapper; // Float

crate::structural_mods! { // _mods, _reexports, _workspace_internals
    _mods {
        pub use super::{
            alias::*,
            bits::*,
            float_const::*,
            wrapper::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
    _workspace_internals {
        pub use super::{
            _consts::*,
            _docs::*,
            _internals::*,
        };
    }
}
