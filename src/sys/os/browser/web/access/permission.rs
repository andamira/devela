// devela/src/sys/os/browser/web/access/permission.rs

use crate::{_tags, PermissionQuery, Web, WebPermissionSnapshot};

crate::enumset! {
    #[doc = crate::_tags!(web member)]
    /// A browser permission kind.
    #[doc = crate::_doc_meta!{
        location("sys/os/browser/web"),
        test_size_of(WebPermission = 1|8),
    }]
    ///
    /// - <https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API>
    /// - <https://developer.mozilla.org/en-US/docs/Web/API/Permissions#browser_compatibility>
    #[must_use]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub enum WebPermission(
        #[doc = crate::_tags!(web set)]
        /// A set of browser permission kinds.
        #[doc = crate::_doc_meta!{
            location("sys/os/browser/web"),
            test_size_of(WebPermissionSet = 4|32),
        }]
        #[must_use]
        pub WebPermissionSet: u32
    ) {
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
    impl enum {
        /// Starts or polls a non-blocking query for this permission.
        ///
        /// The first call may return `AsyncPoll::`[`Pending`][crate::AsyncPoll::Pending]
        /// while the browser resolves the query.
        /// A later call returns the resolved permission state or a query error.
        pub fn query(self) -> PermissionQuery {
            Web::permissions_query(self)
        }
        /// Returns the cached query for this permission without starting one.
        pub fn cached(self) -> Option<PermissionQuery> {
            Web::permissions_cached(self)
        }
        /// Returns the browser permission name.
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
    impl set {
        /// Starts or polls queries for the permissions in this set.
        pub fn query(self) -> WebPermissionSnapshot {
            WebPermissionSnapshot::query(self)
        }
        /// Takes a snapshot of cached queries for the permissions in this set.
        ///
        /// Permissions without a cached query remain unclassified.
        pub fn cached(self) -> WebPermissionSnapshot {
            WebPermissionSnapshot::cached(self)
        }
    }
}
