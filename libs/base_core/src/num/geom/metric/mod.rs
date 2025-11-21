// devela_base_core::num::geom::metric
//
#![doc = crate::_DOC_NUM_GEOM_METRIC!()]
//

mod distance; // Distance
mod extent; // Extent
mod position; // Position
mod region; // Region
mod stride; // Stride

// WIPZONE
// mod _wip_cycle; // CycleOffset, Spacing

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            distance::*,
            extent::*,
            position::*,
            region::*,
            stride::*,
        };

        // WIPZONE
        // pub use super::_wip_cycle::*;
    }
}
