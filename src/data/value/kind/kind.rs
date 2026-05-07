// devela::data::value::kind::kind
//
//! Defines [`ValueKind`].
//

// (22 variants)
#[doc = crate::_tags!(data value)]
/// The semantic category of a value word.
#[doc = crate::_doc_location!("data/value")]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValueKind {
    /* 4-bit compact universal band 0..=15 */
    /// No value.
    #[default]
    Nil = 0,
    /// A boolean truth value.
    Bool = 1,
    /// A signed integer value of unspecified width.
    Int = 2,
    /// An unsigned integer value of unspecified width.
    UInt = 3,
    /// A floating-point numeric value of unspecified width.
    Float = 4,
    /// A character-like scalar value.
    Char = 5,
    /// An interned or externally resolved symbolic identifier.
    Symbol = 6,
    /// A closed local choice from a finite set.
    Enum = 7,
    //
    /// A generic reference into another memory domain.
    Ref = 8,
    /// A byte sequence or byte-oriented value.
    Bytes = 9,
    /// A text or string-like value.
    Text = 10,
    /// An ordered aggregate.
    List = 11,
    /// A membership aggregate.
    Set = 12,
    /// A relational, record-like, or columnar aggregate.
    Table = 13,
    /// A callable, closure, function, command, or executable binding.
    Callable = 14,
    /// A value not directly expressible by the local layout or profile.
    Escape = 15,

    /* 6-bit band */
    /// A domain-specific or externally interpreted body.
    Object = 32,
    /// A resource-like body managed outside the value itself.
    Resource = 33,
    /// A time-shaped value such as a point, span, date, duration, or tick.
    Temporal = 34,
    /// A color value or visual scalar.
    Color = 35,
    /// A minimal two-part composition.
    Pair = 36,

    /* 8-bit band */
    /// An unknown, invalid, or profile-unmapped kind.
    Unknown = 255,
}
impl ValueKind {
    /// Returns the raw 8-bit kind code.
    #[must_use]
    #[inline(always)]
    pub const fn code(self) -> u8 {
        self as u8
    }
    /// Returns a kind from its raw 8-bit code.
    ///
    /// Unknown codes map to [`Unknown`][Self::Unknown].
    #[must_use]
    #[inline(always)]
    pub const fn from_code(code: u8) -> Self {
        match code {
            0 => Self::Nil,
            1 => Self::Bool,
            2 => Self::Int,
            3 => Self::UInt,
            4 => Self::Float,
            5 => Self::Char,
            6 => Self::Symbol,
            7 => Self::Enum,
            //
            8 => Self::Ref,
            9 => Self::Bytes,
            10 => Self::Text,
            11 => Self::List,
            12 => Self::Set,
            13 => Self::Table,
            14 => Self::Callable,
            15 => Self::Escape,
            //
            32 => Self::Object,
            33 => Self::Resource,
            34 => Self::Temporal,
            35 => Self::Color,
            36 => Self::Pair,
            //
            255 => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}
impl ValueKind {
    /// Returns whether this kind belongs to the compact universal band.
    #[must_use]
    #[inline(always)]
    pub const fn is_compact(self) -> bool {
        self.code() <= 15
    }
    /// Returns whether this kind represents absence.
    #[must_use]
    #[inline(always)]
    pub const fn is_nil(self) -> bool {
        matches!(self, Self::Nil)
    }
    /// Returns whether this kind represents a numeric scalar.
    #[must_use]
    #[inline(always)]
    pub const fn is_number(self) -> bool {
        matches!(self, Self::Int | Self::UInt | Self::Float)
    }
    /// Returns whether this kind represents a scalar-like atom.
    #[must_use]
    #[inline(always)]
    pub const fn is_scalar(self) -> bool {
        matches!(
            self,
            Self::Nil
                | Self::Bool
                | Self::Int
                | Self::UInt
                | Self::Float
                | Self::Char
                | Self::Symbol
                | Self::Enum
        )
    }
    /// Returns whether this kind points outside the immediate value body.
    #[must_use]
    #[inline(always)]
    pub const fn is_ref_like(self) -> bool {
        matches!(self, Self::Ref)
    }
    /// Returns whether this kind describes aggregate data.
    #[must_use]
    #[inline(always)]
    pub const fn is_aggregate(self) -> bool {
        matches!(self, Self::Bytes | Self::Set | Self::Text | Self::List | Self::Table)
    }
    /// Returns whether this kind describes callable behavior.
    #[must_use]
    #[inline(always)]
    pub const fn is_callable(self) -> bool {
        matches!(self, Self::Callable)
    }
    /// Returns whether this kind describes a domain-specific body.
    #[must_use]
    #[inline(always)]
    pub const fn is_object(self) -> bool {
        matches!(self, Self::Object)
    }
    /// Returns whether this kind describes an externally managed resource.
    #[must_use]
    #[inline(always)]
    pub const fn is_resource(self) -> bool {
        matches!(self, Self::Resource)
    }
    /// Returns whether this kind needs profile-specific decoding or widening.
    #[must_use]
    #[inline(always)]
    pub const fn is_escape(self) -> bool {
        matches!(self, Self::Escape)
    }
    /// Returns whether this kind is unknown or unmapped.
    #[must_use]
    #[inline(always)]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}
