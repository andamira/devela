// devela::ui::event::target
//
//! Defines [`EventTarget`].
//

use crate::{_impl_init, DeviceId, WindowId};

_impl_init![ConstInit: Self::Global => EventTarget];

#[doc = crate::_TAG_EVENT!()]
#[doc = crate::_TAG_UID!()]
/// Identifies what an [`Event`][crate::Event] is conceptually directed to.
///
/// - Some backends (X11, Wayland, Win32, macOS) generate window–scoped events.
///   These are represented as [`EventTarget::Window`], carrying a window
///   identifier meaningful to the backend.
///
/// - Device–level sources (gamepads, MIDI devices, libinput devices, sensors)
///   do not report a window association. These use [`EventTarget::Device`] with
///   a device identifier meaningful to the backend or driver.
///
/// - Some events conceptually apply to the whole application or have no
///   specific target (timers, system notifications, compositor signals).
///   These use [`EventTarget::Global`].
///
/// Backends are free to choose their own identifier space, but values must be
/// stable enough that the caller can reliably distinguish sources over time.
///
/// This allows a unified cross-backend event pipeline without leaking backend-
/// specific handle types into the UI layer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum EventTarget {
    /// Event originated from a specific window or surface.
    ///
    /// The `u32` is a backend-defined identifier (e.g. an X11 `Window`,
    /// a Wayland `wl_surface` ID, a Win32 `HWND` translated to an integer,
    /// or an application-generated index when using a custom windowing layer).
    Window(WindowId),

    /// Event originated from a hardware or virtual device.
    ///
    /// The `u32` identifies the device within its backend:
    /// - libinput device ID,
    /// - MIDI device ID,
    /// - joystick/gamepad index,
    /// - virtual device channel,
    /// - etc.
    Device(DeviceId),

    /// Event is not associated with any specific window or device.
    ///
    /// Examples:
    /// - timers,
    /// - compositor-level notifications,
    /// - global shortcuts,
    /// - system–wide state changes.
    #[default]
    Global,
}
