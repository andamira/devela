// devela::run::regime::cap::definitions
//
//! Runtime capabilities.
//

use crate::{ColorDepth, Extent2, NonMaxU16, NonMaxU32, set, test_size_of};

#[doc = crate::_tags!(runtime)]
/// The capabilities supported by a `Runtime`.
#[doc = crate::_doc_meta!{
    location("run/regime"),
    test_size_of(RunCap = 28|224),
}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RunCap {
    /// Audio capabilities.
    pub audio: Option<RunCapAudio>,
    /// Color capabilities.
    pub color: Option<RunCapColor>,
    /// Image capabilities.
    pub image: Option<RunCapImage>,
    /// Input capabilities.
    pub input: Option<RunCapInput>,
    /// System capabilities.
    pub system: Option<RunCapSystem>,
    /// Text capabilities.
    pub text: Option<RunCapText>,
    /// Windowing capabilities.
    pub window: Option<RunCapWindow>,
}

#[doc = crate::_tags!(runtime audio)]
/// Runtime audio capabilities.
#[doc = crate::_doc_meta!{
    location("run/regime"),
    test_size_of(RunCapAudio = 1|8),
}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RunCapAudio {
    /// Audio playback capabilities.
    pub play: bool,
}

#[doc = crate::_tags!(runtime color)]
/// Runtime color capabilities.
#[doc = crate::_doc_meta!{
    location("run/regime"),
    test_size_of(RunCapColor = 4|32),
}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RunCapColor {
    /// Maximum color depth.
    pub depth: ColorDepth,
    /// Maximum palette size, when indexed color is supported.
    pub palette_size: Option<NonMaxU16>,
    /// Whether the palette can be changed by the application.
    pub palette_set: bool,
}

#[doc = crate::_tags!(runtime image)]
/// Runtime image capabilities.
#[doc = crate::_doc_meta!{
    location("run/regime"),
    test_size_of(RunCapImage = 12|96),
}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RunCapImage {
    /// Maximum bitmap size, in native pixels.
    pub max_bitmap_extent: Option<Extent2<NonMaxU32>>,
    /// Whether pixel-accurate bitmaps are supported.
    pub pixel_native: bool,
}

set! {
    #[doc = crate::_tags!(runtime interaction)]
    /// Runtime input capabilities.
    #[doc = crate::_doc_meta!{
        location("run/regime"),
        test_size_of(RunCapInput = 1|8),
    }]
    pub struct RunCapInput(u8) {
        /// Gamepad input capabilities.
        GAMEPAD = 0 ;
        /// Keyboard input capabilities.
        KEYBOARD = 1 ;
        /// Midi input capabilities
        MIDI = 2 ;
        /// Mouse input capabilities.
        MOUSE = 3;
        /// Touchscreen input capabilities.
        TOUCHSCREEN = 4;
    }
}

set! {
    #[doc = crate::_tags!(runtime)]
    /// Runtime system capabilities.
    #[doc = crate::_doc_meta!{
        location("run/regime"),
        test_size_of(RunCapSystem = 2|16),
    }]
    pub struct RunCapSystem(u16) {
        /// Current or monotonic time can be queried.
        TIME = 0;
        /// Timers, sleeps, delays, or scheduled wakeups are available.
        TIMER = 1;
        /// Randomness or system entropy is available.
        RANDOM = 2;
        /// Environment variables or host environment metadata can be queried.
        ENV = 3;
        /// Filesystem access is available.
        FS = 4;
        /// Process spawning or process inspection is available.
        PROCESS = 5;
        /// Thread spawning or host threading is available.
        THREAD = 6;
        /// Network access is available.
        NET = 7;
        /// Host signals, interrupts, or shutdown notifications are available.
        SIGNAL = 8;
    }
}

set! {
    #[doc = crate::_tags!(runtime text)]
    /// Runtime text capabilities.
    #[doc = crate::_doc_meta!{
        location("run/regime"),
        test_size_of(RunCapText = 1|8),
    }]
    pub struct RunCapText(u8) {
        /// Text output.
        WRITE = 0;
        /// Fixed-cell text layout.
        CELL_GRID = 1;
        /// Variable-width text layout.
        PROPORTIONAL = 2;
        /// Addressable text cursor.
        CURSOR = 3;
        /// Text styling, such as bold, underline, inverse, or reset.
        STYLE = 4;
        /// Text input or editable text regions.
        EDIT = 5;
        /// Text color is assigned as a foreground/background pair.
        COLOR_PAIR = 6;
        /// Text extents or advances can be measured.
        MEASURE = 7;
    }
}

test_size_of![RunCapWindow = 1]; // 8 bits
#[doc = crate::_tags!(runtime)]
/// Runtime window capabilities.
#[doc = crate::_doc_meta!{location("run/regime")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RunCapWindow {
    /// Whether multiple windows are supported.
    pub multi: bool,
}
