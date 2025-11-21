// devela_base_core::num::geom
//
#![doc = crate::_DOC_NUM_GEOM!()]
//

mod _helpers;

pub mod dir; // Orientation
pub mod metric; // Distance, Extent, Orientation, Position...

crate::structural_mods! { // _pub_mods
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            dir::_all::*,
            metric::_all::*,
        };
    }
    _workspace_internals {
        pub use super::_helpers::*;
    }
}
