// devela_base_core::num::geom::dir
//
#![doc = crate::_DOC_NUM_GEOM_METRIC!()]
//

mod orientation; // Orientation
// mod radial_sectors;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            orientation::*,
            // radial_sectors::*;
        };
    }
}
