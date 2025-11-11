// devela_base_core::num::geom::metric
//
#![doc = crate::_DOC_NUM_GEOM_METRIC!()]
//

mod helpers; // _impl_metric!

mod distance; // Distance
mod extent; // Extent
mod orientation; // Orientation
mod position; // Position
mod region; // Region
mod stride; // Stride

// WIPZONE
// mod _wip_cycle;
// mod _wip_radial_sectors;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            distance::*,
            extent::*,
            orientation::*,
            position::*,
            region::*,
            stride::*,
        };

        // WIPZONE
        // pub use super::_wip_cycle::*;
        // pub use super::radial_sectors::*;
    }
    _workspace_internals {
        pub use super::helpers::*;
    }
}
