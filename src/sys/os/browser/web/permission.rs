// devela::sys::os::browser::web::permission

use crate::{_tags, js_int32};

#[doc = _tags!(web)]
/// Web API permissions
#[doc = crate::_doc_location!("sys/os/browser/web")]
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Permissions#browser_compatibility>
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WebPermission {
    #[doc = _tags!(experimental)]
    /// Access to accelerometer sensor data.
    Accelerometer,
    #[doc = _tags!(experimental)]
    /// Background sync capability for web applications.
    BackgroundSync,
    /// Access to the device camera.
    Camera,
    #[doc = _tags!(experimental non_standard)]
    /// Read access to the system clipboard.
    ClipboardRead,
    #[doc = _tags!(experimental)]
    /// Read access to the system clipboard.
    #[doc = _tags!(experimental)]
    /// Write access to the system clipboard.
    ClipboardWrite,
    /// Access to device geolocation data.
    Geolocation,
    #[doc = _tags!(experimental)]
    /// Access to gyroscope sensor data.
    Gyroscope,
    /// Access to the device microphone.
    Microphone,
    /// MIDI device access (without system exclusive messages).
    Midi,
    /// Permission to display system notifications.
    Notifications,
    #[doc = _tags!(experimental)]
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
    #[doc = _tags!(experimental)]
    /// Allows a site to access storage without top-level navigation.
    TopLevelStorageAccess,
}
impl WebPermission {
    /// Returns the permission name as a string.
    pub fn as_str(self) -> &'static str {
        use WebPermission as P;
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

#[doc = _tags!(web result)]
/// Permission query result state.
#[doc = crate::_doc_location!("sys/os/browser/web")]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum WebPermissionState {
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

impl From<js_int32> for WebPermissionState {
    fn from(from: js_int32) -> Self {
        use WebPermissionState as S;
        match from {
            1 => S::Granted,
            0 => S::Prompt,
            -1 => S::Denied,
            -2 => S::Unknown,
            _ => S::Error,
        }
    }
}
