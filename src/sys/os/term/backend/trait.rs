// devela/src/sys/os/term/backend/trait.rs
//
//! Terminal backend traits.
//

use crate::{Debug, TermCaps, TermMode, TermSize};
#[cfg(feature = "event")]
use crate::{EventKind, TermPollPolicy};

#[doc = crate::_tags!(term platform)]
/// Backend interface for terminal host adapters.
#[doc = crate::_doc_meta!{location("sys/os/term/backend")}]
pub trait TermBackend: Sized {
    /// Backend error type.
    type Error;

    /// Guard type returned when entering a terminal session.
    type Session: Debug;

    /* required methods */

    /// Opens a terminal backend.
    fn open() -> Result<Self, Self::Error>;

    /// Enters a scoped terminal session.
    fn session(&mut self, mode: TermMode) -> Result<Self::Session, Self::Error>;

    /// Writes bytes to the terminal output stream.
    fn print(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;

    /// Returns the last known terminal size.
    fn size(&self) -> Option<TermSize>;

    /// Refreshes and returns the terminal size.
    fn refresh_size(&mut self) -> Result<Option<TermSize>, Self::Error>;

    /// Returns the last known terminal protocol capabilities.
    fn term_capabilities(&self) -> TermCaps;

    /// Polls for one terminal event using `policy`.
    #[cfg(feature = "event")]
    fn poll_event_with(&mut self, policy: TermPollPolicy)
    -> Result<Option<EventKind>, Self::Error>;

    /* provided methods */

    /// Enters a scoped raw terminal session.
    #[inline(always)]
    fn session_raw(&mut self) -> Result<Self::Session, Self::Error> {
        self.session(TermMode::raw())
    }
    /// Probes terminal protocol capabilities.
    ///
    /// Backends that cannot probe may keep the cached/default capabilities.
    #[inline(always)]
    fn probe_term_capabilities(&mut self) -> Result<TermCaps, Self::Error> {
        Ok(self.term_capabilities())
    }
    /// Polls for one terminal event using the default immediate policy.
    #[inline(always)]
    #[cfg(feature = "event")]
    fn poll_event(&mut self) -> Result<Option<EventKind>, Self::Error> {
        self.poll_event_with(TermPollPolicy::Immediate)
    }
}
