// devela::num::dom::real::float
//
#![doc = crate::_DOC_NUM_DOM_REAL_FLOAT!()] // private
#![doc = crate::_doc!(modules: crate::num::dom::real; float)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

mod _reexport_core;

pub(crate) mod _consts; // PI, TAU, SQRT2, …
pub(crate) mod _docs; // _FLOAT_[ALGORITHM|FORMULA|NOTATION|PIECEWISE]_*!()
mod _internals; // _FloatInternals

mod alias; // fsize
mod bits; // f[32|64]bits[_niche]
mod ext_float; // FloatExt
mod float_const; // FloatConst
mod wrapper; // Float

crate::structural_mods! { // _mods, _reexports, _crate_internals
    _mods {
        pub use super::{
            alias::*,
            bits::*,
            ext_float::*,
            float_const::*,
            wrapper::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
    _crate_internals {
        pub use super::{
            _consts::*,
            _docs::*,
            _internals::*,
        };
    }
}
