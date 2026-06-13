// devela/src/sys/os/term/backend/linux/poll.rs
//
//! Implements polling methods for  [`TermLinux`].
//

use crate::{Duration, EventKind, Linux, LinuxResult, TermLinux, TermPollPolicy, is};

/// # Event polling
impl TermLinux {
    /// Polls for one terminal event using the default low-latency policy.
    ///
    /// This is non-blocking. It drains ready bytes and resolves a pending
    /// lone Escape when no more bytes are immediately available.
    #[inline]
    pub fn poll_event(&mut self) -> LinuxResult<Option<EventKind>> {
        self.poll_event_immediate_impl()
    }
    /// Polls for one terminal event using `policy`.
    #[inline]
    pub fn poll_event_with(&mut self, policy: TermPollPolicy) -> LinuxResult<Option<EventKind>> {
        match policy {
            TermPollPolicy::Pending => self.poll_event_pending(),
            TermPollPolicy::Immediate => self.poll_event(),
            TermPollPolicy::Timeout(timeout) => self.poll_event_timeout(timeout),
            TermPollPolicy::Blocking => self.wait_event().map(Some),
        }
    }
    /// Non-blocking. Preserves pending partial input sequences.
    #[inline]
    pub fn poll_event_pending(&mut self) -> LinuxResult<Option<EventKind>> {
        loop {
            is! { let Some(ev) = self.poll_buffered_event(), return Ok(Some(ev)) }
            is! { self.input_buf.refill_from_stdin()? == 0, return Ok(None) }
        }
    }
    /// Non-blocking. Resolves pending lone Escape after ready bytes are drained.
    #[inline]
    fn poll_event_immediate_impl(&mut self) -> LinuxResult<Option<EventKind>> {
        loop {
            is! { let Some(ev) = self.poll_buffered_event(), return Ok(Some(ev)) }
            is! { self.input_buf.refill_from_stdin()? == 0, return Ok(self.parser.flush_escape()) }
        }
    }
    /// Non-blocking. Resolves pending lone Escape after `timeout`.
    ///
    /// This only waits when the parser is holding a pending `ESC`.
    #[inline]
    pub fn poll_event_timeout(&mut self, timeout: Duration) -> LinuxResult<Option<EventKind>> {
        loop {
            is! { let Some(ev) = self.poll_buffered_event(), return Ok(Some(ev)) }
            is! { self.input_buf.refill_from_stdin()? != 0, continue }

            // No ready bytes. If there is no pending ESC, return immediately.
            is! { !self.parser.is_pending_escape(), return Ok(None) }

            // Pending ESC: give a real escape sequence a short chance to arrive.
            Linux::sleep(timeout)?;

            is! { self.input_buf.refill_from_stdin()? != 0, continue }
            return Ok(self.parser.flush_escape());
        }
    }
    /// Blocks until an event is available.
    #[inline]
    pub fn wait_event(&mut self) -> LinuxResult<EventKind> {
        loop {
            is! { let Some(ev) = self.poll_event()?, return Ok(ev) }
            let byte = Linux::get_byte()?;
            is! { let Some(ev) = self.parser.feed(byte), return Ok(ev) }
        }
    }

    /* internal helpers */

    /// Feeds buffered bytes until one event is completed or the buffer is empty.
    #[inline]
    fn poll_buffered_event(&mut self) -> Option<EventKind> {
        while let Some(byte) = self.input_buf.pop_front() {
            is! { let Some(ev) = self.parser.feed(byte), return Some(ev) }
        }
        None
    }
}
