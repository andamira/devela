// devela/src/data/access/cursor/byte/write.rs

use crate::{ByteCursor, IndexOutOfBounds, NotEnoughSpace};
use crate::{is, unwrap, whilst, write_at};

/// # Write methods
impl<'a> ByteCursor<&'a mut [u8]> {
    /// Creates a write cursor at position `0`.
    #[must_use]
    #[inline(always)]
    pub const fn new(storage: &'a mut [u8]) -> Self {
        Self { storage, pos: 0 }
    }

    /// Creates a write cursor at position `0`.
    ///
    /// This is an explicit alias for cases where `ByteCursor::new` inference
    /// would be less clear.
    #[must_use]
    #[inline(always)]
    pub const fn writer(storage: &'a mut [u8]) -> Self {
        Self::new(storage)
    }
    /// Creates a write cursor at `pos`.
    ///
    /// The position is not rejected here. Write methods return
    /// `NotEnoughSpace` when the position cannot satisfy the requested
    /// operation.
    #[must_use]
    #[inline(always)]
    pub const fn at(storage: &'a mut [u8], pos: usize) -> Self {
        Self { storage, pos }
    }

    /// Returns the underlying byte slice as shared.
    #[must_use]
    #[inline(always)]
    pub const fn as_slice(&self) -> &[u8] {
        self.storage
    }
    /// Returns the underlying byte slice as mutable.
    ///
    /// The returned borrow is tied to `&mut self`, not to the original storage
    /// lifetime. This keeps the cursor from lending a long-lived mutable window
    /// while it still owns the exclusive storage capability.
    #[must_use]
    #[inline(always)]
    pub const fn as_mut_slice(&mut self) -> &mut [u8] {
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
    /// Returns the remaining writable byte count, saturating at `0`.
    ///
    /// If `pos > len`, this returns `0`.
    #[must_use]
    #[inline(always)]
    pub const fn remaining_len(&self) -> usize {
        let len = self.storage.len();
        len.saturating_sub(self.pos)
    }

    /// Returns whether exactly `len` bytes can be written at the current
    /// position.
    #[must_use]
    #[inline(always)]
    pub const fn can_write(&self, len: usize) -> bool {
        self.can_write_at(self.pos, len)
    }
    /// Returns whether exactly `len` bytes can be written at `pos`.
    #[must_use]
    #[inline(always)]
    pub const fn can_write_at(&self, pos: usize, len: usize) -> bool {
        let total = self.storage.len();
        pos <= total && len <= total - pos
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
    /// when insufficient space should be treated as an error.
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
    pub const fn skip_exact(&mut self, len: usize) -> Result<(), NotEnoughSpace> {
        if self.can_write(len) {
            self.pos += len;
            Ok(())
        } else {
            Err(NotEnoughSpace(Some(len)))
        }
    }

    /* write */

    /// Writes a dynamic byte slice and advances by its length.
    #[inline]
    pub const fn write(&mut self, bytes: &[u8]) -> Result<(), NotEnoughSpace> {
        let len = bytes.len();
        is! { !self.can_write(len), return Err(NotEnoughSpace(Some(len))) }
        let start = self.pos;
        whilst! { i in 0..len; { self.storage[start + i] = bytes[i]; }}
        self.pos += len;
        Ok(())
    }
    /// Writes one byte and advances by `1`.
    #[inline(always)]
    pub const fn write_u8(&mut self, byte: u8) -> Result<(), NotEnoughSpace> {
        if self.can_write(1) {
            write_at![self.storage, +=self.pos, byte];
            Ok(())
        } else {
            Err(NotEnoughSpace(Some(1)))
        }
    }
    /// Writes two bytes and advances by `2`.
    #[inline(always)]
    pub const fn write_2(&mut self, bytes: [u8; 2]) -> Result<(), NotEnoughSpace> {
        unwrap![ok? self.write_at_2(self.pos, bytes)];
        self.pos += 2;
        Ok(())
    }
    /// Writes four bytes and advances by `4`.
    #[inline(always)]
    pub const fn write_4(&mut self, bytes: [u8; 4]) -> Result<(), NotEnoughSpace> {
        unwrap![ok? self.write_at_4(self.pos, bytes)];
        self.pos += 4;
        Ok(())
    }
    /// Writes eight bytes and advances by `8`.
    #[inline(always)]
    pub const fn write_8(&mut self, bytes: [u8; 8]) -> Result<(), NotEnoughSpace> {
        unwrap![ok? self.write_at_8(self.pos, bytes)];
        self.pos += 8;
        Ok(())
    }

    /// Writes a little-endian `u16`.
    #[inline(always)]
    pub const fn write_u16_le(&mut self, value: u16) -> Result<(), NotEnoughSpace> {
        self.write_2(value.to_le_bytes())
    }
    /// Writes a big-endian `u16`.
    #[inline(always)]
    pub const fn write_u16_be(&mut self, value: u16) -> Result<(), NotEnoughSpace> {
        self.write_2(value.to_be_bytes())
    }
    /// Writes a little-endian `u32`.
    #[inline(always)]
    pub const fn write_u32_le(&mut self, value: u32) -> Result<(), NotEnoughSpace> {
        self.write_4(value.to_le_bytes())
    }
    /// Writes a big-endian `u32`.
    #[inline(always)]
    pub const fn write_u32_be(&mut self, value: u32) -> Result<(), NotEnoughSpace> {
        self.write_4(value.to_be_bytes())
    }
    /// Writes a little-endian `u64`.
    #[inline(always)]
    pub const fn write_u64_le(&mut self, value: u64) -> Result<(), NotEnoughSpace> {
        self.write_8(value.to_le_bytes())
    }
    /// Writes a big-endian `u64`.
    #[inline(always)]
    pub const fn write_u64_be(&mut self, value: u64) -> Result<(), NotEnoughSpace> {
        self.write_8(value.to_be_bytes())
    }

    /* write_at */

    /// Writes a dynamic byte slice at `pos` without advancing.
    #[inline]
    pub const fn write_at(&mut self, pos: usize, bytes: &[u8]) -> Result<(), NotEnoughSpace> {
        let len = bytes.len();
        is! { !self.can_write_at(pos, len), return Err(NotEnoughSpace(Some(len))) }
        whilst! { i in 0..len; { self.storage[pos + i] = bytes[i]; }}
        Ok(())
    }
    /// Writes two bytes at `pos` without advancing.
    #[inline(always)]
    pub const fn write_at_2(&mut self, pos: usize, bytes: [u8; 2]) -> Result<(), NotEnoughSpace> {
        if self.can_write_at(pos, 2) {
            write_at![self.storage, pos, @2 bytes];
            Ok(())
        } else {
            Err(NotEnoughSpace(Some(2)))
        }
    }
    /// Writes four bytes at `pos` without advancing.
    #[inline(always)]
    pub const fn write_at_4(&mut self, pos: usize, bytes: [u8; 4]) -> Result<(), NotEnoughSpace> {
        if self.can_write_at(pos, 4) {
            write_at![self.storage, pos, @4 bytes];
            Ok(())
        } else {
            Err(NotEnoughSpace(Some(4)))
        }
    }
    /// Writes eight bytes at `pos` without advancing.
    #[inline(always)]
    pub const fn write_at_8(&mut self, pos: usize, bytes: [u8; 8]) -> Result<(), NotEnoughSpace> {
        if self.can_write_at(pos, 8) {
            write_at![self.storage, pos, @8 bytes];
            Ok(())
        } else {
            Err(NotEnoughSpace(Some(8)))
        }
    }

    /// Writes a little-endian `u32` at `pos` without advancing.
    ///
    /// Useful for patching size fields after writing a payload.
    #[inline(always)]
    pub const fn write_at_u32_le(&mut self, pos: usize, value: u32) -> Result<(), NotEnoughSpace> {
        self.write_at_4(pos, value.to_le_bytes())
    }
    /// Writes a big-endian `u32` at `pos` without advancing.
    ///
    /// Useful for patching size fields after writing a payload.
    #[inline(always)]
    pub const fn write_at_u32_be(&mut self, pos: usize, value: u32) -> Result<(), NotEnoughSpace> {
        self.write_at_4(pos, value.to_be_bytes())
    }
}
