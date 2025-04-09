// devela::num::quant
//
//! Quantification and periodicity.
//!
//! This module provides general concepts for measuring and quantifying values,
//! including both discrete and continuous representations, without being tied
//! to specific interpretations.
//

mod cycle; // Cycle, CycleCount
mod interval; // Interval
mod ratio; // Ratio

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{cycle::*, interval::*, ratio::*};
        // WIPZONE
        // pub use super::counter::*;
        // pub use super::scale::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// mod counter;
// mod scale;
