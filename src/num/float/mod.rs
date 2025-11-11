// devela::num::float
//
#![doc = crate::_DOC_NUM_FLOAT!()]
//

mod ext_float; // FloatExt
mod float_const; // FloatConst
mod reexports; // core::num::FloatCategory
mod wrapper; // Float

crate::structural_mods! { // _mods, crate_internals
    _mods {
        pub use super::{
            ext_float::*,
            float_const::*,
            reexports::*,
            wrapper::*,
        };
    }
}
