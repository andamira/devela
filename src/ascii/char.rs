// devela::ascii::char
//
//!
//
// Ported from:
// - https://doc.rust-lang.org/stable/core/ascii/enum.Char.html
// - WAITING: https://github.com/rust-lang/rust/issues/110998

use core::fmt;
#[cfg(feature = "unsafe_ascii")]
use core::mem::transmute;

/// One of the 128 Unicode characters from U+0000 through U+007F,
/// often known as the [ASCII] subset.
///
/// Officially, this is the first [block] in Unicode, _Basic Latin_.
/// For details, see the [*C0 Controls and Basic Latin*][chart] code chart.
///
/// This block was based on older 7-bit character code standards such as
/// ANSI X3.4-1977, ISO 646-1973, and [NIST FIPS 1-2].
///
/// # When to use this
///
/// The main advantage of this subset is that it's always valid UTF-8.  As such,
/// the `&[ascii::AsciiChar]` -> `&str` conversion function (as well as other related
/// ones) are O(1): *no* runtime checks are needed.
///
/// If you're consuming strings, you should usually handle Unicode and thus
/// accept `str`s, not limit yourself to `ascii::AsciiChar`s.
///
/// However, certain formats are intentionally designed to produce ASCII-only
/// output in order to be 8-bit-clean.  In those cases, it can be simpler and
/// faster to generate `ascii::AsciiChar`s instead of dealing with the variable width
/// properties of general UTF-8 encoded strings, while still allowing the result
/// to be used freely with other Rust things that deal in general `str`s.
///
/// For example, a UUID library might offer a way to produce the string
/// representation of a UUID as an `[ascii::AsciiChar; 36]` to avoid memory
/// allocation yet still allow it to be used as UTF-8 via `as_str` without
/// paying for validation (or needing `unsafe` code) the way it would if it
/// were provided as a `[u8; 36]`.
///
/// # Layout
///
/// This type is guaranteed to have a size and alignment of 1 byte.
///
/// # Names
///
/// The variants on this type are [Unicode names][NamesList] of the characters
/// in upper camel case, with a few tweaks:
/// - For `<control>` characters, the primary alias name is used.
/// - `LATIN` is dropped, as this block has no non-latin letters.
/// - `LETTER` is dropped, as `CAPITAL`/`SMALL` suffices in this block.
/// - `DIGIT`s use a single digit rather than writing out `ZERO`, `ONE`, etc.
///
/// [ASCII]: https://www.unicode.org/glossary/index.html#ASCII
/// [block]: https://www.unicode.org/glossary/index.html#block
/// [chart]: https://www.unicode.org/charts/PDF/U0000.pdf
/// [NIST FIPS 1-2]: https://nvlpubs.nist.gov/nistpubs/Legacy/FIPS/fipspub1-2-1977.pdf
/// [NamesList]: https://www.unicode.org/Public/15.0.0/ucd/NamesList.txt
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum AsciiChar {
    /// U+0000
    Null = 0,
    /// U+0001
    StartOfHeading = 1,
    /// U+0002
    StartOfText = 2,
    /// U+0003
    EndOfText = 3,
    /// U+0004
    EndOfTransmission = 4,
    /// U+0005
    Enquiry = 5,
    /// U+0006
    Acknowledge = 6,
    /// U+0007
    Bell = 7,
    /// U+0008
    Backspace = 8,
    /// U+0009
    AsciiCharacterTabulation = 9,
    /// U+000A
    LineFeed = 10,
    /// U+000B
    LineTabulation = 11,
    /// U+000C
    FormFeed = 12,
    /// U+000D
    CarriageReturn = 13,
    /// U+000E
    ShiftOut = 14,
    /// U+000F
    ShiftIn = 15,
    /// U+0010
    DataLinkEscape = 16,
    /// U+0011
    DeviceControlOne = 17,
    /// U+0012
    DeviceControlTwo = 18,
    /// U+0013
    DeviceControlThree = 19,
    /// U+0014
    DeviceControlFour = 20,
    /// U+0015
    NegativeAcknowledge = 21,
    /// U+0016
    SynchronousIdle = 22,
    /// U+0017
    EndOfTransmissionBlock = 23,
    /// U+0018
    Cancel = 24,
    /// U+0019
    EndOfMedium = 25,
    /// U+001A
    Substitute = 26,
    /// U+001B
    Escape = 27,
    /// U+001C
    InformationSeparatorFour = 28,
    /// U+001D
    InformationSeparatorThree = 29,
    /// U+001E
    InformationSeparatorTwo = 30,
    /// U+001F
    InformationSeparatorOne = 31,
    /// U+0020
    Space = 32,
    /// U+0021
    ExclamationMark = 33,
    /// U+0022
    QuotationMark = 34,
    /// U+0023
    NumberSign = 35,
    /// U+0024
    DollarSign = 36,
    /// U+0025
    PercentSign = 37,
    /// U+0026
    Ampersand = 38,
    /// U+0027
    Apostrophe = 39,
    /// U+0028
    LeftParenthesis = 40,
    /// U+0029
    RightParenthesis = 41,
    /// U+002A
    Asterisk = 42,
    /// U+002B
    PlusSign = 43,
    /// U+002C
    Comma = 44,
    /// U+002D
    HyphenMinus = 45,
    /// U+002E
    FullStop = 46,
    /// U+002F
    Solidus = 47,
    /// U+0030
    Digit0 = 48,
    /// U+0031
    Digit1 = 49,
    /// U+0032
    Digit2 = 50,
    /// U+0033
    Digit3 = 51,
    /// U+0034
    Digit4 = 52,
    /// U+0035
    Digit5 = 53,
    /// U+0036
    Digit6 = 54,
    /// U+0037
    Digit7 = 55,
    /// U+0038
    Digit8 = 56,
    /// U+0039
    Digit9 = 57,
    /// U+003A
    Colon = 58,
    /// U+003B
    Semicolon = 59,
    /// U+003C
    LessThanSign = 60,
    /// U+003D
    EqualsSign = 61,
    /// U+003E
    GreaterThanSign = 62,
    /// U+003F
    QuestionMark = 63,
    /// U+0040
    CommercialAt = 64,
    /// U+0041
    CapitalA = 65,
    /// U+0042
    CapitalB = 66,
    /// U+0043
    CapitalC = 67,
    /// U+0044
    CapitalD = 68,
    /// U+0045
    CapitalE = 69,
    /// U+0046
    CapitalF = 70,
    /// U+0047
    CapitalG = 71,
    /// U+0048
    CapitalH = 72,
    /// U+0049
    CapitalI = 73,
    /// U+004A
    CapitalJ = 74,
    /// U+004B
    CapitalK = 75,
    /// U+004C
    CapitalL = 76,
    /// U+004D
    CapitalM = 77,
    /// U+004E
    CapitalN = 78,
    /// U+004F
    CapitalO = 79,
    /// U+0050
    CapitalP = 80,
    /// U+0051
    CapitalQ = 81,
    /// U+0052
    CapitalR = 82,
    /// U+0053
    CapitalS = 83,
    /// U+0054
    CapitalT = 84,
    /// U+0055
    CapitalU = 85,
    /// U+0056
    CapitalV = 86,
    /// U+0057
    CapitalW = 87,
    /// U+0058
    CapitalX = 88,
    /// U+0059
    CapitalY = 89,
    /// U+005A
    CapitalZ = 90,
    /// U+005B
    LeftSquareBracket = 91,
    /// U+005C
    ReverseSolidus = 92,
    /// U+005D
    RightSquareBracket = 93,
    /// U+005E
    CircumflexAccent = 94,
    /// U+005F
    LowLine = 95,
    /// U+0060
    GraveAccent = 96,
    /// U+0061
    SmallA = 97,
    /// U+0062
    SmallB = 98,
    /// U+0063
    SmallC = 99,
    /// U+0064
    SmallD = 100,
    /// U+0065
    SmallE = 101,
    /// U+0066
    SmallF = 102,
    /// U+0067
    SmallG = 103,
    /// U+0068
    SmallH = 104,
    /// U+0069
    SmallI = 105,
    /// U+006A
    SmallJ = 106,
    /// U+006B
    SmallK = 107,
    /// U+006C
    SmallL = 108,
    /// U+006D
    SmallM = 109,
    /// U+006E
    SmallN = 110,
    /// U+006F
    SmallO = 111,
    /// U+0070
    SmallP = 112,
    /// U+0071
    SmallQ = 113,
    /// U+0072
    SmallR = 114,
    /// U+0073
    SmallS = 115,
    /// U+0074
    SmallT = 116,
    /// U+0075
    SmallU = 117,
    /// U+0076
    SmallV = 118,
    /// U+0077
    SmallW = 119,
    /// U+0078
    SmallX = 120,
    /// U+0079
    SmallY = 121,
    /// U+007A
    SmallZ = 122,
    /// U+007B
    LeftCurlyBracket = 123,
    /// U+007C
    VerticalLine = 124,
    /// U+007D
    RightCurlyBracket = 125,
    /// U+007E
    Tilde = 126,
    /// U+007F
    Delete = 127,
}

