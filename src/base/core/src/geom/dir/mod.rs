// devela_base_core::geom::dir
//
#![doc = crate::_DOC_GEOM_DIR!()] // public
#![doc = crate::_doc!(modules: crate::geom; dir)]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//

mod boundary; // Boundary1d, Boundary2d, Boundary3d
// mod octant; // WIP
mod orientation; // Orientation
// mod nav; // WIP
// mod radial_sectors;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            boundary::*,
            // octant::*, // WIP
            orientation::*,
            // nav::_all::*, // WIP
            // radial_sectors::*;
        };
    }
}
