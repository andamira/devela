// devela/src/data/id/uuid/define.rs
//
//! Defines [`Uuid`].
//

use crate::{Ascii, ConstInit, Str, impl_trait, is, slice, unwrap, whilst};
use crate::{TextCursor, TextParseError, UuidNonNil, UuidVariant, UuidVersion};

#[doc = crate::_tags!(uid)]
/// A standardized portable 128-bit identifier.
#[doc = crate::_doc_meta!{
    location("data/id/uuid"),
    test_size_of(Uuid = 16|128),
}]
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uuid([u8; 16]);

impl ConstInit for Uuid {
    const INIT: Self = Self::NIL;
}
impl Default for Uuid {
    fn default() -> Self {
        Self::NIL
    }
}
impl_trait! { fmt::Display for Uuid |self, f| {
    let mut buf = [0u8; Self::STR_LEN];
    f.write_str(unwrap![some_guaranteed_or_ub self.as_str_into(&mut buf)])
}}

impl Uuid {
    /* constants */

    /// The nil UUID, with all bits set to zero.
    pub const NIL: Self = Self([0; 16]);

    /// The maximum UUID, with all bits set to one.
    pub const MAX: Self = Self([0xFF; 16]);

    /// The length of the standard textual UUID representation.
    pub const STR_LEN: usize = 36;

    /* construction */

    /// Constructs an IETF UUID by setting its version and variant fields.
    pub(super) const fn from_ietf_bytes(mut bytes: [u8; 16], version: UuidVersion) -> Self {
        bytes[6] = (bytes[6] & 0x0F) | (version.number() << 4);
        bytes[8] = (bytes[8] & 0x3F) | 0x80;
        Self::from_bytes(bytes)
    }

    /// Creates a version 4 UUID from 128 bits of random or pseudo-random input.
    ///
    /// The version and variant bits replace six bits of the input,
    /// leaving 122 random bits in the resulting UUID.
    pub const fn from_random_v4(random: [u8; 16]) -> Self {
        Self::from_ietf_bytes(random, UuidVersion::V4)
    }
    /// Creates a version 7 UUID from a Unix millisecond timestamp
    /// and 80 bits of random input.
    ///
    /// The timestamp occupies the standardized 48-bit `unix_ts_ms` field.
    /// The version and variant fields replace six bits of `random`,
    /// leaving 74 random bits in the resulting UUID.
    ///
    /// Returns `None` if `unix_ts_ms` does not fit in 48 bits.
    #[rustfmt::skip]
    pub const fn from_random_v7(unix_ts_ms: u64, random: [u8; 10]) -> Option<Self> {
        is! { unix_ts_ms > 0xFFFF_FFFF_FFFF, return None }
        let ts = unix_ts_ms.to_be_bytes();
        let bytes = [
            ts[2], ts[3], ts[4], ts[5], ts[6], ts[7],
            random[0], random[1],
            random[2], random[3], random[4], random[5],
            random[6], random[7], random[8], random[9],
        ];
        Some(Self::from_ietf_bytes(bytes, UuidVersion::V7))
    }

    /// Creates a UUID from its 16-byte representation.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }
    /// Creates a UUID from a `u128` interpreted in big-endian byte order.
    pub const fn from_u128(value: u128) -> Self {
        Self(value.to_be_bytes())
    }
    /// Creates a UUID from a non-NIL niche-optimized representation.
    pub const fn from_non_nil(non_nil: UuidNonNil) -> Self {
        non_nil.into_uuid()
    }

    /* representation */