impl AsciiChar {
    /// Creates an ascii character from the byte `b`,
    /// or returns `None` if it's too large.
    #[inline]
    pub const fn from_u8(b: u8) -> Option<Self> {
        match b {
            0 => Some(Self::Null),
            1 => Some(Self::StartOfHeading),
            2 => Some(Self::StartOfText),
            3 => Some(Self::EndOfText),
            4 => Some(Self::EndOfTransmission),
            5 => Some(Self::Enquiry),
            6 => Some(Self::Acknowledge),
            7 => Some(Self::Bell),
            8 => Some(Self::Backspace),
            9 => Some(Self::AsciiCharacterTabulation),
            10 => Some(Self::LineFeed),
            11 => Some(Self::LineTabulation),
            12 => Some(Self::FormFeed),
            13 => Some(Self::CarriageReturn),
            14 => Some(Self::ShiftOut),
            15 => Some(Self::ShiftIn),
            16 => Some(Self::DataLinkEscape),
            17 => Some(Self::DeviceControlOne),
            18 => Some(Self::DeviceControlTwo),
            19 => Some(Self::DeviceControlThree),
            20 => Some(Self::DeviceControlFour),
            21 => Some(Self::NegativeAcknowledge),
            22 => Some(Self::SynchronousIdle),
            23 => Some(Self::EndOfTransmissionBlock),
            24 => Some(Self::Cancel),
            25 => Some(Self::EndOfMedium),
            26 => Some(Self::Substitute),
            27 => Some(Self::Escape),
            28 => Some(Self::InformationSeparatorFour),
            29 => Some(Self::InformationSeparatorThree),
            30 => Some(Self::InformationSeparatorTwo),
            31 => Some(Self::InformationSeparatorOne),
            32 => Some(Self::Space),
            33 => Some(Self::ExclamationMark),
            34 => Some(Self::QuotationMark),
            35 => Some(Self::NumberSign),
            36 => Some(Self::DollarSign),
            37 => Some(Self::PercentSign),
            38 => Some(Self::Ampersand),
            39 => Some(Self::Apostrophe),
            40 => Some(Self::LeftParenthesis),
            41 => Some(Self::RightParenthesis),
            42 => Some(Self::Asterisk),
            43 => Some(Self::PlusSign),
            44 => Some(Self::Comma),
            45 => Some(Self::HyphenMinus),
            46 => Some(Self::FullStop),
            47 => Some(Self::Solidus),
            48 => Some(Self::Digit0),
            49 => Some(Self::Digit1),
            50 => Some(Self::Digit2),
            51 => Some(Self::Digit3),
            52 => Some(Self::Digit4),
            53 => Some(Self::Digit5),
            54 => Some(Self::Digit6),
            55 => Some(Self::Digit7),
            56 => Some(Self::Digit8),
            57 => Some(Self::Digit9),
            58 => Some(Self::Colon),
            59 => Some(Self::Semicolon),
            60 => Some(Self::LessThanSign),
            61 => Some(Self::EqualsSign),
            62 => Some(Self::GreaterThanSign),
            63 => Some(Self::QuestionMark),
            64 => Some(Self::CommercialAt),
            65 => Some(Self::CapitalA),
            66 => Some(Self::CapitalB),
            67 => Some(Self::CapitalC),
            68 => Some(Self::CapitalD),
            69 => Some(Self::CapitalE),
            70 => Some(Self::CapitalF),
            71 => Some(Self::CapitalG),
            72 => Some(Self::CapitalH),
            73 => Some(Self::CapitalI),
            74 => Some(Self::CapitalJ),
            75 => Some(Self::CapitalK),
            76 => Some(Self::CapitalL),
            77 => Some(Self::CapitalM),
            78 => Some(Self::CapitalN),
            79 => Some(Self::CapitalO),
            80 => Some(Self::CapitalP),
            81 => Some(Self::CapitalQ),
            82 => Some(Self::CapitalR),
            83 => Some(Self::CapitalS),
            84 => Some(Self::CapitalT),
            85 => Some(Self::CapitalU),
            86 => Some(Self::CapitalV),
            87 => Some(Self::CapitalW),
            88 => Some(Self::CapitalX),
            89 => Some(Self::CapitalY),
            90 => Some(Self::CapitalZ),
            91 => Some(Self::LeftSquareBracket),
            92 => Some(Self::ReverseSolidus),
            93 => Some(Self::RightSquareBracket),
            94 => Some(Self::CircumflexAccent),
            95 => Some(Self::LowLine),
            96 => Some(Self::GraveAccent),
            97 => Some(Self::SmallA),
            98 => Some(Self::SmallB),
            99 => Some(Self::SmallC),
            100 => Some(Self::SmallD),
            101 => Some(Self::SmallE),
            102 => Some(Self::SmallF),
            103 => Some(Self::SmallG),
            104 => Some(Self::SmallH),
            105 => Some(Self::SmallI),
            106 => Some(Self::SmallJ),
            107 => Some(Self::SmallK),
            108 => Some(Self::SmallL),
            109 => Some(Self::SmallM),
            110 => Some(Self::SmallN),
            111 => Some(Self::SmallO),
            112 => Some(Self::SmallP),
            113 => Some(Self::SmallQ),
            114 => Some(Self::SmallR),
            115 => Some(Self::SmallS),
            116 => Some(Self::SmallT),
            117 => Some(Self::SmallU),
            118 => Some(Self::SmallV),
            119 => Some(Self::SmallW),
            120 => Some(Self::SmallX),
            121 => Some(Self::SmallY),
            122 => Some(Self::SmallZ),
            123 => Some(Self::LeftCurlyBracket),
            124 => Some(Self::VerticalLine),
            125 => Some(Self::RightCurlyBracket),
            126 => Some(Self::Tilde),
            127 => Some(Self::Delete),
            _ => None,
        }
    }

