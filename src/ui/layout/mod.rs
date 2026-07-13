// devela/src/ui/layout/mod.rs
//
#![doc = crate::_DOC_UI_LAYOUT!()] // public
#![doc = crate::_doc!(modules: crate::ui; layout)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//!
//! Layout types express positions, extents, regions, and spatial assignment
//! in a backend-independent logical space.
//!
//! [`Lunit`] is the fixed-point scalar underlying the module's spatial types.
//! The [`UiPos`], [`UiExt`], [`UiStride`], and [`UiRect`] aliases specialize
//! general geometric types for UI layout.
//!
//! ```text
//! Lunit ── specializes ──┬── UiPos
//!                        ├── UiExt
//!                        ├── UiStride
//!                        └── UiRect
//!
//! UiRect ── consumed incrementally by ──> UiStack
//!
//! available + used  ── recorded by ──> Layout1d
//! UiId + UiRect     ── recorded by ──> LayoutReceipt
//! ```
//!
//! - [`UiRect`] provides deterministic region operations
//!   such as insetting and cutting from individual sides.
//! - [`UiStack`] builds on those operations as a cursor-like helper that
//!   takes consecutive regions while retaining the unassigned remainder.
//! - [`Layout1d`] records available and consumed space along one axis.
//! - [`LayoutReceipt`] associates a resolved identity with its assigned region.
//!
//! # Boundaries
//!
//! This module assigns space in logical UI coordinates.
//!
//! - Resolved identity comes from [`ui::frame`][crate::ui::frame].
//! - General spatial forms come from [`geom::metric`][crate::geom::metric].
//! - Interaction routing may consume regions through [`ui::route`][crate::ui::route].
//! - Projection into terminal cells, physical pixels, or other output units
//!   belongs to [`ui::view`][crate::ui::view] and its consumers.
//!
//! These types do not imply a layout tree, constraint solver, styling system,
//! intrinsic measurement protocol, or retained layout engine.
//

#[cfg(test)]
mod _test;

mod metric; // aliases: Ui<Ext|Pos|Rect|Stride|>
// mod partition; // UiPartition, UiRemainder
mod receipt; // Layout1d, LayoutReceipt
mod stack; // UiStack
mod unit; // Lunit

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            metric::*,
            // partition::*,
            receipt::*,
            stack::*,
            unit::*,
        };
    }
}
