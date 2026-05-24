// devela::data::codec::tag
//
//! Fixed binary tags.
//

use crate::{_impl_init, Word, impl_trait, whilst};

#[must_use]
#[doc = crate::_tags!(data codec)]
/// A fixed four-byte binary tag.
#[doc = crate::_doc_location!("data/codec/bin")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialOrd, Ord)]
pub struct BinTag4([u8; 4]);
_impl_init![BinTag4::new([0; 4]) => BinTag4];

#[rustfmt::skip]
impl BinTag4 {
    /// The length of a [`BinTag4`] in bytes.
    pub const LEN: usize = 4;

    /// A zeroed tag.
    pub const NIL: Self = Self([0; 4]);

    /// Creates a tag from its bytes.
    pub const fn new(bytes: [u8; 4]) -> Self { Self(bytes) }

    /// Returns the tag bytes.
    #[must_use]
    pub const fn bytes(self) -> [u8; 4] { self.0 }
    /// Returns a reference to the tag bytes.
    #[must_use] #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8; 4] { &self.0 }

    /// Creates a tag from a little-endian integer.
    #[inline(always)]
    pub const fn from_u32_le(raw: u32) -> Self { Self(raw.to_le_bytes()) }
    /// Creates a tag from a big-endian integer.
    #[inline(always)]
    pub const fn from_u32_be(raw: u32) -> Self { Self(raw.to_be_bytes()) }

    /// Returns this tag as a little-endian integer.
    #[inline(always)]
    pub const fn to_u32_le(self) -> u32 { u32::from_le_bytes(self.0) }
    /// Returns this tag as a big-endian integer.
    #[inline(always)]
    pub const fn to_u32_be(self) -> u32 { u32::from_be_bytes(self.0) }

    /// Returns whether all bytes are ASCII.
    #[must_use]
    pub const fn is_ascii(self) -> bool {
        let b = self.0;
        b[0] <= 0x7F && b[1] <= 0x7F && b[2] <= 0x7F && b[3] <= 0x7F
    }
    /// Returns whether all bytes are printable ASCII or spaces.
    #[must_use]
    pub const fn is_ascii_graphic_or_space(self) -> bool {
        let b = self.0;
        Self::is_graphic_or_space(b[0])
            && Self::is_graphic_or_space(b[1])
            && Self::is_graphic_or_space(b[2])
            && Self::is_graphic_or_space(b[3])
    }

    /// Returns whether this tag follows the canonical FOURCC shape.
    ///
    /// Accepts 1 to 4 ASCII alphanumeric bytes, right-padded with spaces.
    #[must_use]
    pub const fn is_fourcc(self) -> bool {
        let b = self.0;
        let mut seen_space = false;
        let mut seen_symbol = false;
        whilst! { i in 0..4; {
            if b[i] == b' ' {
                seen_space = true;
            } else if Self::is_alnum(b[i]) {
                if seen_space { return false; }
                seen_symbol = true;
            } else {
                return false;
            }
        }}
        seen_symbol
    }

    /// Returns the length without trailing ASCII spaces.
    #[must_use]
    pub const fn trimmed_len(self) -> usize {
        let b = self.0;
        if b[3] != b' ' { 4 }
        else if b[2] != b' ' { 3 }
        else if b[1] != b' ' { 2 }
        else if b[0] != b' ' { 1 }
        else { 0 }
    }

    /// Returns whether the tag equals another tag.
    #[must_use] #[inline(always)]
    pub const fn eq(self, other: Self) -> bool {
        self.eq_bytes(other.bytes())
    }
    /// Returns whether the tag equals the given bytes.
    #[must_use] #[inline(always)]
    pub const fn eq_bytes(self, bytes: [u8; 4]) -> bool {
        self.0[0] == bytes[0]
            && self.0[1] == bytes[1]
            && self.0[2] == bytes[2]
            && self.0[3] == bytes[3]
    }

    /// Creates a tag from a canonical FOURCC byte sequence.
    #[must_use]
    pub const fn from_fourcc(bytes: [u8; 4]) -> Option<Self> {
        let tag = Self(bytes);
        if tag.is_fourcc() { Some(tag) } else { None }
    }

    /* private helpers */

    const fn is_alnum(b: u8) -> bool {
        b >= b'0' && b <= b'9'
            || b >= b'A' && b <= b'Z'
            || b >= b'a' && b <= b'z'
    }
    const fn is_graphic_or_space(b: u8) -> bool {
        b == b' ' || (b >= 0x21 && b <= 0x7E)
    }
}
impl_trait![PartialEq for BinTag4 |self, other| Self::eq(*self, *other)];
impl_trait![Hash for BinTag4 |self, s| self.bytes().hash(s)];

#[rustfmt::skip]
impl Word for BinTag4 {
    type Repr = [u8; 4];
    #[inline(always)]
    fn raw(self) -> Self::Repr { self.0 }
    #[inline(always)]
    fn from_raw(raw: Self::Repr) -> Self { Self(raw) }
}
