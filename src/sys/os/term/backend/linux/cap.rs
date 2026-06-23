// devela/src/sys/os/term/backend/linux/cap.rs
//
//! Implements capability methods for  [`TermLinux`].
//

#[cfg(all(feature = "event", feature = "time"))]
use crate::{Ansi, Linux, TermCap, TermDecModeStatus, TermParsed, TermReply, is};
use crate::{ColorDepth, LinuxResult, NonMaxU16, TermCaps, TermLinux, TermSize};
use crate::{RunCap, RunCapColor, RunCapImage, RunCapInput, RunCapText, RunCapWindow};

/// # Capabilities
impl TermLinux {
    /// Returns the last known terminal protocol capabilities.
    pub const fn term_capabilities(&self) -> TermCaps {
        self.term_caps
    }
    /// Returns the last known runtime capabilities.
    pub const fn run_capabilities(&self) -> RunCap {
        self.run_cap
    }

    /// Probes terminal protocol capabilities exposed by the current terminal endpoint.
    ///
    /// When running inside a terminal multiplexer such as `tmux`, this probes the
    /// multiplexer pseudo-terminal, not necessarily the outer terminal emulator.
    ///
    /// # Features
    /// Available with the `time` feature,
    /// because probing uses bounded waits for terminal replies.
    // IMPROVE: add more queries
    #[cfg(all(feature = "event", feature = "time"))]
    pub fn probe_term_capabilities(&mut self) -> LinuxResult<TermCaps> {
        let _guard = Linux::scoped_event_mode()?;
        let mut caps = TermCaps::new();
        /* input */
        // Keyboard
        is! { self.query_dec_private_mode_supported(1000)?, caps.set_cap(TermCap::Mouse) }
        is! { self.query_dec_private_mode_supported(1002)?, caps.set_cap(TermCap::MouseDrag) }
        is! { self.query_dec_private_mode_supported(1003)?, caps.set_cap(TermCap::MouseMotion) }
        is! { self.query_dec_private_mode_supported(1006)?, caps.set_cap(TermCap::MouseSgr); }
        is! { self.query_dec_private_mode_supported(1016)?, caps.set_cap(TermCap::MouseSgrPixels) }
        is! { self.query_dec_private_mode_supported(1004)?, caps.set_cap(TermCap::Focus) }
        is! { self.query_dec_private_mode_supported(2004)?, caps.set_cap(TermCap::BracketedPaste) }
        // resize
        /* output */
        // Ansi
        // Cursor
        // Style
        // x scroll 1007
        // x screen_only 1047
        is! { self.query_dec_private_mode_supported(1049)?, caps.set_cap(TermCap::AltScreen) }
        is! { self.query_dec_private_mode_supported(2026)?, caps.set_cap(TermCap::SyncUpdate) }
        /* image */
        is! { self.query_dec_private_mode_supported(80)?, caps.set_cap(TermCap::Sixel) }
        // KittyImage
        // ItermImage
        /* query */
        // QueryDeviceAttrs,
        // QueryCursorPos,
        // QueryColor,
        self.term_caps = caps;
        Ok(caps)
    }

    /// Probes terminal capabilities, refreshes size, and recomputes runtime capabilities.
    ///
    /// # Features
    /// Available with the `time` feature, because terminal probing uses bounded waits.
    #[cfg(all(feature = "event", feature = "time"))]
    pub fn probe_run_capabilities(&mut self) -> LinuxResult<RunCap> {
        let _ = self.probe_term_capabilities()?;
        self.refresh_run_capabilities()
    }

    /// Refreshes runtime capabilities from cached terminal capabilities and current size.
    pub fn refresh_run_capabilities(&mut self) -> LinuxResult<RunCap> {
        self.refresh_size()?;
        self.run_cap = Self::derive_run_capabilities(self.term_caps, self.size);
        Ok(self.run_cap)
    }
}

