// devela::data::xipher
//
//! Cryptographic primitives for encryption, authentication, and hashing.
//

// mod otp; // One-time passwords

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
