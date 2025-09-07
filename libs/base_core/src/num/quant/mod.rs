// devela_base_core::num::quant
//
#![doc = crate::_DOC_NUM_QUANT!()]
//!
//! This module provides general concepts for measuring and quantifying values,
//! including both discrete and continuous representations, without being tied
//! to specific interpretations.
//!
//! More specifically it defines foundational types for:
// - Discrete counting and increments: [`Count`].
//! - Periodic phenomena [`Cycle`], [`CycleCount`].
// - Proportional relationships and ratios: [`Ratio`].
//! - Range and interval mathematics: [`Interval`], [`interval!`].
// - Scaling transformations: [`Scale`].
//! - Numerical classification and properties: [`Sign`].
//!
#![doc = crate::doclink!(custom devela "[`interval!`]" "num/quant/macro.interval.html")]
//

mod cycle; // Cycle, CycleCount
mod interval; // Interval, interval!
// mod ratio; // Ratio
mod sign; // Sign

// WIPZONE
// mod _wip_counter;
// mod _wip_norm;
// mod _wip_scale;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{cycle::*, interval::*, sign::*};
        // WIPZONE
        // pub use super::_wip_counter::*;
        // pub use super::_wip_norm::*;
        // pub use super::_wip_scale::*;
    }
}
