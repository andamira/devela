// devela::num::prob::rand
//
#![doc = crate::_DOC_NUM_PROB_RAND!()] // public
#![doc = crate::_doc!(modules: crate::num::prob; rand)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//! This module defines several types:
//! - RNG algorithms specialized for 8-bit devices:
//!   [`Xabc`], [`Xyza8a`], [`Xyza8b`].
//! - [`Lcg16`] 16-bit Linear Congruential Generator.
//! - Classic *XorShift* algorithms and variations with a smaller state.
//!   - [`XorShift8`], [`XorShift16`], [`XorShift32`], [`XorShift64`],
//!     [`XorShift128`], [`XorShift128p`].
//!   - [`Xoroshiro128pp`].
//!   - [`rand_xorshift!`].
//! - *Permuted Congruential Generator* algorithms:
//!     - [`Pcg32`].
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
#![doc = concat!["All ", crate::_ABBR_PRNG!(), "s require the `rand` feature,"]]
//! except for [`Pcg32`] and [`XorShift128p`], which are always compiled.
//

// mod from_rand; // FromRand
// mod noise; // Structured deterministic randomness
mod prng; // concrete PRNGs
mod rand; // Rand

#[cfg(feature = "alloc")]
mod rand_alloc; // RandAlloc
#[cfg(feature = "std")]
mod rand_std; // RandStd

crate::structural_mods! { // _mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            // from_rand::*,
            // noise::*,
            prng::_all::*,
            rand::*,
        };
        #[cfg(feature = "alloc")]
        pub use super::rand_alloc::RandAlloc;
        #[cfg(feature = "std")]
        pub use super::rand_std::RandStd;
    }
    _crate_internals {
        pub(crate) use super::prng::_crate_internals::*;
    }
    _hidden {
        pub(crate) use super::prng::_hidden::*;
    }
}
