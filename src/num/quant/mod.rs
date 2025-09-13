// devela::num::quant
//
#![doc = crate::_DOC_NUM_QUANT!()]
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

mod ratio; // Ratio

crate::structural_mods! { // _mods
    _mods {
        pub use super::ratio::*;

        #[doc(inline)]
        #[rustfmt::skip]
        pub use devela_base_core::num::{
            Cycle, CycleCount,
            Interval, interval,
            Sign,
        };
        pub use devela_base_num::ValueQuant;
    }
}
