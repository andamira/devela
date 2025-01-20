// devela::num::rand::xorshift
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
//! [Xorshift]: https://en.wikipedia.org/wiki/Xorshift
//

mod u128p;
pub use u128p::XorShift128p;

#[cfg(feature = "rand")]
crate::items! {
    mod u128; // 11,  8, 19
    mod u16;  //  7,  9,  8
    mod u32;  // 13, 17,  5  (canonical)
    mod u64;  // 13,  7, 17  (canonical)
    mod u8;   //  3,  4,  2  (customizable)
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rand")))]
    pub use {
        u128::XorShift128,
        u16::XorShift16,
        u32::XorShift32,
        u64::XorShift64,
        u8::XorShift8,
    };
}
