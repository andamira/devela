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
    crate::_doc!(modules: crate; ui: event, frame, layout, route, semantic);
}

// mod error;
mod intent; // WIP Desired UI configuration before capability-bound realization
mod view; // Projection of UI state into concrete presentation forms
mod widget; // WIP Semantic controls expressed through frame-local authorship

#[cfg(feature = "event")]
pub mod event; // Normalized interaction and window events entering the UI frame
pub mod frame; // Immediate UI authorship, scoped identity, and retained facts across frames
// #[cfg(feature = "layout")]
pub mod layout; // Canonical spatial vocabulary for deterministic arrangement
pub mod route; // Routing interaction through regions, focus, active state, and capture
pub mod semantic; // Human-facing meaning for roles, actions, descriptions, and navigation

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            // error::*,
            intent::_all::*,
            view::_all::*,
            widget::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            frame::_all::*,
            layout::_all::*,
            route::_all::*,
            semantic::_all::*,
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
