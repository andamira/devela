// devela_base_core::ui
//
#![doc = crate::_DOC_UI!()] // public, root
#![doc = crate::_DOC_UI_MODULES!()]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(base_safe_ui, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_UI_MODULES =
    crate::_doc!(modules: crate; ui: front); // back, layout
}

pub mod front;
// pub mod layout; // WIP

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            front::_all::*,
            // layout::_all::*, // WIP
        };
    }
    _crate_internals {
        pub(crate) use super::{
            _DOC_UI_MODULES,
            front::_crate_internals::*,
        };
    }
}
