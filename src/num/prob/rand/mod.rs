// devela::num::prob::rand
//
#![doc = crate::_DOC_NUM_PROB_RAND!()] // public
#![doc = crate::_doc!(modules: crate::num::prob; rand)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//! Random sources, value construction, qualities, and pseudo-random generators.
//!
//! # Traits
//!
//! - [`RandTry`] and [`Rand`] define fallible and infallible random sources.
//! - [`RandQualities`] describes source behavior and suitability.
// - [`RandSeedable`] and [`RandSeedableTry`] construct generators from explicit
//   seed material or another random source.
//! - [`FromRandTry`] and [`FromRand`] construct random values.
//!
//! # Canonical generators
//!
//! Devela keeps two complementary PRNGs always available:
//! - [`Pcg32`] is the canonical general-purpose generator.
//! - [`SplitMix64`] is the canonical seed mixer and expander,
//!   and also provides a compact 64-bit random stream.
//!
//! Other generators cover specialized, small-state, legacy, and comparative uses.
//! Their [`RandQualities`] indicate known limitations and suitability.
//!
//! All PRNG types implement [`Copy`]. Copying a PRNG duplicates its current
//! state, so both copies produce the same stream until advanced differently.
//!
//! Some generators are retained for small-device, legacy, educational, or
//! comparative uses despite known generator-side limitations. These are
//! classified with [`RandQualities::WEAK_PRNG`].
//!
//! `Default` and [`ConstInit`][crate::ConstInit] use fixed seeds and are therefore
//! reproducible. Use explicit or externally sourced seed material when distinct
//! streams are required.
//!
//! # Seed representation
//!
//! Byte-backed seeds interpret multi-byte values in little-endian order.
//! This keeps equal seed bytes reproducible across target endianness.
//!
//! # Features
//!
//! - [`Pcg32`] and [`SplitMix64`] are always available.
//! - The `rand` feature enables the extended PRNG palette and generator macros.
//! - The `std` feature enables [`StdRand`].
//

mod _helper; // (_impl_dep_rand_core)

mod from; // FromRandTry, FromRand
// mod noise; // Structured deterministic randomness
mod prng; // concrete PRNGs
mod rand; // RandSeedable, RandTry, Rand
mod qual; // RandQualities

#[cfg(feature = "std")]
mod std; // StdRand

crate::structural_mods! { // _mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            from::*,
            // noise::*,
            prng::_all::*,
            rand::*,
            qual::*,
        };
        #[cfg(feature = "std")]
        pub use super::std::StdRand;
    }
    _crate_internals {
        pub(crate) use super::{
            prng::_crate_internals::*,
        };
    }
    _hidden {
        #[doc(hidden)]
        pub(crate) use super::{
            _helper::*,
            prng::_hidden::*,
        };
    }
}
