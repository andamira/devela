// devela::num::frac
//
//! Fractional functionality.
//

// mod r#trait; // TODO

#[cfg(_int_·)]
mod wrapper;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        #[cfg(_int_·)]
        pub use super::wrapper::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        #[allow(unused, reason = "feature-gated")]
        pub use super::_mods::*;
    }
}
