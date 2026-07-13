// devela/src/ui/route/mod.rs
//
#![doc = crate::_DOC_UI_ROUTE!()] // public
#![doc = crate::_doc!(modules: crate::ui; route)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//!
//! [`HitRegion`] associates a resolved [`UiId`] with the [`UiRect`] that may
//! participate in hit testing. The remaining types represent distinct forms
//! of current interaction ownership or candidacy.
//!
//! ```text
//! HitRegion
//! ├── UiId
//! └── UiRect
//!
//! routing state (each stores an optional UiId)
//! ├── RouteHot       direct pointing candidate
//! ├── RouteActive    ongoing interaction owner
//! ├── RouteFocus     keyboard or navigation owner
//! └── RouteCapture   pointer-capture owner
//! ```
//!
//! These states are related but not interchangeable. An identity may be hot
//! without being active, focused without holding pointer capture, or active
//! after the pointer has moved away from its original region. Separate wrapper
//! types preserve those distinctions even though their current representations
//! are similarly compact.
//!
//! # Boundaries
//!
//! - Identity is supplied by [`ui::frame`][crate::ui::frame].
//! - Region geometry is supplied by [`ui::layout`][crate::ui::layout].
//!
//! This module associates those facts with interaction state;
//! it does not prescribe hit ordering, overlap resolution,
//! event propagation, gesture recognition, or a particular routing algorithm.
//!
//! [`RouteFocus`] records current focus ownership. The corresponding
//! semantic action is [`UiAction::Focus`][crate::UiAction::Focus].
//!
//! [`UiId`]: crate::UiId
//! [`UiRect`]: crate::UiRect
//

#[cfg(test)]
mod _test;

mod hit; // HitRegion
mod state; // RouteActive, RouteCapture, RouteFocus, RouteHot

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            hit::*,
            state::*,
        };
    }
}
