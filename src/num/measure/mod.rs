// devela::num::measure
//
//! Foundational abstractions for measurement and periodicity.
//!
//! This module provides general measurement concepts,
//! that aren't tied to specific spatial interpretations.
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
