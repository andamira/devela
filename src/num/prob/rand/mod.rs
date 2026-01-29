// devela::num::prob::rand
//
#![doc = crate::_DOC_NUM_PROB_RAND!()]
//!
//! This module defines several types:
//! - RNG algorithms specialized for 8-bit devices:
//!   [`Xabc`], [`Xyza8a`], [`Xyza8b`].
//! - Classic *XorShift* algorithms and variations with a smaller state.
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
//! All <abbr title="Pseudo-Random Number Generator">PRNG</abbr>s require the
//! `rand` feature, except for [`XorShift128p`], which is always compiled.
//

// mod from_rand; // FromRand
// mod noise; // Structured deterministic randomness

crate::structural_mods! { // _mods, _reexports
    _mods {
        // pub use super::{
        //     from_rand::*,
        //     noise::*,
        // };
    }
    _reexports {
        pub use devela_base_core::num::prob::rand::{
            Rand,
            XorShift128p,
        };
        #[doc(inline)]
        #[cfg(feature = "rand")]
        pub use devela_base_core::num::prob::rand::{
            Lgc16, Xabc, Xyza8a, Xyza8b, Xoroshiro128pp,
            XorShift8, XorShift16, XorShift32, XorShift64, XorShift128,
            xorshift_custom,
        };
        #[cfg(feature = "alloc")]
        pub use devela_base_alloc::num::prob::rand::RandAlloc;
        #[cfg(feature = "std")]
        pub use devela_base_std::num::prob::rand::RandStd;
    }
}
