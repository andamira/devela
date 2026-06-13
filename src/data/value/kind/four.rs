// devela/src/data/value/kind/four.rs
//
//! Defines [`ValueKind4`].
//

use crate::ValueKind;

// (16 variants)
#[doc = crate::_tags!(data value)]
/// A 4-bit compact value category.
#[doc = crate::_doc_meta!{location("data/value")}]
///
/// It mirrors the compact universal band of [`ValueKind`].
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValueKind4 {
    /// No value.
    #[default]
    Nil = 0,
    /// A boolean truth value.
    Bool = 1,
    /// A signed integer value.
    Int = 2,
    /// An unsigned integer value.
    UInt = 3,
    /// A floating-point numeric value.
    Float = 4,
    /// A character-like scalar value.
    Char = 5,
    /// An interned or externally resolved symbolic identifier.
    Symbol = 6,
    /// A closed local choice from a finite set.
    Enum = 7,

    /// A generic reference into another memory domain.
    Ref = 8,
    /// A byte sequence or byte-oriented value.
    Bytes = 9,
    /// A text or string-like value.
    Text = 10,
    /// A sequential aggregate.
    List = 11,
    /// A membership aggregate.
    Set = 12,
    /// A tabular, record-like, or columnar aggregate.
    Table = 13,
    /// A callable, closure, function, or executable binding.
    Callable = 14,
    /// A value not directly expressible by the local layout or profile.
    Escape = 15,
}
impl ValueKind4 {
    /// Returns the raw 4-bit kind code.
    #[must_use]
    #[inline(always)]
    pub const fn code(self) -> u8 {
        self as u8
    }
    /// Returns a compact kind from a raw 4-bit code.
    #[must_use]
    #[inline(always)]
    pub const fn from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(Self::Nil),
            1 => Some(Self::Bool),
            2 => Some(Self::Int),
            3 => Some(Self::UInt),
            4 => Some(Self::Float),
            5 => Some(Self::Char),
            6 => Some(Self::Symbol),
            7 => Some(Self::Enum),
            8 => Some(Self::Ref),
            9 => Some(Self::Bytes),
            10 => Some(Self::Text),
            11 => Some(Self::List),
            12 => Some(Self::Set),
            13 => Some(Self::Table),
            14 => Some(Self::Callable),
            15 => Some(Self::Escape),
            _ => None,
        }
    }
    /// Widens this compact kind to [`ValueKind`].
    #[must_use]
    #[inline(always)]
    pub const fn to_kind(self) -> ValueKind {
        ValueKind::from_code(self.code())
    }
    /// Narrows a [`ValueKind`] if it belongs to the compact universal band.
    #[must_use]
    #[inline(always)]
    pub const fn from_kind(kind: ValueKind) -> Option<Self> {
        if kind.is_compact() { Self::from_code(kind.code()) } else { None }
    }
}
