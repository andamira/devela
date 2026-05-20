// devela::sys::os::term::cap
//
//! Defines [`TermCap`] and [`TermCaps`].
//

use crate::{RunCap, RunCapImage, RunCapInput, RunCapWindow, TermColorDepth};

#[doc = crate::_tags!(term runtime)]
/// Terminal capability flag.
#[doc = crate::_doc_location!("sys/os/term")]
///
/// These flags describe independent terminal features. Ordered or exclusive
/// properties, such as color depth, are stored separately in [`TermCaps`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TermCap {
    /* input */
    /// Keyboard input.
    Keyboard,

    /// Mouse input reporting.
    Mouse,

    /// Focus-in/focus-out reporting.
    Focus,

    /// Bracketed paste reporting.
    BracketedPaste,

    /// Terminal resize reporting.
    Resize,

    /* output */
    /// ANSI/VT escape sequences.
    Ansi,

    /// Cursor movement and visibility control.
    Cursor,

    /// SGR styling, such as bold, reset, and colors.
    Style,

    /// Alternate screen buffer.
    AltScreen,

    /// Synchronized output updates.
    SyncUpdate,

    /* image */
    /// Sixel image output.
    Sixel,

    /// Kitty graphics protocol image output.
    KittyImage,

    /// iTerm2 inline image output.
    ItermImage,

    /* query replies */
    /// Device-attributes query replies.
    QueryDeviceAttrs,

    /// Cursor-position query replies.
    QueryCursorPos,

    /// Color query replies.
    QueryColor,
}
impl TermCap {
    /// Returns the bit representing this capability.
    #[must_use]
    pub const fn bit(self) -> u32 {
        1 << self as u8
    }
}

#[doc = crate::_tags!(term runtime)]
/// Terminal capability set.
#[doc = crate::_doc_location!("sys/os/term")]
///
/// Stores independent terminal feature flags plus the maximum known color depth.
///
/// `TermCaps` is terminal-local. Use [`to_run_cap`][Self::to_run_cap] to project
/// the portion that can be represented by the broader runtime capability model.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TermCaps {
    caps: u32,
    color: TermColorDepth,
}

impl TermCaps {
    /// No known terminal capabilities.
    pub const EMPTY: Self = Self { caps: 0, color: TermColorDepth::Mono };

    /// A conservative ANSI terminal baseline.
    pub const ANSI: Self = Self::EMPTY
        .with(TermCap::Keyboard)
        .with(TermCap::Ansi)
        .with(TermCap::Cursor)
        .with(TermCap::Style)
        .with_color(TermColorDepth::Ansi4);

    /// Returns an empty terminal capability set.
    #[must_use]
    pub const fn empty() -> Self {
        Self::EMPTY
    }

    /// Returns the maximum known terminal color depth.
    #[must_use]
    pub const fn color(self) -> TermColorDepth {
        self.color
    }

    /// Returns a copy with the given color depth.
    #[must_use]
    pub const fn with_color(mut self, color: TermColorDepth) -> Self {
        self.color = color;
        self
    }

    /// Returns whether `cap` is present.
    #[must_use]
    pub const fn has(self, cap: TermCap) -> bool {
        self.caps & cap.bit() != 0
    }

    /// Returns a copy with `cap` enabled.
    #[must_use]
    pub const fn with(mut self, cap: TermCap) -> Self {
        self.caps |= cap.bit();
        self
    }

    /// Returns a copy with `cap` disabled.
    #[must_use]
    pub const fn without(mut self, cap: TermCap) -> Self {
        self.caps &= !cap.bit();
        self
    }

    /// Returns the raw capability bits.
    #[must_use]
    pub const fn raw(self) -> u32 {
        self.caps
    }

    /// Returns a terminal capability set from raw bits and color depth.
    ///
    /// Unknown high bits are preserved.
    #[must_use]
    pub const fn from_raw(raw: u32, color: TermColorDepth) -> Self {
        Self { caps: raw, color }
    }

    // IMPROVE
    /// Projects this terminal capability set into general runtime capabilities.
    ///
    /// This conversion is intentionally lossy. Terminal-specific features such
    /// as cursor movement, styling, alternate screen buffers, bracketed paste,
    /// and text color depth currently have no direct field in [`RunCap`].
    #[must_use]
    pub const fn to_run_cap(self) -> RunCap {
        let has_input = self.has(TermCap::Keyboard) || self.has(TermCap::Mouse);

        let has_image = self.has(TermCap::Sixel)
            || self.has(TermCap::KittyImage)
            || self.has(TermCap::ItermImage);

        RunCap {
            image: if has_image {
                Some(RunCapImage {
                    max_bitmap_size: None,
                    pixel_native: self.has(TermCap::KittyImage) || self.has(TermCap::ItermImage),
                    rgb: self.has(TermCap::KittyImage) || self.has(TermCap::ItermImage),
                })
            } else {
                None
            },

            input: if has_input {
                Some(RunCapInput {
                    keyboard: self.has(TermCap::Keyboard),
                    mouse: self.has(TermCap::Mouse),
                    gamepad: false,
                    midi: false,
                    touchscreen: false,
                })
            } else {
                None
            },

            audio: None,

            #[cfg(feature = "alloc")]
            system: None,

            window: if self.has(TermCap::Resize) {
                Some(RunCapWindow { multi: false })
            } else {
                None
            },
        }
    }
}
