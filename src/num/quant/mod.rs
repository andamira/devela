// devela::num::quant
//
//! Quantification and periodicity.
//!
//! This module provides general concepts for measuring and quantifying values,
//! including both discrete and continuous representations, without being tied
//! to specific interpretations.
//

mod cycle; // Ćycle, CycleCount
mod interval; // Interval

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{cycle::*, interval::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
