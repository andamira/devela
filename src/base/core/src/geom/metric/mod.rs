// devela_base_core::geom::metric
//
#![doc = crate::_DOC_GEOM_METRIC!()] // public
#![doc = crate::_doc!(modules: crate::geom; metric)]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//

// mod axes; // TODO
// mod cycle; // CycleOffset, Spacing // MAYBE
mod distance; // Distance
mod extent; // Extent
mod position; // Position
mod region; // Region
mod stride; // Stride

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            // axes::*, // TODO
            // cycle::*, // MAYBE
            distance::*,
            extent::*,
            position::*,
            region::*,
            stride::*,
        };
    }
}