    /// Returns a reference to the 16-byte representation.
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
    /// Returns this UUID as a big-endian `u128`.
    pub const fn as_u128(self) -> u128 {
        u128::from_be_bytes(self.0)
    }
    /// Returns this UUID in its standard hyphenated textual representation,
    /// using lowercase hexadecimal digits.
    ///
    /// It requires a byte `buffer` of at least [`STR_LEN`][Self::STR_LEN] bytes.
    pub const fn as_str_into(self, buffer: &mut [u8]) -> Option<&str> {
        is! { buffer.len() < Self::STR_LEN, return None }
        let bytes = self.0;
        let mut dst = 0;
        whilst! { src in 0..16; {
            if src == 4 || src == 6 || src == 8 || src == 10 {
                buffer[dst] = b'-';
                dst += 1;
            }
            let byte = bytes[src];
            buffer[dst] = Ascii::digit_lower(byte >> 4);
            buffer[dst + 1] = Ascii::digit_lower(byte & 0x0F);
            dst += 2;
        }}
        unwrap![ok_some Str::from_utf8(slice![buffer, ..Self::STR_LEN])]
    }
    /// Returns the 16-byte representation.
    pub const fn into_bytes(self) -> [u8; 16] {
        self.0
    }
    /// Converts this UUID into a non-NIL niche-optimized representation.
    pub const fn into_non_nil(self) -> Option<UuidNonNil> {
        UuidNonNil::from_uuid(self)
    }

    /* classification */

    /// Returns whether this is the nil UUID.
    pub const fn is_nil(self) -> bool {
        self.as_u128() == 0
    }
    /// Returns whether this is the maximum UUID.
    pub const fn is_max(self) -> bool {
        self.as_u128() == u128::MAX
    }
    /// Returns the UUID variant.
    pub const fn variant(self) -> UuidVariant {
        let byte = self.0[8];
        if byte & 0x80 == 0 {
            UuidVariant::Ncs
        } else if byte & 0xC0 == 0x80 {
            UuidVariant::Ietf
        } else if byte & 0xE0 == 0xC0 {
            UuidVariant::Microsoft
        } else {
            UuidVariant::Future
        }
    }
    /// Returns the version number for an IETF UUID.
    ///
    /// Returns `None` for UUIDs belonging to another variant.
    pub const fn version_number(self) -> Option<u8> {
        match self.variant() {
            UuidVariant::Ietf => Some(self.0[6] >> 4),
            _ => None,
        }
    }
    /// Returns the recognized UUID version.
    ///
    /// Returns `None` for another variant or for an unused or
    /// reserved version number.
    pub const fn version(self) -> Option<UuidVersion> {
        match self.version_number() {
            Some(number) => UuidVersion::from_number(number),
            None => None,
        }
    }
    /// Parses the standard hyphenated textual UUID representation.
    ///
    /// Hexadecimal digits may use either case.
    pub const fn parse_str(string: &str) -> Result<Self, TextParseError> {
        let input = string.as_bytes();
        if input.len() < Self::STR_LEN {
            return Err(TextParseError::unexpected_eof(TextCursor::new_prim(input.len() as _)));
        }
        if input.len() > Self::STR_LEN {
            return Err(TextParseError::trailing_input(TextCursor::new_prim(Self::STR_LEN as _)));
        }
        let mut bytes = [0u8; 16];
        let mut src = 0usize;
        whilst! { dst in 0..16; {
            if src == 8 || src == 13 || src == 18 || src == 23 {
                if input[src] != b'-' {
                    let cursor = TextCursor::new_prim(src as _);
                    return Err(TextParseError::unexpected_byte(cursor, b'-', Some(input[src])));
                }
                src += 1;
            }
           let hi = unwrap! { some_ok_or? Ascii::hex_digit_value(input[src]),
                TextParseError::invalid_digit(TextCursor::new_prim(src as _)) };
           let lo = unwrap! { some_ok_or? Ascii::hex_digit_value(input[src + 1]),
                TextParseError::invalid_digit(TextCursor::new_prim((src + 1) as _)) };
            bytes[dst] = hi << 4 | lo;
            src += 2;
        }}
        Ok(Self::from_bytes(bytes))
    }
}
impl From<[u8; 16]> for Uuid {
    fn from(bytes: [u8; 16]) -> Self {
        Self::from_bytes(bytes)
    }
}
impl From<Uuid> for [u8; 16] {
    fn from(uuid: Uuid) -> Self {
        uuid.into_bytes()
    }
}
impl From<u128> for Uuid {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}
impl From<Uuid> for u128 {
    fn from(uuid: Uuid) -> Self {
        uuid.as_u128()
    }
}
impl_trait! { FromStr<TextParseError> for Uuid |s|
    Self::parse_str(s)
}
