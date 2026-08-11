// devela/src/data/id/uuid/define.rs
//
//! Defines [`Uuid`].
//

use crate::{ConstInit, UuidNonNil, UuidVariant, UuidVersion};

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

impl Default for Uuid {
    fn default() -> Self {
        Self::NIL
    }
}
impl ConstInit for Uuid {
    const INIT: Self = Self::NIL;
}

impl Uuid {
    /* constants */

    /// The nil UUID, with all bits set to zero.
    pub const NIL: Self = Self([0; 16]);

    /// The maximum UUID, with all bits set to one.
    pub const MAX: Self = Self([0xFF; 16]);

    /* construction */

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
