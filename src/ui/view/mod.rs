// devela/src/ui/view/mod.rs
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_UI_VIEW!()] // public
#![doc = crate::_doc!(modules: crate::ui; view)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//!
//! This module describes how logical UI state is prepared for presentation.
//! It provides frame-local visual records, drawing instructions, broad output
//! forms, and measures for projecting logical space into concrete units.
//!
//! [`UiView`] associates a resolved [`UiId`] with its [`UiRect`],
//! [`UiLayer`], and [`UiViewFlags`].
//!
//! ```text
//! UiView
//! ├── UiId
//! ├── UiRect
//! ├── UiLayer
//! └── UiViewFlags ── HIDDEN | CLIPPED | OPAQUE
//!
//! UiDrawList ── borrowed as ──> UiDrawListView
//!
//! logical layout
//! ├── UiDensity + UiRound ──> physical pixels
//! └── UiCellMetric ─────────> discrete cells
//! ```
//!
//! [`UiViewForm`] classifies broad presentation families without selecting a backend:
//!
//! ```text
//! UiViewForm
//! ├── Cell
//! ├── Graphic
//! ├── Document
//! └── Message
//! ```
//!
//! These forms identify materially different ways to present UI state.
//! They do not require every application to support every form, nor
//! do they prescribe one common rendering strategy for all of them.
//!
//! This module remains incomplete. Its current types establish shared
//! visual records and projection measures without fixing how complete
//! views, form-specific profiles, or backend presenters must be assembled.
//!
//! # Boundaries
//!
//! - Resolved identity comes from [`ui::frame`][crate::ui::frame].
//! - Logical regions come from [`ui::layout`][crate::ui::layout].
//! - Semantic meaning belongs to [`ui::semantic`][crate::ui::semantic].
//! - General drawing and media resources belong to [`media`][crate::media].
//! - Concrete host surfaces and backend presentation belong at system and backend boundaries.
//!
//! This implies no retained view tree, scene graph, styling system,
//! universal draw model, or fixed rendering pipeline.
//!
//! [`UiId`]: crate::UiId
//! [`UiRect`]: crate::UiRect
//

#[cfg(test)]
mod _test;

// mod cue; // Interactive visual cues for view presentation
mod draw; // UiDraw, UiDrawList, UiDrawListView
mod form; // WIP UiViewForm: cell, document, graphic and message projection forms
mod layer; // UiLayer
mod profile; // WIP Presentation profiles for fitting logical views into output space
mod scale; // Pixel, density, and text scaling units for view projection
mod view; // UiView, UiViewFlags

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // cue::_all::*,
            draw::_all::*,
            form::_all::*,
            layer::*,
            profile::_all::*,
            scale::_all::*,
            view::*,
        };
    }
}
