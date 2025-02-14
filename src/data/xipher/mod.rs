// devela::data::xipher
//
//! Cryptographic primitives for encryption, authentication, and hashing.
//

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        // WIPZONE
        // pub use super::otp::*;
        // pub use super::sha1::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// mod otp;
// mod sha1;
