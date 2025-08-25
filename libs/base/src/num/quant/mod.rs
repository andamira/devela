// devela_base::num::quant
//
#![doc = crate::_DOC_NUM!()]
//!
//! This module provides general concepts for measuring and quantifying values,
//! including both discrete and continuous representations, without being tied
//! to specific interpretations.
//!
//! More specifically it defines foundational types for:
//! - Periodic phenomena [`Cycle`], [`CycleCount`].
//! - Range and interval mathematics: [`Interval`], [`interval!`].
//! - Proportional relationships and ratios: [`Ratio`].
// - Scaling transformations: [`Scale`].
//! - Numerical classification and properties: [`Sign`].
// - Discrete counting and increments: [`Count`].
//

mod cycle; // Cycle, CycleCount
mod interval; // Interval, interval!
// mod ratio; // Ratio
mod sign; // Sign

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{cycle::*, interval::*, sign::*};
        // WIPZONE
        // pub use super::counter::*;
        // pub use super::norm::*;
        // pub use super::ratio::*;
        // pub use super::scale::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// mod counter;
// mod norm;
// mod scale;
