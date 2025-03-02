// devela::lang:js::definitions
//
//! Defines the [`Js`] namespace and other items.
//
// TOC
// - struct Js
// - enum JsEvent
// - enum JsPermission
// - enum JsPermissionState
// - struct JsTextMetrics
// - struct JsTextMetricsFull

use crate::{TAG_EXPERIMENTAL, TAG_NON_STANDARD};

/// A Javascript namespace.
///
/// # Methods
/// - core APis
///   - [console](#web-api-console)
///   - [events](#web-api-events)
///   - [history](#web-api-history--navigation)
///   - [permissions](#web-api-permissions)
/// - extended APis
///   - media & graphics
///     - [canvas](#web-api-canvas)
//   - system & hardware
///   - performance & optimization
//     - time
//   - advanced & experimental
pub struct Js;

/// # Web API Events
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Event>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/EventTarget>
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum JsEvent {
    /// Fires when an element is clicked.
    Click,
    /// Fires when a key is pressed down.
    KeyDown,
    /// Fires when a key is released.
    KeyUp,
    /// Fires when the mouse moves over an element.
    MouseMove,
    /// Fires when the mouse button is pressed down.
    MouseDown,
    /// Fires when the mouse button is released.
    MouseUp,
    /// Fires when the window is resized.
    Resize,
}
impl JsEvent {
    /// Returns the event name as a string.
    pub fn as_str(self) -> &'static str {
        use JsEvent as E;
        match self {
            E::Click => "click",
            E::KeyDown => "keydown",
            E::KeyUp => "keyup",
            E::MouseMove => "mousemove",
            E::MouseDown => "mousedown",
            E::MouseUp => "mouseup",
            E::Resize => "resize",
        }
    }
}

/// # Web API permissions
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Permissions#browser_compatibility>
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum JsPermission {
    #[doc = TAG_EXPERIMENTAL!()]
    /// Access to accelerometer sensor data.
    Accelerometer,
    #[doc = TAG_EXPERIMENTAL!()]
    /// Background sync capability for web applications.
    BackgroundSync,
    /// Access to the device camera.
    Camera,
    #[doc = TAG_EXPERIMENTAL!()]
    #[doc = TAG_NON_STANDARD!()]
    /// Read access to the system clipboard.
    ClipboardRead,
    #[doc = TAG_EXPERIMENTAL!()]
    /// Write access to the system clipboard.
    ClipboardWrite,
    /// Access to device geolocation data.
    Geolocation,
    #[doc = TAG_EXPERIMENTAL!()]
    /// Access to gyroscope sensor data.
    Gyroscope,
    /// Access to the device microphone.
    Microphone,
    /// MIDI device access (without system exclusive messages).
    Midi,
    /// Permission to display system notifications.
    Notifications,
    #[doc = TAG_EXPERIMENTAL!()]
    /// Permission to use a payment handler.
    PaymentHandler,
    /// Persistent storage access to prevent data loss.
    PersistentStorage,
    /// Permission to receive push notifications.
    Push,
    /// Allows preventing the screen from sleeping.
    ScreenWakeLock,
    /// Access to storage that requires explicit user permission.
    StorageAccess,
    #[doc = TAG_EXPERIMENTAL!()]
    /// Allows a site to access storage without top-level navigation.
    TopLevelStorageAccess,
}
impl JsPermission {
    /// Returns the permission name as a string.
    pub fn as_str(self) -> &'static str {
        use JsPermission as P;
        match self {
            P::Accelerometer => "accelerometer",
            P::BackgroundSync => "background-sync",
            P::Camera => "camera",
            P::ClipboardRead => "clipboard-read",
            P::ClipboardWrite => "clipboard-write",
            P::Geolocation => "geolocation",
            P::Gyroscope => "gyroscope",
            P::Microphone => "microphone",
            P::Midi => "midi",
            P::Notifications => "notifications",
            P::PaymentHandler => "payment-handler",
            P::PersistentStorage => "persistent-storage",
            P::Push => "push",
            P::ScreenWakeLock => "screen-wake-lock",
            P::StorageAccess => "storage-access",
            P::TopLevelStorageAccess => "top-level-storage-access",
        }
    }
}

/// # Permission query result state.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum JsPermissionState {
    /// The permission has been granted by the user.
    Granted = 1,
    /// The user has not yet granted or denied the permission.
    Prompt = 0,
    /// The user has not yet granted or denied the permission.
    Denied = -1,
    /// The queried permission is unsupported or unrecognized.
    Unknown = -2,
    /// An error occurred while querying the permission state.
    Error = -3,
}
impl From<i32> for JsPermissionState {
    fn from(from: i32) -> Self {
        use JsPermissionState as S;
        match from {
            1 => S::Granted,
            0 => S::Prompt,
            -1 => S::Denied,
            -2 => S::Unknown,
            _ => S::Error,
        }
    }
}

/// Text Metrics.
///
/// Represents the size of rendered text, measured by [`Js::measure_text`].
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics>
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct JsTextMetrics {
    /// The width of the rendered text.
    pub width: f32,
    /// The distance from the baseline to the highest point.
    pub ascent: f32,
    /// The distance from the baseline to the lowest point.
    pub descent: f32,
}

/// Full Text Metrics.
///
/// Represents the size of rendered text, measured by [`Js::measure_text_full`].
///
/// Includes all available text measurement properties.
/// - <https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics>
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct JsTextMetricsFull {
    /// The width of the rendered text.
    pub width: f32,
    /// Distance from the alignment point to the leftmost glyph edge.
    pub left: f32,
    /// Distance from the alignment point to the rightmost glyph edge.
    pub right: f32,
    /// Distance from the baseline to the highest glyph edge.
    pub ascent: f32,
    /// Distance from the baseline to the lowest glyph edge.
    pub descent: f32,
    /// The topmost possible bounding box for text.
    pub font_ascent: f32,
    /// The lowest possible bounding box for text.
    pub font_descent: f32,
    /// Height from the baseline to the top of the `em` square.
    pub em_ascent: f32,
    /// Height from the baseline to the bottom of the `em` square.
    pub em_descent: f32,
    /// Hanging baseline position.
    pub hanging_baseline: f32,
    /// Alphabetic baseline position.
    pub alphabetic_baseline: f32,
    /// Ideographic baseline position.
    pub ideographic_baseline: f32,
}
