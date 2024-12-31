// devela::num::float
//
//! Floating point functionality.
//

mod alias; // fsize
mod constants; // ExtFloatConst
mod reexports;
mod wrapper; // Float

#[cfg(_float··)]
crate::items! {
    mod ext_float; // ExtFloat
    mod shared_docs; // FORMULA_*!()
}

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{alias::*, constants::*, reexports::*, wrapper::*};

        #[cfg(_float··)] #[allow(unused, reason = "feature-gated")]
        pub use super::{ext_float::*, wrapper::*, shared_docs::*};
    }
    pub(super) mod _all { #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{alias::*, reexports::*};
    }
}
