// devela::media::audio
//
//! Audio functionality.
//
// safety
#![cfg_attr(feature = "safe_audio", forbid(unsafe_code))]

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        // WIPZONE
        // pub use super::drum_machine::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
    }
}
// WIPZONE
// mod drum_machine;
