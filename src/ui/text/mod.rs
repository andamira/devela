// devela/src/ui/text/mod.rs
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_UI_TEXT!()] // public
#![doc = crate::_doc!(modules: crate::ui; text)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//!
//! [`TextInput`] models one-line UTF-8 editing over caller-chosen storage.
//! It keeps the initialized length, cursor position, and [`TextInputConfig`],
//! while its storage type determines ownership and capacity.
//!
//! ```text
//! EventKey ── TextInputKeymap ──> TextInputAction
//!                                      │
//!                                      ▼ apply
//! TextInput<B> ─────────────────> TextInputOutcome
//!      │                               └── Rejected(TextInputReject)
//!      └── borrow ──────────────> TextInputView
//! ```
//!
//! [`TextInputAction`] expresses storage-independent editing requests.
//! Applying one produces a [`TextInputOutcome`] that distinguishes unchanged
//! state, mutation, acceptance, cancellation, and rejection.
//!
//! [`TextInputView`] exposes the initialized text and cursor without exposing
//! its storage representation. The cursor is a byte position on a UTF-8
//! boundary; character movement and deletion operate on Unicode scalar
//! boundaries.
//!
//! When event support is enabled, [`TextInputKeymap`] translates normalized
//! keyboard events into editing actions. Key mapping remains separate from the
//! editing state itself.
//!
//! # Boundaries
//!
//! This module owns editable text state and its explicit transitions.
//!
//! - General encoding, segmentation, and text processing belong to [`text`][crate::text].
//! - Normalized keyboard events come from [`ui::event`][crate::ui::event].
//! - Keyboard focus and interaction ownership belong to [`ui::route`][crate::ui::route].
//! - Control composition and presentation belong to [`ui::widget`][crate::ui::widget]
//!   and [`ui::view`][crate::ui::view].
//!
//! It does not define focus acquisition, selection, composition, multiline layout, or rendering.
//

mod input; // TextInput[Action|Config|Outcome|Reject|View]

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            input::_all::*,
        };
    }
}