// Private helpers
impl TermLinux {
    /// Returns whether the terminal reports support for a DEC private mode.
    #[cfg(all(feature = "event", feature = "time"))]
    fn query_dec_private_mode_supported(&mut self, mode: u16) -> LinuxResult<bool> {
        Ok(self.query_dec_private_mode(mode)?.is_some_and(|s| s.is_supported()))
    }
    /// Sends a DEC private mode status query and reads the matching reply.
    #[cfg(all(feature = "event", feature = "time"))]
    fn query_dec_private_mode(&mut self, mode: u16) -> LinuxResult<Option<TermDecModeStatus>> {
        let mut query = [0u8; 10];
        let query = Ansi::QUERY_DEC_PRIVATE_MODE_N_B(&mut query, mode);
        self.print(query)?;
        self.read_dec_private_mode_reply(mode)
    }

    /// Reads a DEC private mode reply for `mode`, if the terminal answers.
    ///
    /// Expects a DECRPM reply of the form:
    /// `ESC [ ? mode ; status $ y`.
    ///
    /// Returns `Ok(None)` on timeout or when another reply is received.
    #[cfg(all(feature = "event", feature = "time"))]
    fn read_dec_private_mode_reply(&mut self, mode: u16) -> LinuxResult<Option<TermDecModeStatus>> {
        // Conservative probe timeout:
        // 40 × 1ms keeps startup responsive while giving terminal replies time to arrive.
        //
        // This avoids blocking forever on terminals that do not implement DECRQM.
        const TRIES: usize = 40;
        let mut tries = 0;
        while tries < TRIES {
            if let Some(byte) = Linux::try_get_byte() {
                match self.parser.feed_parsed(byte) {
                    TermParsed::Reply(TermReply::DecPrivateMode { mode: reply_mode, status })
                        if reply_mode == mode =>
                    {
                        return Ok(Some(status));
                    }
                    // Other terminal replies may be from previous/proximate probes.
                    // For now we ignore them.
                    TermParsed::Reply(_) => {}
                    // User-facing input during probing.
                    TermParsed::Event(ev) => {
                        let _ = self.queue_event_kind(ev);
                    }
                    TermParsed::Pending | TermParsed::Unknown => {}
                }
            } else {
                tries += 1;
                Linux::sleep_ms(1)?;
            }
        }
        Ok(None)
    }

    /// Projects terminal capabilities into runtime capabilities.
    pub(super) const fn derive_run_capabilities(caps: TermCaps, _size: Option<TermSize>) -> RunCap {
        let depth = caps.color_depth();
        RunCap {
            color: if depth.has_color() {
                Some(RunCapColor {
                    depth,
                    palette_size: Self::palette_size(depth),
                    palette_set: caps.get_query_color() != 0,
                })
            } else {
                None
            },
            image: if caps.get_sixel() != 0
                || caps.get_kitty_image() != 0
                || caps.get_iterm_image() != 0
            {
                Some(RunCapImage {
                    max_bitmap_extent: None,
                    pixel_native: caps.get_kitty_image() != 0 || caps.get_iterm_image() != 0,
                })
            } else {
                None
            },
            input: Some(
                RunCapInput::new()
                    .with_if(caps.get_keyboard() != 0, RunCapInput::KEYBOARD)
                    .with_if(caps.get_mouse() != 0, RunCapInput::MOUSE),
            ),
            text: Some(
                RunCapText::new()
                    .with(RunCapText::WRITE)
                    .with(RunCapText::CELL_GRID)
                    .with_if(caps.get_cursor() != 0, RunCapText::CURSOR)
                    .with_if(caps.get_style() != 0, RunCapText::STYLE)
                    .with_if(caps.get_keyboard() != 0, RunCapText::EDIT)
                    .with_if(depth.has_color(), RunCapText::COLOR_PAIR),
            ),
            audio: None,
            system: None,
            window: if caps.get_resize() != 0 { Some(RunCapWindow { multi: false }) } else { None },
        }
    }
    const fn palette_size(depth: ColorDepth) -> Option<NonMaxU16> {
        match depth {
            ColorDepth::Palette2 => NonMaxU16::new(2),
            ColorDepth::Palette8 => NonMaxU16::new(8),
            ColorDepth::Palette16 => NonMaxU16::new(16),
            ColorDepth::Palette256 => NonMaxU16::new(256),
            _ => None,
        }
    }
}
