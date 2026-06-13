// devela/src/data/access/cursor/byte/define.rs
//
//! Defines [`ByteCursor`], and [`ByteCursorError`].
//

#[doc = crate::_tags!(data parser)]
/// A byte-position cursor over storage `S`.
#[doc = crate::_doc_meta!{location("data/access/cursor")}]
///
/// The storage determines the available operations:
/// - `ByteCursor<&[u8]>` provides read methods.
/// - `ByteCursor<&mut [u8]>` provides write methods.
///
/// The generic parameter is not meant as a broad trait abstraction.
/// It keeps one public type name while allowing concrete, const-friendly
/// implementations for shared and mutable byte slices.
///
/// This is intentionally not a serializer, deserializer, parser, stream, or I/O
/// abstraction. It is only `(byte storage, byte position)` plus explicit movement.
///
/// # Performance
/// The fixed-width methods such as `take_4`, `write_4`, `take_u32_le`, and
/// `write_u32_le` use direct indexed loads/stores after one bounds check.
/// They are meant to compile close to the current manual binary-codec style.
///
/// The borrowed slice methods such as `take` and `peek` do not copy the input.
/// They return byte slices from the original storage.
///
/// The generic copied-array methods such as `take_array::<N>` use a loop.
/// The dynamic write methods such as `write` and `write_at` also use a loop.
/// They should not replace [`read_at!`], [`write_at!`], or fixed-width cursor
/// methods in tiny hot codec headers unless that trade-off is acceptable.
///
/// # Example
/// ```
/// # use devela::{ByteCursor, NotEnoughSpace};
/// let bytes = b"RIFF\x24\x00\x00\x00WAVE";
/// let mut cur = ByteCursor::reader(bytes);
///
/// assert_eq!(cur.take_4(), Some(*b"RIFF"));
/// assert_eq!(cur.take_u32_le(), Some(36));
/// assert_eq!(cur.take_4(), Some(*b"WAVE"));
/// assert_eq!(cur.pos(), 12);
///
/// let mut out = [0u8; 12];
/// let mut cur = ByteCursor::writer(&mut out);
///
/// cur.write_4(*b"RIFF")?;
/// cur.write_u32_le(36)?;
/// cur.write_4(*b"WAVE")?;
///
/// assert_eq!(&out, b"RIFF\x24\x00\x00\x00WAVE");
/// # Ok::<(), NotEnoughSpace>(())
/// ```
///
/// [`read_at!`]: crate::read_at
/// [`write_at!`]: crate::write_at
///
/// # Methods
/// - Common:
///   - [from_storage_at](#method.from_storage_at).
///   - [pos](#method.pos).
///   - [set_pos](#method.set_pos).
///   - [storage](#method.storage).
///   - [into_storage](#method.into_storage) ([*copy*](#method.into_storage_copy)).
///
/// - Shared for readers & writes:
///   - [new](#method.new).
///   - [at](#method.at).
///   - [as_slice](#method.as_slice).
///   - [len](#method.len).
///   - [is_empty](#method.is_empty).
///   - [remaining_len](#method.remaining_len).
///   - [try_set_pos](#method.try_set_pos).
///   - [set_pos_clamped](#method.set_pos_clamped).
///   - [advance](#method.advance).
///   - [skip_exact](#method.skip_exact).
///
/// - Reader specific:
///   - [new_read](#method.new_read).
///   - [is_eof](#method.is_eof).
///   - [can_take](#method.can_take).
///   - [rest](#method.rest).
///   - [peek](#method.peek) ([*_array*](#method.peek_array), [*_u8*](#method.peek_u8),
///     [*_2*](#method.peek_2), [*_4*](#method.peek_4), [*_8*](#method.peek_8)).
///   - [take](#method.take) ([*_array*](#method.take_array), [*_u8*](#method.take_u8),
///     [*_2*](#method.take_2), [*_4*](#method.take_4), [*_8*](#method.take_8),
///     [*_u16_le*](#method.take_u16_le), [*_u16_be*](#method.take_u16_be),
///     [*_u32_le*](#method.take_u32_le), [*_u32_be*](#method.take_u32_be),
///     [*_u64_le*](#method.take_u64_le), [*_u64_be*](#method.take_u64_be)).
///
/// - Writer specific:
///   - [new_write](#method.new_write).
///   - [as_mut_slice](#method.as_mut_slice).
///   - [can_write](#method.can_write) ([*at*](#method.can_write_at)).
///   - [write](#method.write) ([*_u8*](#method.write_u8),
///     [*_2*](#method.write_2), [*_4*](#method.write_4), [*_8*](#method.write_8),
///     [*_u16_le*](#method.write_u16_le), [*_u16_be*](#method.write_u16_be),
///     [*_u32_le*](#method.write_u32_le), [*_u32_be*](#method.write_u32_be),
///     [*_u64_le*](#method.write_u64_le), [*_u64_be*](#method.write_u64_be)).
///   - [write_at](#method_write_at) ([*_u8*](#method.write_at_u8), [*_2*](#method.write_at_2),
///     [*_4*](#method.write_at_4), [*_8*](#method.write_at_8),
///     [*_u32_le*](#method.write_at_u32_le), [*_u32_be*](#method.write_at_u32_be)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ByteCursor<S> {
    pub(super) storage: S,
    pub(super) pos: usize,
}

/// # Common methods
impl<S> ByteCursor<S> {
    /// Creates a cursor from `storage` and `pos`.
    ///
    /// This does not validate `pos`, because generic storage has no length
    /// contract. Slice-specific constructors and methods perform bounds checks
    /// where needed.
    #[must_use]
    #[inline(always)]
    pub const fn from_storage_at(storage: S, pos: usize) -> Self {
        Self { storage, pos }
    }
    /// Returns the current byte position.
    #[must_use]
    #[inline(always)]
    pub const fn pos(&self) -> usize {
        self.pos
    }
    /// Sets the current byte position without validating it.
    ///
    /// Slice-specific read/write methods will fail if the position is out of
    /// bounds. Use this only when the position was computed by trusted format
    /// logic.
    #[inline(always)]
    pub const fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }
    /// Returns a shared reference to the underlying storage.
    #[must_use]
    #[inline(always)]
    pub const fn storage(&self) -> &S {
        &self.storage
    }
    /// Consumes the cursor and returns its storage.
    #[must_use]
    #[inline(always)]
    pub fn into_storage(self) -> S {
        self.storage
    }
    /// Consumes the cursor and returns its storage.
    #[must_use]
    #[inline(always)]
    pub const fn into_storage_copy(self) -> S
    where
        S: Copy,
    {
        self.storage
    }
}
