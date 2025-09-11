// devela::num::float
//
#![doc = crate::_DOC_NUM_FLOAT!()]
//

mod constants; // FloatConst
mod reexports; // core::num::FloatCategory
mod wrapper; // Float

#[cfg(_float··)]
crate::items! {
    mod ext_float; // ExtFloat
}

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::{
            constants::*,
            reexports::*,
            wrapper::*,
        };

        #[cfg(_float··)] #[allow(unused, reason = "feature-gated")]
        pub use super::{
            ext_float::*,
            wrapper::*,
        };
    }
    _always {
        pub use super::{
            reexports::*,
        };
    }
}
