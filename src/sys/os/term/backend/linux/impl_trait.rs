// devela/src/sys/os/term/backend/linux/trait_impl.rs
//
//! Implements [`TermBackend`] for [`TermLinux`].
//

#[cfg(feature = "event")]
use crate::{EventKind, TermPollPolicy};
use crate::{LinuxError, LinuxResult, TermLinux, TermLinuxRestore};
use crate::{TermBackend, TermCaps, TermMode, TermSession, TermSize};

impl TermBackend for TermLinux {
    type Error = LinuxError;
    type Restore = TermLinuxRestore;

    fn open() -> LinuxResult<Self> {
        TermLinux::open()
    }
    fn session(&mut self, mode: TermMode) -> LinuxResult<TermSession<TermLinuxRestore>> {
        Ok(TermSession::new(TermLinuxRestore::enter(mode)?))
    }
    fn print(&mut self, bytes: &[u8]) -> LinuxResult<()> {
        TermLinux::print(self, bytes)
    }
    fn size(&self) -> Option<TermSize> {
        TermLinux::size(self)
    }
    fn refresh_size(&mut self) -> LinuxResult<Option<TermSize>> {
        TermLinux::refresh_size(self)
    }
    fn term_capabilities(&self) -> TermCaps {
        TermLinux::term_capabilities(self)
    }
    #[cfg(feature = "time")]
    fn probe_term_capabilities(&mut self) -> LinuxResult<TermCaps> {
        TermLinux::probe_term_capabilities(self)
    }
    #[cfg(feature = "event")]
    fn poll_event_with(&mut self, policy: TermPollPolicy) -> LinuxResult<Option<EventKind>> {
        TermLinux::poll_event_with(self, policy)
    }
}
