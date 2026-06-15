// devela/src/ui/mod.rs
//
#![doc = crate::_DOC_UI!()] // public, root
#![doc = crate::_DOC_UI_MODULES!()]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_QUO_UI!()]
//
// safety
#![cfg_attr(feature = "safe_ui", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_UI_MODULES =
    crate::_doc!(modules: crate; ui: event); // intent, layout, view
}

// mod error; // RETHINK
mod intent; // WIP
// #[cfg(feature = "layout")]
mod layout; // WIP
mod route; // WIP
mod sem; // WIP
mod view; // WIP
mod widget; // WIP

pub mod frame; // UiFrame, UiId, UiKey, UiPhase, UiScope
#[cfg(feature = "event")]
pub mod event; // Event[Button[State]|Key[State]|Kind|Mouse|Pointer[Type]|TimeStamp|Wheel], Key*

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            // error::*,
            intent::*,
            layout::*,
            route::*,
            sem::*,
            view::*,
            widget::*,
        };
    }
    _pub_mods {
        pub use super::{
            frame::*,
        };
        #[cfg(feature = "event")]
        pub use super::event::_all::*;
        // #[cfg(feature = "layout")]
        // pub use super::layout::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_UI_MODULES;
    }
}
