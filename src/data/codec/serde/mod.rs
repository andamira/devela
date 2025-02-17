// devela::data::codec::serde
//
//! Structured serialization/deserialization.
//

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        // WIPZONE
        // pub use super::rle::*;
        // pub use super::utils::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// mod rle;
// mod utils;
