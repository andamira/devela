// devela/src/ui/frame/mod.rs
//
#![doc = crate::_DOC_UI_FRAME!()] // public
#![doc = crate::_doc!(modules: crate::ui; frame)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//!
//! [`UiFrame`] carries the current [`UiScope`] and [`UiPhase`].
//!
//! UI authors provide [`UiKey`] values, which the current scope
//! resolves into [`UiId`] values used by neighboring UI systems.
//!
//! ```text
//! UiFrame
//! ├── UiScope + UiKey ──resolve──> UiId
//! └── UiPhase
//! ```
//!
//! A key is an author-provided identity seed, while an id is its resolved
//! identity. The same key may be reused in different scopes and resolve to
//! distinct ids, avoiding the need for one globally coordinated key space.
//!
//! A phase identifies the current kind of frame processing. It does not
//! prescribe a scheduler or require to perform every phase.
//!
//! # Boundaries
//!
//! This module establishes frame context and identity. It does not define
//! layout, interaction ownership, meaning, or presentation.
//!
//! - Layout geometry comes from [`ui::layout`][crate::ui::layout].
//! - Current interaction ownership belongs to [`ui::route`][crate::ui::route].
//! - Semantic meaning comes from [`ui::semantic`][crate::ui::semantic].
//! - Concrete presentation belongs to [`ui::view`][crate::ui::view].
//!
//! `UiFrame` is a small context value. No particular storage model for
//! longer-lived UI memory, caches, or collected output is implied.
//

mod id; // UiId, UiKey, UiScope
mod frame; // UiFrame, UiPhase
// mod mem; // UiMemory
// mod cache; // UiCache
mod output; // UiOutput

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            id::*,
            frame::*,
            output::*,
        };
    }
}
