// devela/src/data/id/uuid/non_nil.rs
//
//! Defines [`UuidNonNil`].
//

use crate::{NonZeroU128, Uuid, UuidVariant, UuidVersion, impl_trait};

#[doc = crate::_tags!(uid niche)]
/// A UUID excluding [`Uuid::NIL`] to provide a memory niche.
#[doc = crate::_doc_meta!{
    location("data/id/uuid"),
    test_size_of(UuidNonNil = 16|128; niche Option),
}]
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UuidNonNil(NonZeroU128);

impl UuidNonNil {
    /* constants */

    /// The maximum UUID, with all bits set to one.
    pub const MAX: Self = Self(NonZeroU128::new(u128::MAX).unwrap());

    /* construction */

    /// Creates a non-nil UUID from its 16-byte representation.
    ///
    /// Returns `None` for [`Uuid::NIL`].
    pub const fn from_bytes(bytes: [u8; 16]) -> Option<Self> {
        Self::from_u128(u128::from_be_bytes(bytes))
    }
    /// Creates a non-nil UUID from a `u128` interpreted in big-endian byte order.
    ///
    /// Returns `None` for zero.
    pub const fn from_u128(value: u128) -> Option<Self> {
        match NonZeroU128::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
    /// Creates a non-nil UUID from a [`Uuid`].
    ///
    /// Returns `None` for [`Uuid::NIL`].
    pub const fn from_uuid(uuid: Uuid) -> Option<Self> {
        Self::from_u128(uuid.as_u128())
    }

    /* representation */

    /// Returns the 16-byte UUID representation.
    pub const fn into_bytes(self) -> [u8; 16] {
        self.0.get().to_be_bytes()
    }
    /// Returns this UUID as a big-endian `u128`.
    pub const fn as_u128(self) -> u128 {
        self.0.get()
    }
    /// Returns this value as an unrestricted [`Uuid`].
    pub const fn into_uuid(self) -> Uuid {
        Uuid::from_u128(self.as_u128())
    }

    /* classification */

    /// Returns whether this is the maximum UUID.
    pub const fn is_max(self) -> bool {
        self.as_u128() == u128::MAX
    }
    /// Returns the UUID variant.
    pub const fn variant(self) -> UuidVariant {
        self.into_uuid().variant()
    }

    /// Returns the version number for an IETF UUID.
    ///
    /// Returns `None` for UUIDs belonging to another variant.
    pub const fn version_number(self) -> Option<u8> {
        self.into_uuid().version_number()
    }
    /// Returns the recognized UUID version.
    pub const fn version(self) -> Option<UuidVersion> {
        self.into_uuid().version()
    }
}

/* trait impls */

impl_trait! { fmt::Display for UuidNonNil |self, f| {
    self.into_uuid().fmt(f)
}}

impl From<UuidNonNil> for Uuid {
    fn from(uuid: UuidNonNil) -> Self {
        uuid.into_uuid()
    }
}
impl From<UuidNonNil> for [u8; 16] {
    fn from(uuid: UuidNonNil) -> Self {
        uuid.into_bytes()
    }
}
impl From<UuidNonNil> for u128 {
    fn from(uuid: UuidNonNil) -> Self {
        uuid.as_u128()
    }
}
