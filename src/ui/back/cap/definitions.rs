// devela::ui::back::cap::definitions
//
//! UI backend capabilities.
//

#[cfg(feature = "alloc")]
use crate::String;

#[doc = crate::_TAG_UI!()]
/// The capabilities supported by a [`UiService`][crate::UiService].
#[doc = crate::_doc_location!("ui/back")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UiCap {
    /// Image capabilities.
    pub image: Option<UiCapImage>,
    /// Input capabilities.
    pub input: Option<UiCapInput>,
    /// Audio capabilities.
    pub audio: Option<UiCapAudio>,
    /// System capabilities.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub system: Option<UiCapSystem>,
    /// Windowing capabilities.
    pub window: Option<UiCapWindow>,
}

#[doc = crate::_TAG_UI!()]
#[doc = crate::_TAG_IMAGE!()]
/// Image capabilities.
#[doc = crate::_doc_location!("ui/back")]
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

#[doc = crate::_TAG_UI!()]
#[doc = crate::_TAG_INTERACTION!()]
/// Input capabilities.
#[doc = crate::_doc_location!("ui/back")]
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

#[doc = crate::_TAG_UI!()]
#[doc = crate::_TAG_AUDIO!()]
/// Audio capabilities.
#[doc = crate::_doc_location!("ui/back")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiCapAudio {
    /// Audio playback capabilities.
    pub play: bool,
}

#[doc = crate::_TAG_UI!()]
/// System capabilities.
#[doc = crate::_doc_location!("ui/back")]
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UiCapSystem {
    /// The name of the detected OS version.
    pub os_version: Option<String>,
    /// The name of the current user.
    pub user_name: Option<String>,
    /// The name of the current host.
    pub host_name: Option<String>,
}

#[doc = crate::_TAG_UI!()]
/// Window capabilities.
#[doc = crate::_doc_location!("ui/back")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiCapWindow {
    /// Whether multiple windows are supported.
    pub multi: bool,
}
