// devela::num::rand
//
//! Random number generation.
//!
//! This module defines several types:
//! - RNG algorithms specialized for 8-bit devices:
//!   [`Xabc`], [`Xyza8a`], [`Xyza8b`].
//! - Classic *XorShift* algorithms and variations with a smaller state.
//!
//! The RNGs implement `Copy` for convenience and versatility.
//! Be careful to not duplicate the state by accident.
//!
//! The `Default` implementation uses a fixed seed for convenience.
//! Use a custom seed for unique random sequences.
//!
//! [`RngCore`]: https://docs.rs/rand_core/latest/rand_core/trait.RngCore.html
//!
//! # Features
//! All <abbr title="Pseudo-Random Number Generator">PRNG</abbr>s require the
//! `rand` feature, except for [`XorShift128p`], which is always compiled.
//

mod xorshift;

#[cfg(feature = "rand")]
crate::items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rand")))]
    mod lgc;
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rand")))]
    mod xoroshiro;
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rand")))]
    mod xyza8;
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rand")))]
    mod xabc;
}

crate::items! { // structural access: _mods, _internals, _all
    #[allow(unused)]
    pub use {_mods::*, _internals::*};

    mod _mods {
        pub use super::xorshift::*;
        #[cfg(feature = "rand")]
        pub use super::{lgc::*, xabc::*, xoroshiro::*, xyza8::*};
    }
    pub(super) mod _internals { #![allow(unused)]
        #[cfg(feature = "rand")]
        pub(crate) use super::xorshift::xorshift_basis;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
