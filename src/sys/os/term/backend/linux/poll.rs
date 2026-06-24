// devela/src/sys/os/term/backend/linux/poll.rs
//
//! Implements polling methods for  [`TermLinux`].
//

#[cfg(feature = "time")]
use crate::{Duration, EventTimestamp, LinuxInstant, TimeSource};
use crate::{Event, EventKind, EventWindow};
use crate::{Linux, LinuxResult, TermLinux, TermPollPolicy};
use crate::{ext, is};

#[cfg(feature = "time")]
const TERM_ESC_BLOCKING_TIMEOUT: Duration = Duration::from_millis(10);

/// # Event polling
impl TermLinux {
    /// Polls for one terminal event using the default low-latency policy.
    ///
    /// This is non-blocking. It drains ready bytes and resolves a pending
    /// lone Escape when no more bytes are immediately available.
    pub fn poll_event(&mut self) -> LinuxResult<Option<EventKind>> {
        self.poll_event_immediate_impl()
    }
    /// Polls for one terminal event using `policy`.
    ///
    /// # Features
    /// With the `time` feature, this also supports [`TermPollPolicy::Timeout`].
    pub fn poll_event_with(&mut self, policy: TermPollPolicy) -> LinuxResult<Option<EventKind>> {
        match policy {
            TermPollPolicy::Pending => self.poll_event_pending(),
            TermPollPolicy::Immediate => self.poll_event(),
            #[cfg(feature = "time")]
            TermPollPolicy::Timeout(timeout) => self.poll_event_timeout(timeout),
            TermPollPolicy::Blocking => self.wait_event().map(Some),
        }
    }
    /// Non-blocking. Preserves pending partial input sequences.
    pub fn poll_event_pending(&mut self) -> LinuxResult<Option<EventKind>> {
        loop {
            is! { let Some(ev) = self.poll_buffered_event(), return Ok(Some(ev)) }
            is! { self.input_buf.refill_from_stdin()? == 0, return Ok(None) }
        }
    }
    /// Non-blocking. Resolves pending lone Escape after ready bytes are drained.
    fn poll_event_immediate_impl(&mut self) -> LinuxResult<Option<EventKind>> {
        loop {
            is! { let Some(ev) = self.poll_buffered_event(), return Ok(Some(ev)) }
            is! { self.input_buf.refill_from_stdin()? == 0, return Ok(self.parser.flush_escape()) }
        }
    }
    /// Non-blocking. Resolves pending lone Escape after `timeout`.
    ///
    /// This only waits when the parser is holding a pending `ESC`.
    #[cfg(feature = "time")]
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
    /// Blocks until a terminal event is available.
    ///
    /// Buffered events are returned first. Newly read bytes are parsed through
    /// the semantic event queue, preserving the same construction
    /// and timestamping path used by non-blocking polling.
    ///
    /// With `time`, a pending lone `ESC` is briefly disambiguated
    /// before being emitted as an Escape event.
    pub fn wait_event(&mut self) -> LinuxResult<EventKind> {
        loop {
            is! { let Some(ev) = self.poll_buffered_event(), return Ok(ev) }
            let byte = Linux::get_byte()?;
            let _ = self.feed_byte_to_event_queue(byte);
            is! { let Some(ev) = self.pop_event_kind(), return Ok(ev) }
            // Drain ready bytes so multibyte input and escape sequences can finish.
            while self.input_buf.refill_from_stdin()? != 0 {
                self.fill_event_queue_from_input();
                is! { let Some(ev) = self.pop_event_kind(), return Ok(ev) }
            }
            // Resolve a standalone ESC without bypassing the semantic event queue.
            if self.parser.is_pending_escape() {
                #[cfg(feature = "time")]
                {
                    Linux::sleep(TERM_ESC_BLOCKING_TIMEOUT)?;
                    if self.input_buf.refill_from_stdin()? != 0 {
                        self.fill_event_queue_from_input();
                        is! { let Some(ev) = self.pop_event_kind(), return Ok(ev) }
                    }
                }
                if let Some(ev) = self.parser.flush_escape() {
                    let queued = self.queue_event(ev);
                    debug_assert!(queued);
                    is! { let Some(ev) = self.pop_event_kind(), return Ok(ev) }
                }
            }
        }
    }
}

// Internal helpers
impl TermLinux {
    #[cfg(feature = "time")]
    fn event_from_kind(&self, kind: EventKind) -> Event {
        let ns = <LinuxInstant as TimeSource<u64>>::time_now();
        let ms = (ns / 1_000_000).min(u32::MAX as u64) as u32;
        Event::new(kind, Some(EventTimestamp::from_millis_u32(ms)))
    }
    #[cfg(not(feature = "time"))]
    fn event_from_kind(&self, kind: EventKind) -> Event {
        Event::new(kind, None)
    }

    /// Feeds one byte into the parser and queues a completed event, if any.
    ///
    /// Returns `true` if an event was queued.
    #[inline]
    fn feed_byte_to_event_queue(&mut self, byte: u8) -> bool {
        is! { let Some(ev) = self.parser.feed(byte), self.queue_event(ev), false }
    }
    /// Feeds buffered bytes into the parser
    /// until the semantic event queue is full or the byte buffer is empty.
    fn fill_event_queue_from_input(&mut self) {
        while !self.events.is_full() {
            let Some(byte) = self.input_buf.pop_front() else { break };
            let queued = self.feed_byte_to_event_queue(byte);
            debug_assert!(!queued || !self.events.is_empty());
        }
    }
    fn fill_event_queue_from_app_controls(&mut self) {
        let controls = Self::take_app_controls();
        controls.for_each_while(|control| {
            let queued = self.queue_event(EventKind::Control(control));
            debug_assert!(queued || self.events.is_full());
            !self.events.is_full()
        });
    }
    fn fill_event_queue_from_resize(&mut self) {
        is! { !Self::take_resize(), return }
        match self.refresh_size() {
            Ok(Some(size)) => {
                let extent = ext![size.cols as u32, size.rows as u32];
                let queued =
                    self.queue_event(EventKind::Window(EventWindow::Resized(Some(extent))));
                debug_assert!(queued || self.events.is_full());
            }
            Ok(None) | Err(_) => {
                let queued = self.queue_event(EventKind::Window(EventWindow::Resized(None)));
                debug_assert!(queued || self.events.is_full());
            }
        }
    }
    /// Returns one queued event, filling the queue from buffered input if needed.
    fn poll_buffered_event(&mut self) -> Option<EventKind> {
        // events already queued have top priority
        is! { let Some(ev) = self.pop_event_kind(), return Some(ev) }
        // control notices have priority over newly parsed input
        self.fill_event_queue_from_app_controls();
        is! { let Some(ev) = self.pop_event_kind(), return Some(ev) }
        // resize notices are also external semantic events
        self.fill_event_queue_from_resize();
        is! { let Some(ev) = self.pop_event_kind(), return Some(ev) }
        self.fill_event_queue_from_input();
        self.pop_event_kind()
    }
    /// Queues one terminal event kind as a global event.
    ///
    /// Returns `false` if the semantic event queue is full.
    pub(super) fn queue_event(&mut self, kind: EventKind) -> bool {
        let ev = self.event_from_kind(kind);
        self.events.try_push(ev).is_ok()
    }
    /// Pops the oldest queued terminal event kind.
    fn pop_event_kind(&mut self) -> Option<EventKind> {
        self.events.pop().map(|ev| ev.kind)
    }
}
