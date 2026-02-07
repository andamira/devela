// devela::run::regime::cap::definitions
//
//! Runtime capabilities.
//

#[cfg(feature = "alloc")]
use crate::String;

#[doc = crate::_tags!(runtime)]
/// The capabilities supported by a `Runtime`.
#[doc = crate::_doc_location!("run/regime")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RunCap {
    /// Image capabilities.
    pub image: Option<RunCapImage>,
    /// Input capabilities.
    pub input: Option<RunCapInput>,
    /// Audio capabilities.
    pub audio: Option<RunCapAudio>,
    /// System capabilities.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub system: Option<RunCapSystem>,
    /// Windowing capabilities.
    pub window: Option<RunCapWindow>,
}

#[doc = crate::_tags!(runtime image)]
/// Image capabilities.
#[doc = crate::_doc_location!("run/regime")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RunCapImage {
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

#[doc = crate::_tags!(runtime interaction)]
/// Runtime input capabilities.
#[doc = crate::_doc_location!("run/regime")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RunCapInput {
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

#[doc = crate::_tags!(runtime audio)]
/// Runtime audio capabilities.
#[doc = crate::_doc_location!("run/regime")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RunCapAudio {
    /// Audio playback capabilities.
    pub play: bool,
}

#[doc = crate::_tags!(runtime)]
/// Runtime system capabilities.
#[doc = crate::_doc_location!("run/regime")]
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RunCapSystem {
    /// The name of the detected OS version.
    pub os_version: Option<String>,
    /// The name of the current user.
    pub user_name: Option<String>,
    /// The name of the current host.
    pub host_name: Option<String>,
}

#[doc = crate::_tags!(runtime)]
/// Runtime window capabilities.
#[doc = crate::_doc_location!("run/regime")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RunCapWindow {
    /// Whether multiple windows are supported.
    pub multi: bool,
}
