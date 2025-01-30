// devela::ui::service::cap::definitions
//
//!
//

/// The capabilities supported by a [`UiService`][crate::UiService].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UiCap {
    /// Image capabilities.
    pub image: Option<UiCapImage>,
    /// Input capabilities.
    pub input: Option<UiCapInput>,
    /// Sound capabilities.
    pub sound: Option<UiCapSound>,
    /// System capabilities.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    pub system: Option<UiCapSystem>,
    /// Windowing capabilities.
    pub window: Option<UiCapWindow>,
}

/// Image-related capabilities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiCapImage {
    /// Maximum bitmap size, in native pixels.
    pub max_bitmap_size: Option<[usize; 2]>,
    /// Whether pixel-accurate bitmaps are supported.
    pub pixel_native: bool,

    /// Whether it's possible to specify rgb values.
    pub rgb: bool,
    // ///
    // pub palette: Option<u16>,
    // ///
    // pub palette_change: bool,
    // ///
    // pub palette_size: u16,
}

/// General Input capabilities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiCapInput {
    /// Gamepad input capabilities.
    pub gamepad: bool,
    /// Keyboard input capabilities.
    pub keyboard: bool,
    /// Midi input capabilities
    pub midi: bool,
    /// Mouse input capabilities.
    pub mouse: bool,
    /// Touchscreen input capabilities.
    pub touchscreen: bool,
}

/// Sound and music related capabilities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiCapSound {
    /// Sound playback capabilities.
    pub play: bool,
}

/// System capabilities.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UiCapSystem {
    /// The name of the detected OS version.
    pub os_version: Option<String>,
    /// The name of the current user.
    pub user_name: Option<String>,
    /// The name of the current host.
    pub host_name: Option<String>,
}

/// Window capabilities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiCapWindow {
    /// Whether multiple windows are supported.
    pub multi: bool,
}
