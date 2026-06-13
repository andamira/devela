// devela/src/sys/os/term/backend/linux/buf.rs
//
//! Defines (`TermLinuxInputBuf`).
//

use crate::{Linux, LinuxResult, is};

// IMPROVE: make this a more generic ByteQueue.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct TermLinuxInputBuf<const CAP: usize> {
    buf: [u8; CAP],
    pos: usize,
    len: usize,
}
impl<const CAP: usize> Default for TermLinuxInputBuf<CAP> {
    fn default() -> Self {
        Self::new()
    }
}
impl<const CAP: usize> TermLinuxInputBuf<CAP> {
    pub const fn new() -> Self {
        Self { buf: [0; CAP], pos: 0, len: 0 }
    }
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.pos == self.len
    }
    // #[must_use]
    // pub const fn remaining(&self) -> usize {
    //     self.len - self.pos
    // }
    pub const fn clear(&mut self) {
        self.pos = 0;
        self.len = 0;
    }
    pub fn pop_front(&mut self) -> Option<u8> {
        is! { self.is_empty(), return None }
        let byte = self.buf[self.pos];
        self.pos += 1;
        is! { self.pos == self.len, self.clear() }
        Some(byte)
    }
    /// Refills the buffer from currently available stdin bytes.
    ///
    /// Does nothing while buffered bytes remain.
    pub fn refill_from_stdin(&mut self) -> LinuxResult<usize> {
        is! { !self.is_empty(), return Ok(0) }
        let n = Linux::read_available(&mut self.buf)?;
        self.pos = 0;
        self.len = n;
        Ok(n)
    }
}
