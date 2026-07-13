// devela/src/ui/mod.rs
//
#![doc = crate::_DOC_UI!()] // public, root
#![doc = crate::_DOC_UI_MODULES!()]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_QUO_UI!()]
//!
//! This module provides backend-independent vocabulary for interactive
//! surfaces. It separates the questions a UI system must answer so each
//! concern can be used independently or composed with the others.
//!
//! ```text
//! event      What changed at the host-facing boundary?
//! frame      Which identity is being authored, and in which phase?
//! layout     Where is it in logical UI space?
//! route      Which identity owns or may receive interaction?
//! semantic   What does it represent, expose, and communicate?
//! text       How does editable text state change?
//! view       How is UI state prepared for presentation?
//! widget     How are these primitives composed into controls?
//! ```
//!
//! These are neighboring concerns, not a required processing pipeline.
//! A consumer may use only normalized events, text editing, layout vocabulary,
//! or any other subset. No particular widget tree, retained-state store,
//! scheduler, or renderer is required.
//!
//! Resolved [`UiId`] values connect several of these concerns:
//!
//! ```text
//! UiKey + UiScope ──resolve──> UiId
//!
//! UiId + UiRect ───────────────> route and view records
//! UiId + semantic descriptors ─> semantic records
//! ```
//!
//! # Boundaries
//!
//! This module defines interaction-facing state and backend-neutral
//! presentation vocabulary, not a complete application framework.
//!
//! - Host devices, windows, and event acquisition belong to [`sys`][crate::sys].
//!   Backends normalize native events into [`ui::event`][crate::ui::event] values.
//! - Runtime progression, polling cadence, and application orchestration belong
//!   to [`run`][crate::run].
//! - General geometry and text processing belong to [`geom`][crate::geom] and
//!   [`text`][crate::text]. [`ui::layout`][crate::ui::layout] and
//!   [`ui::text`][crate::ui::text] add UI-specific semantics.
//! - Perceivable artifacts and resources belong to [`media`][crate::media].
//!   [`ui::view`][crate::ui::view] projects UI state for presentation.
//
// safety
#![cfg_attr(feature = "safe_ui", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_UI_MODULES =
    crate::_doc!(modules: crate; ui: event, frame, layout, route, semantic, text, view, widget);
}

#[cfg(feature = "ui")]
mod _helper; // (UiNum)

#[cfg(feature = "event")]
pub mod event; // Normalized input, control, and presentation-surface events

#[cfg(feature = "ui")]
mod intent; // WIP Desired UI configuration before capability-bound realization
// mod notice; // WIP Attention requests, alarm discipline, and delivery routing

#[cfg(feature = "ui")]
pub mod frame; // Frame context, scoped identity, and processing phases
#[cfg(feature = "ui")]
pub mod layout; // Canonical spatial vocabulary for deterministic arrangement
#[cfg(feature = "ui")]
pub mod route; // Identity-based interaction regions and ownership state
#[cfg(feature = "ui")]
pub mod semantic; // Human-facing roles, actions, state, and text
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
            // notice::_all::*,
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
        #[cfg(feature = "ui")]
        pub(crate) use super::_helper::*;
    }
}
