// devela::num::prob::rand::prng::shift
//
//! Pseudo-random number generators based on [Xorshift].
//!
//! This module defines several types:
//! - classic *XorShift* algorithms:
//!   ([`XorShift32`], [`XorShift64`], [`XorShift128`], [`XorShift128p`]).
//! - variations with a smaller state:
//!   ([`XorShift16`], [`XorShift8`]).
//!
//! - Original paper: <https://www.jstatsoft.org/article/view/v008i14>
//!
//! All Xorshift generators in this module are classified as weak PRNGs due to
//! their linear structure and, for smaller variants, limited state and period.
//!
//! [Xorshift]: https://en.wikipedia.org/wiki/Xorshift
//

#[cfg(feature = "rand")]
crate::items! {
    mod u8;   // ( 3,  4,  2)   (customizable)
    mod u16;  // ( 7,  9,  8)
    mod u32;  // ( 5, 17, 13)   (customizable, canonical default)
    mod u64;  // (13,  7, 17)   (customizable, canonical default)
    mod u128; // (11,  8, 19)   (canonical)
    mod u128p; // XorShift128+  (canonical)
}
#[cfg(feature = "rand")]
mod macros;

crate::structural_mods! { // _mods, _hidden
    _mods {
        #[cfg(feature = "rand")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "rand")))]
        pub use super::{
            macros::rand_xorshift,
            u8::XorShift8,
            u16::XorShift16,
            u32::XorShift32,
            u64::XorShift64,
            u128::XorShift128,
            u128p::XorShift128p,
        };
    }
    _crate_internals {
        #[cfg(feature = "rand")]
        pub(crate) use super::{
            macros::_xorshift_basis,
        };
    }
    _hidden {
        #[cfg(feature = "rand")]
        pub use super::{
            u16::XOROSHIFT_16_TRIPLETS,
            u32::XOROSHIFT_32_TRIPLETS,
            u64::XOROSHIFT_64_TRIPLETS,
        };
    }
}
