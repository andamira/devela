// devela::sys::os::term::backend::linux
//
//! Linux terminal backend.
//
// TOC
// - struct TermLinux
// - impl TermLinux (public, private)
// - impl traits
// - struct TermLinuxRestore (private)

use crate::LinuxRawModeGuard;
use crate::{ColorDepth, Debug, EventKind, NonMaxU16, VersionFull, is};
use crate::{Linux, LinuxError, LinuxResult};
use crate::{
    RunCap, RunCapColor, RunCapImage, RunCapInput, RunCapText, RunCapWindow, RunService,
    RunServiceProbe,
};
use crate::{TermCaps, TermInputParser, TermMode, TermSession, TermSize};

#[doc = crate::_tags!(term linux event)]
/// Linux terminal frontend.
#[doc = crate::_doc_meta!{
    location("sys/os/term/backend"),
    test_size_of(TermLinux = 60|480),
}]
///
/// Coordinates terminal input parsing, scoped sessions, cached terminal
/// capabilities, cached runtime capabilities, and the last known terminal size
/// over the lower-level [`Linux`] terminal helpers.
#[derive(Clone, Debug)]
pub struct TermLinux {
    parser: TermInputParser,
    term_caps: TermCaps,
    run_cap: RunCap,
    size: Option<TermSize>,
}

// Public methods
impl TermLinux {
    /// Opens a Linux terminal frontend.
    ///
    /// This does not enter raw mode and does not install terminal cleanup guards.
    /// Session discipline belongs to the future terminal session layer.
    pub fn open() -> LinuxResult<Self> {
        let size = Linux::terminal_size().ok();
        let mut term = Self {
            parser: TermInputParser::new(),
            term_caps: Self::default_term_caps(),
            run_cap: RunCap::default(),
            size,
        };
        term.run_cap = Self::run_cap_from_term(term.term_caps, term.size);
        Ok(term)
    }

    /// Enters a scoped raw terminal session.
    ///
    /// The previous line discipline is restored when the returned guard is dropped.
    pub fn session_raw(&mut self) -> LinuxResult<TermSession<impl Sized + Debug + use<>>> {
        Ok(TermSession::new(TermLinuxRestore::raw()?))
    }

    /// Enters a scoped terminal session.
    pub fn session(
        &mut self,
        mode: TermMode,
    ) -> LinuxResult<TermSession<impl Sized + Debug + use<>>> {
        let restore =
            if mode.has_raw() { TermLinuxRestore::raw()? } else { TermLinuxRestore::none() };
        Ok(TermSession::new(restore))
    }

    /// Polls and parses one pending terminal event, if available.
    ///
    /// This method is non-blocking. It returns `Ok(None)` when no byte is
    /// currently available on stdin.
    pub fn poll_event(&mut self) -> LinuxResult<Option<EventKind>> {
        is! { Linux::available_bytes()? == 0, return Ok(None) }
        let byte = Linux::get_byte()?;
        Ok(self.parser.feed(byte))
    }
    // pub fn wait_event(&mut self) -> LinuxResult<EventKind> { todo![] } // TODO

    /// Probes and refreshes the known terminal capabilities.
    ///
    /// This first implementation is conservative and does not yet perform
    /// interactive terminal queries. It refreshes size and rebuilds the cached
    /// runtime capability snapshot from the current terminal capability set.
    pub fn probe_capabilities(&mut self) -> LinuxResult<RunCap> {
        self.size = Linux::terminal_size().ok();
        // TODO
        // - send DA / DSR / color queries
        // - parse replies through TermInputParser
        // - update self.term_caps from observed replies
        self.run_cap = Self::run_cap_from_term(self.term_caps, self.size);
        Ok(self.run_cap)
    }
    /// Refreshes and returns the terminal size.
    pub fn refresh_size(&mut self) -> LinuxResult<TermSize> {
        let size = Linux::terminal_size()?;
        self.size = Some(size);
        Ok(size)
    }
    /// Writes bytes to stdout.
    pub fn print(&mut self, bytes: &[u8]) -> LinuxResult<()> {
        Linux::print_bytes(bytes)
    }
    /// Returns the cached terminal capabilities.
    #[must_use]
    pub const fn term_capabilities(&self) -> TermCaps {
        self.term_caps
    }
    /// Returns the cached terminal size, if known.
    #[must_use]
    pub const fn size(&self) -> Option<TermSize> {
        self.size
    }
}

// Private methods
impl TermLinux {
    /// Returns the conservative baseline assumed before interactive probing.
    const fn default_term_caps() -> TermCaps {
        TermCaps::ANSI_BASE
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
    /// Projects terminal capabilities into runtime capabilities.
    const fn run_cap_from_term(caps: TermCaps, _size: Option<TermSize>) -> RunCap {
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
}

/* impl traits */

impl RunService for TermLinux {
    fn run_capabilities(&self) -> RunCap {
        self.run_cap
    }
    fn run_version(&self) -> VersionFull<'_> {
        VersionFull::new(0, 0, 1)
    }
}
impl RunServiceProbe for TermLinux {
    type Error = LinuxError;
    fn run_capabilities_refresh(&mut self) -> Result<RunCap, Self::Error> {
        self.probe_capabilities()
    }
}

/* internal helpers */

#[derive(Debug)]
struct TermLinuxRestore {
    _raw: Option<LinuxRawModeGuard>,
}
#[rustfmt::skip]
impl TermLinuxRestore {
    fn none() -> Self { Self { _raw: None } }
    fn raw() -> LinuxResult<Self> { Ok(Self { _raw: Some(Linux::scoped_raw_mode()?) }) }
}
