// devela::num::rand
//
//! Random number generation.
//!
//! This module defines several types:
//! - RNG algorithms specialized for 8-bit devices:
//!   [`Xabc`], [`Xyza8a`], [`Xyza8b`].
//! - Classic *XorShift* algorithms and variations with a smaller state.
//

/* always compiled, non-public modules */

mod xabc;
mod xorshift;
mod xyza8;

pub use {xabc::*, xorshift::*, xyza8::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{xabc::*, xorshift::*, xyza8::*};
}
