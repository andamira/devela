// devela/src/ui/event/mod.rs
//
#![doc = crate::_DOC_UI_EVENT!()] // public
#![doc = crate::_doc!(modules: crate::ui; event)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//!
//! This module defines normalized event values shared across host backends,
//! runtimes, and higher-level UI processing.
//!
//! [`Event`] combines a conceptual [`EventTarget`], a typed [`EventKind`],
//! and optional timing or update metadata.
//!
//! ```text
//! backend event
//!   │ normalize
//!   ▼
//! Event
//! ├── EventTarget ── Global | Window | Device
//! ├── EventKind   ── tag ──> EventTag
//! │   ├── Key
//! │   ├── Mouse | Pointer | Wheel
//! │   ├── Window
//! │   └── Control
//! ├── emitted
//! ├── processed
//! └── count
//!
//! EventQueue<CAP> ── buffers Event values in FIFO order
//! ```
//!
//! [`EventKind`] preserves each event's payload, while [`EventTag`] provides
//! a data-less category for filtering and dispatch.
//!
//! Keyboard events distinguish the interpreted [`EventKey::semantic`] key from
//! its [`EventKey::physical`] origin. Backends may provide either or both,
//! depending on their capabilities.
//!
//! Event metadata separates three notions of occurrence:
//!
//! - `emitted` records when the backend generated the event.
//! - `processed` records when it entered application processing.
//! - `count` records the logical update iteration in which it was observed.
//!
//! [`EventQueue`] provides fixed-capacity, allocation-free FIFO buffering for
//! normalized events.
//!
//! # Boundaries
//!
//! Native event formats are translated inward by host implementations such as
//! those under [`sys`][crate::sys]. This module defines their normalized
//! representation, not the platform-specific acquisition mechanism.
//!
//! [`EventTarget`] identifies a window, device, or global destination. It does
//! not identify a widget or other [`UiId`][crate::UiId]; identity-level
//! interaction belongs to [`ui::route`][crate::ui::route].
//!
//! This module does not define widget behavior, hit testing, focus policy,
//! gesture recognition, event propagation, polling cadence, or an application
//! loop. Those policies belong to routing, controls, or runtime orchestration.
//
//

mod event; // Event
mod id; // DeviceId, WindowId
mod key; // EventKey[Ffi], Key[Ffi|Media|Mod|Mods|Pad|State]
mod kind; // EventKind, EventKindTimed, EventTag
mod pointer; // Event[Button[State]|Mouse|Pointer[Type]|Wheel]
mod queue; // EventQueue
mod target; // EventTarget
mod time; // EventTimestamp[Mode]
mod window; // EventWindow

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            event::*,
            id::*,
            key::*,
            kind::*,
            pointer::*,
            queue::*,
            target::*,
            time::*,
            window::*,
        };
    }
}
