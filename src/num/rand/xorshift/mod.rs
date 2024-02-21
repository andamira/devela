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
//! [Xorshift]: https://en.wikipedia.org/wiki/Xorshift
//

mod u128;
mod u16;
mod u32;
mod u64;
mod u8;

pub use u128::{XorShift128, XorShift128p};
pub use u16::XorShift16;
pub use u32::XorShift32;
pub use u64::XorShift64;
pub use u8::{XorShift8, XorShift8Custom};
