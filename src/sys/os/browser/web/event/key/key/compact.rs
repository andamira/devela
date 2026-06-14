// devela/src/sys/os/browser/web/event/key/key/compact.rs
//
//! implements methods for [`Key`].
//
// IMPROVE: could we use LUTs? an 8bit for NAMED, and maybe a 4bit for PAD?

use crate::{Key, KeyMedia, KeyMod, KeyPad, WebKeyLocation, is, js_uint32};

/// Compact key code sent by JavaScript keyboard callbacks.
pub type WebKeyCode = js_uint32;

pub const WEB_KEY_UNKNOWN: u32 = 0x0000_0000;
pub const WEB_KEY_CHAR: u32 = 0x0100_0000;
pub const WEB_KEY_NAMED: u32 = 0x0200_0000;
pub const WEB_KEY_FN: u32 = 0x0300_0000;
pub const WEB_KEY_PAD: u32 = 0x0400_0000;
pub const WEB_KEY_MEDIA: u32 = 0x0500_0000;
pub const WEB_KEY_MOD: u32 = 0x0600_0000;

const WEB_KEY_TAG: u32 = 0xFF00_0000;
const WEB_KEY_VALUE: u32 = 0x00FF_FFFF;

#[rustfmt::skip]
impl Key {
    /// Converts a compact JavaScript `KeyboardEvent.key` token into `Key`.
    #[must_use]
    pub const fn from_web_compact_key(code: WebKeyCode, location: WebKeyLocation) -> Self {
        let (tag, value) = (code & WEB_KEY_TAG, code & WEB_KEY_VALUE);
        match tag {
            WEB_KEY_UNKNOWN => Key::Unknown,
            WEB_KEY_CHAR => is![let Some(c) = char::from_u32(value), Key::Char(c), Key::Unknown],
            WEB_KEY_NAMED => Self::from_web_compact_named(value),
            WEB_KEY_FN => Self::from_web_compact_fn(value),
            WEB_KEY_PAD => Self::from_web_compact_pad(value),
            WEB_KEY_MEDIA => Self::from_web_compact_media(value),
            WEB_KEY_MOD => Self::from_web_compact_mod(value, location),
            _ => Key::Unknown,
        }
    }
    /// Converts a compact JavaScript `KeyboardEvent.code` token into `Key`.
    #[must_use]
    pub const fn from_web_compact_code(code: WebKeyCode, location: WebKeyLocation) -> Self {
        let (tag, value) = (code & WEB_KEY_TAG, code & WEB_KEY_VALUE);
        match tag {
            WEB_KEY_UNKNOWN => Key::Unknown,
            WEB_KEY_CHAR => Key::Unknown, // physical `code` should not produce text chars
            WEB_KEY_NAMED => Self::from_web_compact_named(value),
            WEB_KEY_FN => Self::from_web_compact_fn(value),
            WEB_KEY_PAD => Self::from_web_compact_pad(value),
            WEB_KEY_MEDIA => Self::from_web_compact_media(value),
            WEB_KEY_MOD => Self::from_web_compact_mod(value, location),
            _ => Key::Unknown,
        }
    }
    const fn from_web_compact_fn(value: u32) -> Self {
        if value >= 1 && value <= 48 { Key::Fn(value as u8) } else { Key::Unknown }
    }
    const fn from_web_compact_named(value: u32) -> Self {
        match value {
            1 => Key::Backspace, //
            2 => Key::Enter,
            3 => Key::Tab,
            4 => Key::Escape,
            5 => Key::Space,
            10 => Key::Left, //
            11 => Key::Right,
            12 => Key::Up,
            13 => Key::Down,
            20 => Key::Home, //
            21 => Key::End,
            22 => Key::PageUp,
            23 => Key::PageDown,
            30 => Key::Delete, //
            31 => Key::Insert,
            40 => Key::CapsLock, //
            41 => Key::ScrollLock,
            42 => Key::NumLock,
            50 => Key::PrintScreen, //
            51 => Key::Pause,
            52 => Key::Menu,
            100 => Key::A, //
            101 => Key::B,
            102 => Key::C,
            103 => Key::D,
            104 => Key::E,
            105 => Key::F,
            106 => Key::G,
            107 => Key::H,
            108 => Key::I,
            109 => Key::J,
            110 => Key::K,
            111 => Key::L,
            112 => Key::M,
            113 => Key::N,
            114 => Key::O,
            115 => Key::P,
            116 => Key::Q,
            117 => Key::R,
            118 => Key::S,
            119 => Key::T,
            120 => Key::U,
            121 => Key::V,
            122 => Key::W,
            123 => Key::X,
            124 => Key::Y,
            125 => Key::Z,
            200 => Key::Digit0, //
            201 => Key::Digit1,
            202 => Key::Digit2,
            203 => Key::Digit3,
            204 => Key::Digit4,
            205 => Key::Digit5,
            206 => Key::Digit6,
            207 => Key::Digit7,
            208 => Key::Digit8,
            209 => Key::Digit9,
            _ => Key::Unknown, //
        }
    }
    const fn from_web_compact_pad(value: u32) -> Self {
        match value {
            0 => Key::Pad(KeyPad::Num0), //
            1 => Key::Pad(KeyPad::Num1),
            2 => Key::Pad(KeyPad::Num2),
            3 => Key::Pad(KeyPad::Num3),
            4 => Key::Pad(KeyPad::Num4),
            5 => Key::Pad(KeyPad::Num5),
            6 => Key::Pad(KeyPad::Num6),
            7 => Key::Pad(KeyPad::Num7),
            8 => Key::Pad(KeyPad::Num8),
            9 => Key::Pad(KeyPad::Num9),
            10 => Key::Pad(KeyPad::Multiply), //
            11 => Key::Pad(KeyPad::Add),
            12 => Key::Pad(KeyPad::Subtract),
            13 => Key::Pad(KeyPad::Divide),
            14 => Key::Pad(KeyPad::Decimal),
            15 => Key::Pad(KeyPad::Enter),
            16 => Key::Pad(KeyPad::Equal),
            17 => Key::Pad(KeyPad::Comma),
            _ => Key::Unknown,
        }
    }
    const fn from_web_compact_mod(value: u32, location: WebKeyLocation) -> Self {
        use WebKeyLocation as L;
        let key = match (value, location) {
            (0, L::Left) => KeyMod::LeftShift, //
            (1, L::Left) => KeyMod::LeftControl,
            (2, L::Left) => KeyMod::LeftAlt,
            (3, L::Left) => KeyMod::LeftSuper,
            (0, L::Right) => KeyMod::RightShift, //
            (1, L::Right) => KeyMod::RightControl,
            (2, L::Right) => KeyMod::RightAlt,
            (3, L::Right) => KeyMod::RightSuper,
            (4, L::Standard) => KeyMod::AltGr, //
            (5, L::Standard) => KeyMod::IsoLevel5Shift,
            _ => return Key::Unknown,
        };
        Key::Mod(key)
    }
    const fn from_web_compact_media(value: u32) -> Self {
        let key = match value {
            0 => KeyMedia::Play, //
            1 => KeyMedia::Pause,
            2 => KeyMedia::PlayPause,
            3 => KeyMedia::Reverse,
            4 => KeyMedia::Stop,
            5 => KeyMedia::FastForward,
            6 => KeyMedia::Rewind,
            7 => KeyMedia::Next,
            8 => KeyMedia::Previous,
            9 => KeyMedia::Record,
            10 => KeyMedia::LowerVolume, //
            11 => KeyMedia::RaiseVolume,
            12 => KeyMedia::MuteVolume,
            20 => KeyMedia::Eject, //
            21 => KeyMedia::MediaSelect,
            22 => KeyMedia::LaunchMedia,
            23 => KeyMedia::BassBoost,
            24 => KeyMedia::BassUp,
            25 => KeyMedia::BassDown,
            26 => KeyMedia::TrebleUp,
            27 => KeyMedia::TrebleDown,
            28 => KeyMedia::MicrophoneMute,
            29 => KeyMedia::MicrophoneVolumeUp,
            30 => KeyMedia::MicrophoneVolumeDown, //
            31 => KeyMedia::BrightnessUp,
            32 => KeyMedia::BrightnessDown,
            33 => KeyMedia::Sleep,
            34 => KeyMedia::Wake,
            35 => KeyMedia::Power,
            _ => return Key::Unknown,
        };
        Key::Media(key)
    }
}
