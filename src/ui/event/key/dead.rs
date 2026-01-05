// devela::ui::event::key::dead
//
//! Defines [`KeyDead`].
//
// Every dead keysym defined across all X11/XKB headers falls into one of two disjoint ranges:
// - Primary dead-key block (32 entries)
// - Dead-vowel and Greek block (13 entries)

#![allow(missing_docs, unused, non_camel_case_types)]

use crate::{ConstInit, NonZeroU32, impl_trait, is};

#[doc = crate::_TAG_INTERACTION!()]
/// Dead-key accent operators used during text composition.
#[doc = crate::_doc_location!("ui/event")]
///
/// Modeled after XKB dead keysym from `xkbcommon-keysyms.h`.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyDead {
    /* 32 dead vowels */
    Grave = 0, // keysym:fe50
    Acute,
    Circumflex,
    Tilde,
    Macron,
    Breve,
    DotAbove,
    Diaeresis,
    RingAbove,
    DoubleAcute,
    Caron,
    Cedilla,
    Ogonek,
    Iota,
    VoicedSound,
    SemiVoicedSound,
    DotBelow,
    Hook,
    Horn,
    Stroke,
    CommaAbove,
    ReversedCommaAbove,
    DoubleGrave,
    RingBelow,
    MacronBelow,
    CircumflexBelow,
    TildeBelow,
    BreveBelow,
    DiaeresisBelow,
    InvertedBreve,
    CommaBelow,
    Currency = 31, // keysym:fe6f
    /* gap of 16 */
    /* 13 dead vowels for universal syllable entry */
    DeadA = 31 + 16, // = 48; keysym:fe80
    DeadA_Cap,
    DeadE,
    DeadE_Cap,
    DeadI,
    DeadI_Cap,
    DeadO,
    DeadO_Cap,
    DeadU,
    DeadU_Cap,
    DeadSmallSchwa,
    DeadCapitalSchwa,
    DeadGreek = 31 + 16 + 13, // = 60 keysym:fe8c
    /* gap of 3 */
    /* 4 extra dead elements for German T3 layout */
    LowLine = 31 + 16 + 13 + 4, // = 64 fe90
    VerticalLineAbove,
    VerticalLineBelow,
    LongSolidusOverlay, // = 67 0xfe93

    /// Not part of the known dead-key set.
    Unknown = u8::MAX,
}
impl ConstInit for KeyDead {
    const INIT: Self = KeyDead::Unknown;
}

/// Aliases.
#[allow(non_upper_case_globals)]
impl KeyDead {
    pub const Perispomeni: Self = Self::Tilde;
    pub const Psili: Self = Self::CommaAbove;
    pub const Dasia: Self = Self::ReversedCommaAbove;
}

impl KeyDead {
    // Valid dead-key range: FE50 ..= FE93  (offset 0 ..= 67)
    const BASE: u32 = 0xFE50;

    /// Converts a dead-key keysym into a `KeyDead`.
    pub const fn from_keysym(sym: u32) -> KeyDead {
        if sym.wrapping_sub(Self::BASE) < DEAD_TO_SYM.len() as u32 {
            DEAD_TO_SYM[(sym - Self::BASE) as usize] // safe: offset in [0..67]
        } else {
            KeyDead::Unknown
        }
    }

    /// Returns the canonical dead-key keysym for this variant.
    #[inline(always)]
    pub const fn to_keysym(kind: KeyDead) -> u32 {
        match kind {
            KeyDead::Unknown => 0,
            _ => 0xFE50 + (kind as u8 as u32),
        }
    }
}

// Helper LUT
const DEAD_TO_SYM: [KeyDead; 68] = [
    KeyDead::Grave, // keysym:fe50
    KeyDead::Acute,
    KeyDead::Circumflex,
    KeyDead::Tilde,
    KeyDead::Macron,
    KeyDead::Breve,
    KeyDead::DotAbove,
    KeyDead::Diaeresis,
    KeyDead::RingAbove,
    KeyDead::DoubleAcute,
    KeyDead::Caron,
    KeyDead::Cedilla,
    KeyDead::Ogonek,
    KeyDead::Iota,
    KeyDead::VoicedSound,
    KeyDead::SemiVoicedSound,
    KeyDead::DotBelow,
    KeyDead::Hook,
    KeyDead::Horn,
    KeyDead::Stroke,
    KeyDead::CommaAbove,
    KeyDead::ReversedCommaAbove,
    KeyDead::DoubleGrave,
    KeyDead::RingBelow,
    KeyDead::MacronBelow,
    KeyDead::CircumflexBelow,
    KeyDead::TildeBelow,
    KeyDead::BreveBelow,
    KeyDead::DiaeresisBelow,
    KeyDead::InvertedBreve,
    KeyDead::CommaBelow,
    KeyDead::Currency, // keysym:fe6f
    /* gap of 16 */
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    /* block of 13 */
    KeyDead::DeadA, // keysym:fe80
    KeyDead::DeadA_Cap,
    KeyDead::DeadE,
    KeyDead::DeadE_Cap,
    KeyDead::DeadI,
    KeyDead::DeadI_Cap,
    KeyDead::DeadO,
    KeyDead::DeadO_Cap,
    KeyDead::DeadU,
    KeyDead::DeadU_Cap,
    KeyDead::DeadSmallSchwa,
    KeyDead::DeadCapitalSchwa,
    KeyDead::DeadGreek, // keysym:fe8c
    /* gap of 3 */
    KeyDead::Unknown,
    KeyDead::Unknown,
    KeyDead::Unknown,
    /* block of 4 */
    KeyDead::LowLine, // fe90
    KeyDead::VerticalLineAbove,
    KeyDead::VerticalLineBelow,
    KeyDead::LongSolidusOverlay, // 0xfe93
];
