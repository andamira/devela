// devela::num::rand
//
//! Random number generation.
//!
//! This module defines several types:
//! - RNG algorithms specialized for 8-bit devices:
//!   [`Xabc`], [`Xyza8a`], [`Xyza8b`].
//! - Classic *XorShift* algorithms and variations with a smaller state.
//!
//! These RNGs differ from the recommendations in [`RngCore`]
//! by implementing [`Copy`] and [`Default`].
//!
//! [`RngCore`]: https://docs.rs/rand_core/latest/rand_core/trait.RngCore.html
//

mod xabc;

#[cfg(feature = "cast")]
crate::items! {
    mod lgc;
    mod xoroshiro;
    mod xorshift;
    mod xyza8;
}

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::xabc::*;

        #[cfg(feature = "cast")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "cast")))]
        pub use super::{lgc::*, xoroshiro::*, xorshift::*, xyza8::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
