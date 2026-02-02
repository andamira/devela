// devela_base_core::geom::dir
//
#![doc = crate::_DOC_GEOM_DIR!()] // public
#![doc = crate::_doc!(modules: crate::geom; dir)]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//

mod boundary; // Boundary1d, Boundary2d, Boundary3d
// mod nav; // Spatial navigation and facing semantics.
// mod octant; // WIP
mod orientation; // Orientation
// mod radial_sectors;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            boundary::*,
            // nav::_all::*, // WIP
            // octant::*, // WIP
            orientation::*,
            // radial_sectors::*;
        };
    }
}
