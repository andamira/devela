// devela/src/sys/os/term/backend/linux/buf.rs
//
//! Defines (`TermLinuxInputBuf`).
//

use crate::{BufferRingU8, Linux, LinuxResult, is};

/// Buffered stdin bytes for the Linux terminal backend.
///
/// This stores raw input bytes before they are consumed by `TermInputParser`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct TermLinuxInputBuf<const CAP: usize> {
    ring: BufferRingU8<u8, [u8; CAP]>,
}
impl<const CAP: usize> Default for TermLinuxInputBuf<CAP> {
    fn default() -> Self {
        Self::new()
    }
}
impl<const CAP: usize> TermLinuxInputBuf<CAP> {
    /// The maximum capacity supported by this buffer.
    pub const MAX_CAPACITY: usize = u8::MAX as usize;

    /// Creates an empty input byte buffer.
    ///
    /// # Panics
    /// Panics if `CAP` cannot be represented by the buffer index type.
    pub const fn new() -> Self {
        assert!(CAP <= Self::MAX_CAPACITY, "TermLinuxInputBuf capacity does not fit in u8");
        Self {
            ring: BufferRingU8::<u8, [u8; CAP]>::from_array_empty([0; CAP]),
        }
    }
    /// Returns `true` if no bytes are buffered.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.ring.is_empty()
    }
    /// Removes all buffered bytes.
    #[allow(dead_code, reason = "TEMP")]
    pub const fn clear(&mut self) {
        self.ring.clear();
    }
    /// Pops the oldest buffered byte.
    pub const fn pop_front(&mut self) -> Option<u8> {
        self.ring.pop_front_copy()
    }
    /// Refills the buffer from currently available stdin bytes.
    ///
    /// Does nothing while buffered bytes remain.
    pub fn refill_from_stdin(&mut self) -> LinuxResult<usize> {
        is! { !self.is_empty(), return Ok(0) }
        // Normalize the physical head/tail so the next spare slice starts at 0.
        self.ring.clear();
        let spare = self.ring.spare_back_slice_mut();
        let n = Linux::read_available(spare)?;
        // `n` came from writing into `spare`, so this should be infallible.
        let committed = self.ring.commit_back_slice_usize(n);
        debug_assert!(committed.is_ok());
        let _ = committed;
        Ok(n)
    }
}
