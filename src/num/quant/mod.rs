// devela::num::quant
//
#![doc = crate::_DOC_NUM_QUANT!()] // public
#![doc = crate::_doc!(modules: crate::num; quant)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
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

// mod align; // Align
// mod cont;
mod cycle; // Cycle, CycleCount
mod interval; // Interval, interval!
// mod norm; // Norm
// mod ratio; // Ratio
// mod power; // Log, Power, Root
mod ratio; // Ratio
// mod scale; // Scale
mod sign; // Sign
mod value; // ValueQuant

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // align::*,
            // cont::_all::*,
            cycle::*,
            interval::*,
            // norm::*,
            // power::*,
            ratio::*,
            // scale::*,
            sign::*,
            value::*,
        };
    }
}
