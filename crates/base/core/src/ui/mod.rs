// devela_base_core::ui
//
#![doc = crate::_DOC_UI!()] // public, root
#![doc = crate::_DOC_UI_MODULES!()]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_ui", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_UI_MODULES =
    crate::_doc!(modules: crate; ui); // event, layout, view
}

// mod view; // stateful interactive projections WIP

// pub mod event; // WIP
// pub mod layout; // WIP

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        // pub use super::{
        //     view::_all::*,
        // };
    }
    _pub_mods {
        // pub use super::{
        //     event::_all::*, // WIP
        //     layout::_all::*, // WIP
        // };
    }
    _crate_internals {
        pub(crate) use super::{
            _DOC_UI_MODULES,
        };
    }
}
