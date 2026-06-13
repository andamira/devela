// devela/src/num/prob/rand/prng/xoroshiro/mod.rs
//
//! Pseudo-random number generators based on [Xoroxhiro].
//!
//! [Xoroshiro]: https://en.wikipedia.org/wiki/Xorshift#xoroshiro
//

mod u128;
pub use u128::Xoroshiro128pp;
