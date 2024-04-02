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

/* always compiled */

mod xabc;
mod xorshift;
mod xyza8;
pub use {xabc::*, xorshift::*, xyza8::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{xabc::*, xorshift::*, xyza8::*};
}
