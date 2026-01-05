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

// mod align; // Align WIP
// mod counter; // Counter
mod cycle; // Cycle, CycleCount
mod interval; // Interval, interval!
// mod norm; // Norm
// mod ratio; // Ratio
// mod scale; // Scale
mod sign; // Sign

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // align::*, WIP
            // counter::*,
            cycle::*,
            interval::*,
            // norm::*,
            // scale::*,
            sign::*,
        };
    }
}
