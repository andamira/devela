// devela::lang::js
//
//! Javascript interfacing.
//

#[cfg(feature = "unsafe_ffi")]
crate::items! {
    mod namespace; // Js
    mod reexport; // js_reexport!
}

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        #[cfg(feature = "unsafe_ffi")]
        pub use super::{namespace::*, reexport::*};
        // WIPZONE
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
