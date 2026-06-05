// devela::data::access::cursor::read

use crate::{ByteCursor, IndexOutOfBounds, UnexpectedEof};
use crate::{is, read_at, slice, whilst};

/// # Read methods
impl<'a> ByteCursor<&'a [u8]> {
    /// Creates a read cursor at position `0`.
    #[must_use]
    #[inline(always)]
    pub const fn new(storage: &'a [u8]) -> Self {
        Self { storage, pos: 0 }
    }

    /// Creates a read cursor at position `0`.
    ///
    /// This is an explicit alias for cases where `ByteCursor::new` inference
    /// would be less clear.
    #[must_use]
    #[inline(always)]
    pub const fn reader(storage: &'a [u8]) -> Self {
        Self::new(storage)
    }
    /// Creates a read cursor at `pos`.
    ///
    /// The position is not rejected here. Read methods return `None` or
    /// `UnexpectedEof` when the position cannot satisfy the requested operation.
    #[must_use]
    #[inline(always)]
    pub const fn at(storage: &'a [u8], pos: usize) -> Self {
        Self { storage, pos }
    }

    /// Returns the underlying byte slice.
    #[must_use]
    #[inline(always)]
    pub const fn as_slice(&self) -> &'a [u8] {
        self.storage
    }

    /// Returns the total storage length.
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.storage.len()
    }
    /// Returns whether the storage is empty.
    #[must_use]
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }
    /// Returns the remaining byte count, saturating at `0`.
    ///
    /// If `pos > len`, this returns `0`.
    #[must_use]
    #[inline(always)]
    pub const fn remaining_len(&self) -> usize {
        let len = self.storage.len();
        len.saturating_sub(self.pos)
    }

    /// Returns whether the cursor is at or beyond the end.
    #[must_use]
    #[inline(always)]
    pub const fn is_eof(&self) -> bool {
        self.pos >= self.storage.len()
    }
    /// Returns whether exactly `len` bytes can be read at the current position.
    #[must_use]
    #[inline(always)]
    pub const fn can_take(&self, len: usize) -> bool {
        let total = self.storage.len();
        self.pos <= total && len <= total - self.pos
    }
    /// Returns the remaining slice.
    ///
    /// If `pos >= len`, this returns an empty slice at the end.
    #[must_use]
    #[inline(always)]
    pub const fn rest(&self) -> &'a [u8] {
        let total = self.storage.len();
        if self.pos >= total {
            slice![&self.storage, total, ..total]
        } else {
            slice![&self.storage, self.pos, ..total]
        }
    }

    /// Sets the position if it is inside the slice bounds.
    #[inline(always)]
    pub const fn try_set_pos(&mut self, pos: usize) -> Result<(), IndexOutOfBounds> {
        if pos <= self.storage.len() {
            self.pos = pos;
            Ok(())
        } else {
            Err(IndexOutOfBounds(Some(pos)))
        }
    }
    /// Sets the position, clamped to the slice length.
    ///
    /// Returns the new position.
    #[inline(always)]
    pub const fn set_pos_clamped(&mut self, pos: usize) -> usize {
        let len = self.storage.len();
        self.pos = if pos <= len { pos } else { len };
        self.pos
    }

    /// Advances by up to `len` bytes, saturating at the end.
    ///
    /// Returns the new position.
    ///
    /// Use [`skip_exact`](#method.skip_exact)
    /// when truncation should be treated as an error.
    #[inline(always)]
    pub const fn advance(&mut self, len: usize) -> usize {
        let total = self.storage.len();
        if self.pos > total {
            self.pos = total;
            return self.pos;
        }
        let rem = total - self.pos;
        self.pos += if len <= rem { len } else { rem };
        self.pos
    }
    /// Advances by exactly `len` bytes.
    #[inline(always)]
    pub const fn skip_exact(&mut self, len: usize) -> Result<(), UnexpectedEof> {
        if self.can_take(len) {
            self.pos += len;
            Ok(())
        } else {
            Err(UnexpectedEof(Some(len)))
        }
    }

    /* peek */

    /// Peeks a borrowed slice of `len` bytes without advancing.
    ///
    /// This is the preferred operation for payload bodies
    /// because it borrows the original input rather than copying.
    #[must_use]
    #[inline(always)]
    pub const fn peek(&self, len: usize) -> Option<&'a [u8]> {
        if self.can_take(len) {
            let start = self.pos;
            let end = start + len;
            Some(slice![&self.storage, start, ..end])
        } else {
            None
        }
    }
    /// Peeks a generic fixed-size array without advancing.
    ///
    /// This generic method uses a loop. Prefer `peek_2`, `peek_4`, or `peek_8`
    /// for common small binary fields when direct fixed-width loads are desired.
    #[must_use]
    #[inline]
    pub const fn peek_array<const N: usize>(&self) -> Option<[u8; N]> {
        is! { !self.can_take(N), return None }
        let mut out = [0u8; N];
        whilst! { i in 0..N; { out[i] = self.storage[self.pos + i]; }}
        Some(out)
    }

    /// Peeks one byte without advancing.
    #[must_use]
    #[inline(always)]
    pub const fn peek_u8(&self) -> Option<u8> {
        is! { self.can_take(1), Some(self.storage[self.pos]), None }
    }
    /// Peeks a fixed two-byte array without advancing.
    #[must_use]
    #[inline(always)]
    pub const fn peek_2(&self) -> Option<[u8; 2]> {
        if self.can_take(2) { Some(read_at![self.storage, self.pos, @2]) } else { None }
    }
    /// Peeks a fixed four-byte array without advancing.
    #[must_use]
    #[inline(always)]
    pub const fn peek_4(&self) -> Option<[u8; 4]> {
        if self.can_take(4) { Some(read_at![self.storage, self.pos, @4]) } else { None }
    }
    /// Peeks a fixed eight-byte array without advancing.
    #[must_use]
    #[inline(always)]
    pub const fn peek_8(&self) -> Option<[u8; 8]> {
        if self.can_take(8) { Some(read_at![self.storage, self.pos, @8]) } else { None }
    }

    /* take */

    /// Takes a borrowed slice of `len` bytes and advances by `len`.
    ///
    /// This is the preferred operation for payload bodies
    /// because it borrows the original input rather than copying.
    #[inline(always)]
    pub const fn take(&mut self, len: usize) -> Option<&'a [u8]> {
        match self.peek(len) {
            Some(slice) => {
                self.pos += len;
                Some(slice)
            }
            None => None,
        }
    }
    /// Takes a generic fixed-size array and advances by `N`.
    ///
    /// # Performance
    /// This generic method uses a loop. Prefer `take_2`, `take_4`, or `take_8`
    /// for common small binary fields in hot codec paths.
    #[inline]
    pub const fn take_array<const N: usize>(&mut self) -> Option<[u8; N]> {
        match self.peek_array::<N>() {
            Some(bytes) => {
                self.pos += N;
                Some(bytes)
            }
            None => None,
        }
    }

    /// Takes one byte and advances by `1`.
    #[inline(always)]
    pub const fn take_u8(&mut self) -> Option<u8> {
        is! { self.can_take(1), { self.pos += 1; Some(self.storage[self.pos -1]) }, None }
    }
    /// Takes a fixed two-byte array and advances by `2`.
    #[inline(always)]
    pub const fn take_2(&mut self) -> Option<[u8; 2]> {
        is! { self.can_take(2), Some(read_at![self.storage, += self.pos, @2]), None }
    }
    /// Takes a fixed four-byte array and advances by `4`.
    #[inline(always)]
    pub const fn take_4(&mut self) -> Option<[u8; 4]> {
        is! { self.can_take(4), Some(read_at![self.storage, += self.pos, @4]), None }
    }
    /// Takes a fixed eight-byte array and advances by `8`.
    #[inline(always)]
    pub const fn take_8(&mut self) -> Option<[u8; 8]> {
        is! { self.can_take(8), Some(read_at![self.storage, += self.pos, @8]), None }
    }
    /// Takes a little-endian `u16`.
    #[inline(always)]
    pub const fn take_u16_le(&mut self) -> Option<u16> {
        match self.take_2() {
            Some(bytes) => Some(u16::from_le_bytes(bytes)),
            None => None,
        }
    }
    /// Takes a big-endian `u16`.
    #[inline(always)]
    pub const fn take_u16_be(&mut self) -> Option<u16> {
        match self.take_2() {
            Some(bytes) => Some(u16::from_be_bytes(bytes)),
            None => None,
        }
    }
    /// Takes a little-endian `u32`.
    #[inline(always)]
    pub const fn take_u32_le(&mut self) -> Option<u32> {
        match self.take_4() {
            Some(bytes) => Some(u32::from_le_bytes(bytes)),
            None => None,
        }
    }
    /// Takes a big-endian `u32`.
    #[inline(always)]
    pub const fn take_u32_be(&mut self) -> Option<u32> {
        match self.take_4() {
            Some(bytes) => Some(u32::from_be_bytes(bytes)),
            None => None,
        }
    }
    /// Takes a little-endian `u64`.
    #[inline(always)]
    pub const fn take_u64_le(&mut self) -> Option<u64> {
        match self.take_8() {
            Some(bytes) => Some(u64::from_le_bytes(bytes)),
            None => None,
        }
    }
    /// Takes a big-endian `u64`.
    #[inline(always)]
    pub const fn take_u64_be(&mut self) -> Option<u64> {
        match self.take_8() {
            Some(bytes) => Some(u64::from_be_bytes(bytes)),
            None => None,
        }
    }
}
