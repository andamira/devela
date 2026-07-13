// devela/src/ui/widget/mod.rs
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_UI_WIDGET!()] // public
#![doc = crate::_doc!(modules: crate::ui; widget)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//!
//! Widgets compose UI primitives into semantic controls.
//! They report interaction results for resolved identities.
//!
//! - [`UiResponse`] provides a common per-identity result for controls.
//! - Its [`UiResponseFlags`] distinguish current interaction state
//!   from outcomes produced during the frame.
//!
//! ```text
//! UiResponse
//! ├── UiId
//! └── UiResponseFlags
//!     ├── current state ── HOT | ACTIVE | FOCUSED
//!     └── frame outcome ── ACTIVATED | CHANGED
//! ```
//!
//! The module is intentionally incomplete. [`UiButton`] is its first concrete
//! control, not an exhaustive widget set. Additional controls and shared
//! composition patterns remain to be established.
//!
//! [`UiButton`] carries an author-provided [`UiKey`] with its human-facing
//! label and description. Once resolved, it can produce the corresponding
//! [`UiEntry`] and [`UiText`] semantic records.
//!
//! ```text
//! UiButton
//! ├── UiKey ── resolved by UiFrame ──> UiId
//! ├── label
//! └── description
//!
//! UiButton + UiId ──> UiEntry + UiText
//! ```
//!
//! A response reports the result of interaction processing; it does not
//! perform hit testing or determine ownership.
//!
//! # Boundaries
//!
//! This module composes established UI concepts
//! rather than replacing their respective systems.
//!
//! - Keys are resolved into identities by [`ui::frame`][crate::ui::frame].
//! - Roles, actions, and human-facing text belong to [`ui::semantic`][crate::ui::semantic].
//! - Hit regions and interaction ownership belong to [`ui::route`][crate::ui::route].
//! - Geometry and presentation belong to [`ui::layout`][crate::ui::layout]
//!   and [`ui::view`][crate::ui::view].
//!
//! No widget tree, retained control store, renderer, or event loop is implied.
//!
//! [`UiKey`]: crate::UiKey
//! [`UiEntry`]: crate::UiEntry
//! [`UiText`]: crate::UiText
//

#[cfg(test)]
mod _test;

mod button; // UiButton
mod response; // UiResponse, UiResponseFlags

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            button::*,
            response::*,
        };
    }
}
