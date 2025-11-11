// devela_base_core::num::geom
//
#![doc = crate::_DOC_NUM_GEOM!()]
//

pub mod metric; // Distance, Extent, Orientation, Position...

crate::structural_mods! { // _pub_mods
    _pub_mods {
        #[doc(inline)]
        pub use super::metric::_all::*;
    }
    _workspace_internals {
        pub use super::metric::_workspace_internals::*;
    }
}
