// devela_base_core::num::quant
//
#![doc = crate::_DOC_NUM_QUANT!()]
//!
#![doc = crate::_doc!(flat:"num")]
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
//

// mod align; // Align
// mod cont;
mod cycle; // Cycle, CycleCount
mod interval; // Interval, interval!
// mod norm; // Norm
// mod ratio; // Ratio
// mod power; // Log, Power, Root, TriPow
// mod scale; // Scale
mod sign; // Sign
mod value; // ValueQuant

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // align::*,
            // cont::*,
            cycle::*,
            interval::*,
            // norm::*,
            // scale::*,
            sign::*,
            value::*,
        };
    }
}
