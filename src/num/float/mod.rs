// devela::num::float
//
#![doc = crate::_DOC_NUM_FLOAT!()]
//

mod float_const; // FloatConst
mod reexports; // core::num::FloatCategory
mod wrapper; // Float

#[cfg(_float··)]
crate::items! {
    mod ext_float; // ExtFloat
}

crate::structural_mods! { // _mods, crate_internals
    _mods {
        pub use super::{
            float_const::*,
            reexports::*,
            wrapper::*,
        };

        #[cfg(_float··)] #[allow(unused, reason = "feature-gated")]
        pub use super::{
            ext_float::*,
            wrapper::*,
        };
    }
}
