// devela_base_core::num::prob::rand::xoroshiro
//
//! Pseudo-random number generators based on [Xoroxhiro].
//!
//! [Xoroshiro]: https://en.wikipedia.org/wiki/Xorshift#xoroshiro
//

mod u128;
pub use u128::Xoroshiro128pp;
