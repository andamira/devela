// devela_base_num::float
//
#![doc = crate::_DOC_NUM_FLOAT!()]
//

pub(crate) mod _consts; // PI, TAU, SQRT2, …
pub(crate) mod _docs; // _FLOAT_[ALGORITHM|FORMULA|NOTATION|PIECEWISE]_*!()
mod _internals; // _FloatInternals

mod float_const; // FloatConst
// mod wrapper; // Float

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            float_const::*,
            // wrapper::_all::*,
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
