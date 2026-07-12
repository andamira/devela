// devela/src/ui/sem/mod.rs
//
#![doc = crate::_DOC_UI_SEMANTIC!()] // public
#![doc = crate::_doc!(modules: crate::ui; semantic)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//!
//! Semantic records describe what a resolved UI identity represents,
//! which operations it exposes, and the human-facing state associated with it.
//!
//! [`UiEntry`] combines three independent dimensions:
//! - [`UiRole`] classifies what the identity represents.
//! - [`UiActions`] describes the operations it exposes.
//! - [`UiFlags`] qualifies its current semantic state.
//!
//! [`UiText`] separately associates a label and description with an identity.
//! This keeps compact structural records independent of text ownership and storage.
//!
//! ```text
//! UiId ── associates ──┬── UiEntry { UiRole, UiActions, UiFlags }
//!                      └── UiText  { label, description }
//! ```
//!
//! - Actions describe available operations, not events or commands that have already occurred.
//! - Flags describe exposed semantic state, not transient routing ownership.
//!
//! # Boundaries
//!
//! This module does not determine geometry, styling,
//! hit testing, input mappings, or backend rendering.
//!
//! - Identity comes from [`ui::frame`][crate::ui::frame].
//! - Current interaction ownership belongs to [`ui::route`][crate::ui::route].
//! - Concrete presentation belongs to [`ui::view`][crate::ui::view].
//!
//! No semantic tree, traversal order, or storage model is implied by these records.
//

#[cfg(test)]
mod _test;

mod action; // UiAction, UiActions
mod entry; // UiEntry
mod flags; // UiFlags
mod role; // UiRole
mod text; // UiText

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            action::*,
            entry::*,
            flags::*,
            role::*,
            text::*,
        };
    }
}
