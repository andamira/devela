// devela::ui::event::id
//
//! Defines [`DeviceId`], [`WindowId`].
//

#[cfg(doc)]
use crate::EventTarget;

crate::_impl_init![ConstInit: Self(0) => WindowId, DeviceId];

#[doc = crate::_tags!(ui uid)]
/// A backend-agnostic identifier for a UI window.
#[doc = crate::_doc_location!("ui/event")]
///
/// Backends (X11, Wayland, Win32, macOS, terminal, web…) map their native
/// window or surface handles into a compact, stable `WindowId`.
///
/// This decouples the UI event system from backend-specific handle types
/// (`xcb_window_t`, `HWND`, `NSWindow*`, browser document, terminal view…).
///
/// Events originating from these sources use [`EventTarget`]`::Window(WindowId)`.
///
/// The value is treated as an opaque token. It is not an index into user
/// data structures, and it has no arithmetic meaning.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(pub u32);

#[doc = crate::_tags!(interaction uid)]
/// A backend-agnostic identifier for an input device.
#[doc = crate::_doc_location!("ui/event")]
///
/// Backends map native device handles (libinput device, MIDI port, gamepad,
/// tablet, virtual input stream…) into a stable `DeviceId`.
///
/// Events originating from these sources use [`EventTarget`]`::Device(DeviceId)`.
///
/// The value is treated as an opaque token. It is not an index into user
/// data structures, and it has no arithmetic meaning.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId(pub u32);

// /// Maps backend-native window handles to stable [`WindowId`]s.
// pub type WindowRegistry<const MAX: usize> = IdRegistry<WindowId, MAX>;
// /// Maps backend-native window handles to stable [`DeviceId`]s.
// pub type DeviceRegistry<const MAX: usize> = IdRegistry<DeviceId, MAX>;
