// devela_base_core::num::geom::dir
//
#![doc = crate::_DOC_NUM_GEOM_DIR!()]
//

mod boundary; // Boundary1d, Boundary2d, Boundary3d
// mod octant; // WIP
mod orientation; // Orientation
// mod radial_sectors;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            boundary::*,
            // octant::*, // WIP
            orientation::*,
            // radial_sectors::*;
        };
    }
}
