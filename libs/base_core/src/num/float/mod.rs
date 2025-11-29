// devela_base_core::num::float
//
#![doc = crate::_DOC_NUM_FLOAT!()]
//

pub(crate) mod _consts; // PI, TAU, SQRT2, …
pub(crate) mod _docs; // _FLOAT_[ALGORITHM|FORMULA|NOTATION|PIECEWISE]_*!()
mod _internals; // _FloatInternals

mod alias; // fsize
mod bits; // f32bits, f64bits
mod float_const; // FloatConst
mod reexports; // FloatCategory
mod wrapper; // Float

crate::structural_mods! { // _mods, _workspace_internals
    _mods {
        pub use super::{
            alias::*,
            bits::*,
            float_const::*,
            reexports::*,
            wrapper::_all::*,
        };
    }
    _workspace_internals {
        pub use super::{
            _consts::*,
            _docs::*,
            _internals::*,
        };
    }
}
