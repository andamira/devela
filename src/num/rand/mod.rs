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

mod lgc;
mod xabc;
mod xoroshiro;
mod xorshift;
mod xyza8;

// structural access
crate::items! {
    mod doc_inline {
        pub use super::{lgc::*, xabc::*, xoroshiro::*, xorshift::*, xyza8::*};
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(crate) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
