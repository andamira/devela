// devela/src/data/id/uuid/variant.rs
//
//! Defines [`UuidVariant`], [`UuidVersion`].
//

#[cfg(doc)]
use crate::Uuid;

#[doc = crate::_tags!(uid)]
/// The layout variant of a [`Uuid`].
#[doc = crate::_doc_meta!{
    location("data/id/uuid", enum UuidVariant),
    test_size_of(UuidVariant = 1|8; niche Option),
}]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UuidVariant {
    /// The legacy Network Computing System variant.
    Ncs,

    /// The OSF DCE / IETF UUID variant.
    Ietf,

    /// The legacy Microsoft variant.
    Microsoft,

    /// The variant reserved for future definition.
    Future,
}

#[doc = crate::_tags!(uid)]
/// A recognized IETF UUID version.
#[doc = crate::_doc_meta!{
    location("data/id/uuid", enum UuidVersion),
    test_size_of(UuidVersion = 1|8; niche Option),
}]
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UuidVersion {
    /// Version 1: Gregorian time-based.
    V1 = 1,

    /// Version 2: reserved for DCE Security.
    V2 = 2,

    /// Version 3: name-based using MD5.
    V3 = 3,

    /// Version 4: randomly or pseudorandomly generated.
    V4 = 4,

    /// Version 5: name-based using SHA-1.
    V5 = 5,

    /// Version 6: reordered Gregorian time-based.
    V6 = 6,

    /// Version 7: Unix Epoch time-based.
    V7 = 7,

    /// Version 8: custom.
    V8 = 8,
}
impl UuidVersion {
    /// Returns the standardized version number.
    pub const fn number(self) -> u8 {
        self as u8
    }
    /// Returns a recognized version from its standardized number.
    pub const fn from_number(number: u8) -> Option<Self> {
        match number {
            1 => Some(Self::V1),
            2 => Some(Self::V2),
            3 => Some(Self::V3),
            4 => Some(Self::V4),
            5 => Some(Self::V5),
            6 => Some(Self::V6),
            7 => Some(Self::V7),
            8 => Some(Self::V8),
            _ => None,
        }
    }
}