    /// Creates an ASCII character from the byte `b`,
    /// without checking whether it's valid.
    ///
    /// # Safety
    /// `b` must be in `0..=127`, or else this is UB.
    #[inline]
    #[cfg(feature = "unsafe_ascii")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_ascii")))]
    pub const unsafe fn from_u8_unchecked(b: u8) -> Self {
        // SAFETY: Our safety precondition is that `b` is in-range.
        unsafe { transmute(b) }
    }

    /// When passed the *number* `0`, `1`, …, `9`, returns the *character*
    /// `'0'`, `'1'`, …, `'9'` respectively.
    ///
    /// If `d >= 10`, returns `None`.
    #[inline]
    pub const fn digit(d: u8) -> Option<Self> {
        if d < 10 {
            Self::from_u8(b'0' + d)
        } else {
            None
        }
    }

    /// When passed the *number* `0`, `1`, …, `9`, returns the *character*
    /// `'0'`, `'1'`, …, `'9'` respectively, without checking that it's in-range.
    ///
    /// # Safety
    /// This is immediate UB if called with `d > 64`.
    ///
    /// If `d >= 10` and `d <= 64`, this is allowed to return any value or panic.
    /// Notably, it should not be expected to return hex digits, or any other
    /// reasonable extension of the decimal digits.
    ///
    /// (This lose safety condition is intended to simplify soundness proofs
    /// when writing code using this method, since the implementation doesn't
    /// need something really specific, not to make those other arguments do
    /// something useful. It might be tightened before stabilization.)
    #[inline]
    #[cfg(feature = "unsafe_ascii")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_ascii")))]
    pub const unsafe fn digit_unchecked(d: u8) -> Self {
        debug_assert!(d < 10);

        // SAFETY: `'0'` through `'9'` are U+00030 through U+0039,
        // so because `d` must be 64 or less the addition can return at most
        // 112 (0x70), which doesn't overflow and is within the ASCII range.
        unsafe {
            // WAITING: https://github.com/rust-lang/rust/issues/85122
            // let byte = b'0'.unchecked_add(d);
            let byte = b'0' + d;
            Self::from_u8_unchecked(byte)
        }
    }

