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
    crate::_doc!(modules: crate; ui: event, frame, layout, route, semantic, text, view, widget);
}

#[cfg(feature = "event")]
pub mod event; // Normalized interaction and window events entering the UI frame

#[cfg(feature = "ui")]
mod intent; // WIP Desired UI configuration before capability-bound realization

#[cfg(feature = "ui")]
pub mod frame; // Immediate UI authorship, scoped identity, and retained facts across frames
#[cfg(feature = "ui")]
pub mod layout; // Canonical spatial vocabulary for deterministic arrangement
#[cfg(feature = "ui")]
pub mod route; // Routing interaction through regions, focus, active state, and capture
#[cfg(feature = "ui")]
pub mod semantic; // Human-facing meaning for roles, actions, descriptions, and navigation
#[cfg(feature = "ui")]
pub mod text; // Interactive text editing state.
#[cfg(feature = "ui")]
pub mod view; // Projection of UI state into concrete presentation forms

#[cfg(feature = "widget")]
pub mod widget; // Semantic controls expressed through frame-local authorship

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        #[cfg(feature = "ui")]
        pub use super::{
            intent::_all::*,
        };
    }
    _pub_mods {
        #[cfg(feature = "ui")]
        pub use super::{
            frame::_all::*,
            layout::_all::*,
            route::_all::*,
            semantic::_all::*,
            text::_all::*,
            view::_all::*,
        };
        #[cfg(feature = "event")]
        pub use super::event::_all::*;
        #[cfg(feature = "widget")]
        pub use super::widget::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_UI_MODULES;
    }
}
