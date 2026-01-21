// devela_base_core::geom
//
#![doc = crate::_DOC_GEOM!()]
#![doc = crate::_DOC_GEOM_MODULES!()]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(newline)]
//
// safety
#![cfg_attr(base_safe_num, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_GEOM_MODULES =
    crate::_doc!(modules: crate; geom: dir, metric); // field, linear, shape
}

mod _helpers;

pub mod dir; // Orientation
// pub mod field; // WIP
pub mod metric; // Distance, Extent, Orientation, Position...

crate::structural_mods! { // _pub_mods, _crate_internals, _workspace_internals
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            dir::_all::*,
            // field::_all::*, // WIP
            metric::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_GEOM_MODULES;
    }
    _workspace_internals {
        pub use super::_helpers::*;
    }
}