    /// Gets this ASCII character as a byte.
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Gets this ASCII character as a `char` Unicode Scalar Value.
    #[inline]
    pub const fn as_char(self) -> char {
        self as u8 as char
    }

    /// Views this ASCII character as a one-code-unit UTF-8 `str`.
    #[inline]
    #[cfg(feature = "unsafe_ascii")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_ascii")))]
    pub const fn as_str(&self) -> &str {
        Self::slice_as_str(core::slice::from_ref(self))
    }
}

impl AsciiChar {
    /// Views a slice of ASCII characters as a UTF-8 `str`.
    #[inline]
    #[cfg(feature = "unsafe_ascii")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_ascii")))]
    pub const fn slice_as_str(slice: &[AsciiChar]) -> &str {
        let ascii_ptr: *const [AsciiChar] = slice;
        let str_ptr = ascii_ptr as *const str;
        // SAFETY: Each ASCII codepoint in UTF-8 is encoded as one single-byte
        // code unit having the same value as the ASCII byte.
        unsafe { &*str_ptr }
    }

    /// Views a slice of ASCII characters as a slice of `u8` bytes.
    #[inline]
    #[cfg(feature = "unsafe_ascii")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_ascii")))]
    pub const fn slice_as_bytes(slice: &[AsciiChar]) -> &[u8] {
        AsciiChar::slice_as_str(slice).as_bytes()
    }
}
// impl [AsciiChar] {
//     /// Views this slice of ASCII characters as a UTF-8 `str`.
//     #[inline]
//     pub const fn as_str(&self) -> &str {
//         let ascii_ptr: *const Self = self;
//         let str_ptr = ascii_ptr as *const str;
//         // SAFETY: Each ASCII codepoint in UTF-8 is encoded as one single-byte
//         // code unit having the same value as the ASCII byte.
//         unsafe { &*str_ptr }
//     }
//
//     /// Views this slice of ASCII characters as a slice of `u8` bytes.
//     #[inline]
//     pub const fn as_bytes(&self) -> &[u8] {
//         self.as_str().as_bytes()
//     }
// }

impl fmt::Display for AsciiChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_char(), f)
    }
}
