// devela::sys::os::term::cap::field
//
//! Defines [`TermCap`] and [`TermCaps`].
//

use crate::{BitSized, ColorDepth, TermCap};

crate::bitfield! {
    #[must_use]
    #[doc = crate::_tags!(term runtime)]
    /// Terminal capability bits.
    #[doc = crate::_doc_location!("sys/os/term")]
    ///
    /// Stores independent terminal feature flags plus the maximum known color depth.
    ///
    /// `TermCaps` is terminal-local. Use [`to_run_cap`][Self::to_run_cap] to project
    /// the portion that can be represented by the broader runtime capability model.
    // #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] // TEST
    pub struct TermCaps(u32) {
        /* input */
        /// Keyboard input.
        KEYBOARD           = 0;
        /// Mouse input reporting.
        MOUSE              = 1;
        /// Focus-in/focus-out reporting.
        FOCUS              = 2;
        /// Bracketed paste reporting.
        BRACKETED_PASTE    = 3;
        /// Terminal resize reporting.
        RESIZE             = 4;
        /* output */
        /// ANSI/VT escape sequences.
        ANSI               = 5;
        /// Cursor movement and visibility control.
        CURSOR             = 6;
        /// SGR styling, such as bold, reset, and colors.
        STYLE              = 7;
        /// Alternate screen buffer.
        ALT_SCREEN         = 8;
        /// Synchronized output updates.
        SYNC_UPDATE        = 9;
        /* image */
        /// Sixel image output.
        SIXEL              = 10;
        /// Kitty graphics protocol image output.
        KITTY_IMAGE        = 11;
        /// iTerm2 inline image output.
        ITERM_IMAGE        = 12;
        /* query replies */
        /// Device-attributes query replies.
        QUERY_DEVICE_ATTRS = 13;
        /// Cursor-position query replies.
        QUERY_CURSOR_POS   = 14;
        /// Color query replies.
        QUERY_COLOR        = 15;

        /// Terminal color depth.
        COLOR_DEPTH        = 16..=18;
    }

    impl {
        /// Returns an empty terminal capability set.
        pub const EMPTY: Self = Self::new();

        /// A conservative ANSI terminal baseline.
        pub const ANSI_BASE: Self = Self::new()
            .with_keyboard(1)
            .with_ansi(1)
            .with_cursor(1)
            .with_style(1)
            .with_color_depth_enum(ColorDepth::Ansi4);

        /// Returns the maximum known terminal color depth.
        #[must_use]
        pub const fn color_depth(self) -> ColorDepth {
            ColorDepth::from_bits_trunc(self.get_color_depth() as u8)
        }

        /// Returns a copy with the given color depth.
        pub const fn with_color_depth_enum(self, depth: ColorDepth) -> Self {
            self.with_color_depth(depth as u32)
        }

        /// Sets the maximum known terminal color depth.
        pub const fn set_color_depth_enum(&mut self, depth: ColorDepth) {
            self.set_color_depth(depth as u32);
        }

        /// Returns whether `cap` is present.
        #[must_use]
        pub const fn has(self, cap: TermCap) -> bool {
            self.bits() & cap.bit() != 0
        }

        /// Returns a copy with `cap` enabled.
        pub const fn with_cap(self, cap: TermCap) -> Self {
            Self::from_bits(self.bits() | cap.bit())
        }

        /// Returns a copy with `cap` disabled.
        pub const fn without_cap(self, cap: TermCap) -> Self {
            Self::from_bits(self.bits() & !cap.bit())
        }
    }
}
