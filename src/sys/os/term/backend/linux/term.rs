// devela/src/sys/os/term/backend/linux/term.rs
//
//! Defines [`TermLinux`].
//

use crate::{Debug, Linux, LinuxResult, RunCap, RunService, TermCaps, TermSize, VersionFull};
#[cfg(feature = "time")]
use crate::{LinuxError, RunServiceProbe};
use crate::{TermInputParser, TermLinuxInputBuf, TermLinuxRestore, TermMode, TermSession};

/// Default byte capacity for Linux terminal input batching.
const TERM_INPUT_BUF_CAP: usize = 64;

#[doc = crate::_tags!(term linux)]
/// Linux terminal frontend.
#[doc = crate::_doc_meta!{
    location("sys/os/term"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(TermLinux = 128|1024),
    #[cfg(target_pointer_width = "64")]
    test_size_of(TermLinux = 128|1024),
}]
/// Owns terminal input parsing, scoped session control, cached terminal and
/// runtime capabilities, and the last known terminal size over the lower-level
/// [`Linux`] terminal helpers.
#[derive(Clone, Debug)]
pub struct TermLinux {
    pub(super) parser: TermInputParser,
    pub(super) input_buf: TermLinuxInputBuf<TERM_INPUT_BUF_CAP>,
    pub(super) term_caps: TermCaps,
    pub(super) run_cap: RunCap,
    pub(super) size: Option<TermSize>,
}

/// # Core frontend control
impl TermLinux {
    /// Opens a Linux terminal frontend.
    ///
    /// This does not enter raw mode and does not install terminal cleanup guards.
    /// Session discipline belongs to the future terminal session layer.
    pub fn open() -> LinuxResult<Self> {
        let size = Linux::terminal_size().ok();
        let mut term = Self {
            parser: TermInputParser::new(),
            input_buf: TermLinuxInputBuf::new(),
            term_caps: TermCaps::ANSI_BASE,
            run_cap: RunCap::default(),
            size,
        };
        term.run_cap = Self::derive_run_capabilities(term.term_caps, term.size);
        Ok(term)
    }

    /// Enters a scoped terminal session.
    ///
    /// Applies the requested terminal `mode` changes and
    /// returns a guard that restores the changed state when dropped.
    ///
    /// The requested mode is translated by the backend restore payload,
    /// which records only the changes that must be undone on drop.
    ///
    /// Only successfully applied changes are registered for restoration.
    /// This makes partial setup failures safe: if entering a later mode fails,
    /// earlier changes are still undone as the restore payload is dropped.
    ///
    /// The returned guard does not borrow `self`, so the terminal
    /// frontend can continue to be used while the session is active.
    pub fn session(
        &mut self,
        mode: TermMode,
    ) -> LinuxResult<TermSession<impl Drop + Debug + use<>>> {
        Ok(TermSession::new(TermLinuxRestore::enter(mode)?))
    }
    /// Enters a scoped raw terminal session.
    ///
    /// This is a convenience wrapper around [`session`][Self::session] with [`TermMode::raw`].
    pub fn session_raw(&mut self) -> LinuxResult<TermSession<impl Drop + Debug + use<>>> {
        Ok(TermSession::new(TermLinuxRestore::raw()?))
    }

    /// Writes bytes to stdout.
    pub fn print(&mut self, bytes: &[u8]) -> LinuxResult<()> {
        Linux::print_bytes(bytes)
    }

    /// Returns the last known terminal size.
    pub const fn size(&self) -> Option<TermSize> {
        self.size
    }
    /// Refreshes and returns the terminal size.
    pub fn refresh_size(&mut self) -> LinuxResult<Option<TermSize>> {
        self.size = Linux::terminal_size().ok();
        Ok(self.size)
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
#[cfg(feature = "time")]
impl RunServiceProbe for TermLinux {
    type Error = LinuxError;
    fn run_capabilities_refresh(&mut self) -> Result<RunCap, Self::Error> {
        self.probe_run_capabilities()
    }
}
