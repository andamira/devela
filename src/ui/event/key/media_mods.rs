// devela::ui::event::key::media_mods
//
//! Defines [`KeyMedia`], [`KeyMod`], [`KeyMods`].
//
// TOC
// - definitions
//   - KeyMedia
//   - KeyMod
//   - KeyMods
// - impls
//   - js

#[cfg(feature = "js")]
use crate::JsKeyLocation;

/* definitions */

/// Media key codes.
///
/// These keys are commonly found on multimedia keyboards and remote controls.
//
// - https://docs.rs/crossterm/latest/crossterm/event/enum.MediaKey.html
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum KeyMedia {
    /// Starts media playback.
    Play,
    /// Pauses media playback.
    Pause,
    /// Toggles between playing and pausing media.
    PlayPause,
    /// Plays media in reverse.
    Reverse,
    /// Stops media playback.
    Stop,
    /// Fast-forwards through media.
    FastForward,
    /// Rewinds through media.
    Rewind,
    /// Skips to the next media track.
    Next,
    /// Returns to the previous media track.
    Previous,
    /// Starts recording media.
    Record,
    /// Decreases the audio volume.
    LowerVolume,
    /// Increases the audio volume.
    RaiseVolume,
    /// Mutes or unmutes the audio.
    MuteVolume,

    /* Uncommon multimedia keys */
    /// Ejects a removable media disk (found on some keyboards, especially Macs).
    Eject,
    /// Opens the default media player application.
    MediaSelect,
    /// Launches a media-related application.
    LaunchMedia,

    /// Toggles bass boost mode.
    BassBoost,
    /// Increases bass levels.
    BassUp,
    /// Decreases bass levels.
    BassDown,
    /// Increases treble levels.
    TrebleUp,
    /// Decreases treble levels.
    TrebleDown,

    /// Mutes or unmutes the microphone.
    MicrophoneMute,
    /// Increases the microphone input volume.
    MicrophoneVolumeUp,
    /// Decreases the microphone input volume.
    MicrophoneVolumeDown,

    /// Increases the screen brightness.
    BrightnessUp,
    /// Decreases the screen brightness.
    BrightnessDown,

    /// Puts the system into sleep mode.
    Sleep,
    /// Wakes the system from sleep mode.
    Wake,
    /// Powers the system off or performs a power-related action.
    Power,
}

/// Modifier key codes (when pressed by themselves)
///
/// These keys modify the behavior of other keys when held down.
//
// - https://docs.rs/crossterm/latest/crossterm/event/enum.ModifierKey.html
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum KeyMod {
    /// Left **Shift** key.
    LeftShift,
    /// Left **Control** (Ctrl) key.
    LeftControl,
    /// Left **Alt** key.
    LeftAlt,
    /// Left **Super** key (Windows key on Windows, Command ⌘ on macOS).
    LeftSuper,
    /// Left **Hyper** key (historically used in some Unix systems).
    LeftHyper,
    /// Left **Meta** key (used in some Unix-based systems, overlaps with Super).
    LeftMeta,

    /// Right **Shift** key.
    RightShift,
    /// Right **Control** (Ctrl) key.
    RightControl,
    /// Right **Alt** key.
    RightAlt,
    /// Right **Super** key (Windows key on Windows, Command ⌘ on macOS).
    RightSuper,
    /// Right **Hyper** key (historically used in some Unix systems).
    RightHyper,
    /// Right **Meta** key (used in some Unix-based systems, overlaps with Super).
    RightMeta,

    /// **ISO Level 3 Shift** key (commonly known as **AltGr**).
    ///
    /// Used to access alternative characters on some keyboards.
    IsoLevel3Shift,
    /// **ISO Level 5 Shift** key (used in some advanced keyboard layouts).
    IsoLevel5Shift,
}
#[allow(non_upper_case_globals)]
impl KeyMod {
    /// AltGr key.
    pub const AltGr: KeyMod = KeyMod::IsoLevel3Shift;
}

/// A bitfield of keys modifiers (Shift, Ctrl…) + extra (repeating, composing).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyMods(u16);
#[rustfmt::skip]
impl KeyMods {
    const CTRL: u16 = 1 << 0;
    const SHIFT: u16 = 1 << 1;
    const ALT: u16 = 1 << 2;
    const META: u16 = 1 << 3;
    const ALT_GRAPH: u16 = 1 << 4;
    const CAPS_LOCK: u16 = 1 << 5;
    const NUM_LOCK: u16 = 1 << 6;
    const SCROLL_LOCK: u16 = 1 << 7;
    const REPEAT: u16 = 1 << 8;
    const IS_COMPOSING: u16 = 1 << 9;

    /// Constructs a `KeyMods` from a bitfield representation.
    pub const fn from_bits(bits: u16) -> Self { Self(bits) }

    // /// Checks if a given modifier is active.
    // pub const fn has(self, mod_mask: u16) -> bool { self.0 & mod_mask != 0 }
    /// Checks if the *Control* modifier is set.
    pub const fn has_ctrl(self) -> bool { self.0 & Self::CTRL != 0 }
    /// Checks if the *Shift* modifier is set.
    pub const fn has_shift(self) -> bool { self.0 & Self::SHIFT != 0 }
    /// Checks if the *Alt* modifier is set.
    pub const fn has_alt(self) -> bool { self.0 & Self::ALT != 0 }
    /// Checks if the *AltGraph* modifier is set.
    pub const fn has_alt_gr(self) -> bool { self.0 & Self::ALT_GRAPH != 0 }
    /// Checks if the *Meta* modifier is set.
    pub const fn has_meta(self) -> bool { self.0 & Self::META != 0 }
    /// Checks if the *Caps Lock* modifier is set.
    pub const fn has_caps_lock(self) -> bool { self.0 & Self::CAPS_LOCK != 0 }
    /// Checks if the *Num Lock* modifier is set.
    pub const fn has_num_lock(self) -> bool { self.0 & Self::NUM_LOCK != 0 }
    /// Checks if the *Scroll Lock* modifier is set.
    pub const fn has_scroll_lock(self) -> bool { self.0 & Self::SCROLL_LOCK != 0 }

    /// Queries if a key event is a repeat.
    pub const fn is_repeating(self) -> bool { self.0 & Self::REPEAT != 0 }
    /// Queries if a key event is composing (IME input).
    pub const fn is_composing(self) -> bool { self.0 & Self::IS_COMPOSING != 0 }
}

/* impls */

#[rustfmt::skip]
#[cfg(feature = "js")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "js")))]
impl KeyMedia {
    /// Atempts to construct a `KeyMedia` from a JavaScript `KeyboardEvent` physical [code].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    pub const fn from_js_code(code: &str) -> Option<Self> {
        use KeyMedia as K;
        match code.as_bytes() {
            b"MediaPlay" => Some(K::Play),
            b"MediaPause" => Some(K::Pause),
            b"MediaPlayPause" => Some(K::PlayPause),
            b"MediaReverse" => Some(K::Reverse),
            b"MediaStop" => Some(K::Stop),
            b"MediaFastForward" => Some(K::FastForward),
            b"MediaRewind" => Some(K::Rewind),
            b"MediaTrackNext" => Some(K::Next),
            b"MediaTrackPrevious" => Some(K::Previous),
            b"MediaRecord" => Some(K::Record),
            b"AudioVolumeDown" => Some(K::LowerVolume),
            b"AudioVolumeUp" => Some(K::RaiseVolume),
            b"AudioVolumeMute" => Some(K::MuteVolume),
            //
            b"Eject" => Some(K::Eject),
            b"MediaSelect" => Some(K::MediaSelect),
            b"LaunchMedia" => Some(K::LaunchMedia),
            b"BassBoost" => Some(K::BassBoost),
            b"BassUp" => Some(K::BassUp),
            b"BassDown" => Some(K::BassDown),
            b"TrebleUp" => Some(K::TrebleUp),
            b"TrebleDown" => Some(K::TrebleDown),
            b"MicrophoneMute" => Some(K::MicrophoneMute),
            b"MicrophoneVolumeUp" => Some(K::MicrophoneVolumeUp),
            b"MicrophoneVolumeDown" => Some(K::MicrophoneVolumeDown),
            b"BrightnessUp" => Some(K::BrightnessUp),
            b"BrightnessDown" => Some(K::BrightnessDown),
            b"Sleep" => Some(K::Sleep),
            b"Wake" => Some(K::Wake),
            b"Power" => Some(K::Power),
            _ => None,
        }
    }
    /// Returns a JavaScript `KeyboardEvent` physical [code].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    pub const fn to_js_code(self) -> &'static str {
        use KeyMedia as K;
        match self {
            K::Play => "MediaPlay",
            K::Pause => "MediaPause",
            K::PlayPause => "MediaPlayPause",
            K::Reverse => "MediaReverse",
            K::Stop => "MediaStop",
            K::FastForward => "MediaFastForward",
            K::Rewind => "MediaRewind",
            K::Next => "MediaTrackNext",
            K::Previous => "MediaTrackPrevious",
            K::Record => "MediaRecord",
            K::LowerVolume => "AudioVolumeDown",
            K::RaiseVolume => "AudioVolumeUp",
            K::MuteVolume => "AudioVolumeMute",
            //
            K::Eject => "Eject",
            K::MediaSelect => "MediaSelect",
            K::LaunchMedia => "LaunchMedia",
            K::BassBoost => "BassBoost",
            K::BassUp => "BassUp",
            K::BassDown => "BassDown",
            K::TrebleUp => "TrebleUp",
            K::TrebleDown => "TrebleDown",
            K::MicrophoneMute => "MicrophoneMute",
            K::MicrophoneVolumeUp => "MicrophoneVolumeUp",
            K::MicrophoneVolumeDown => "MicrophoneVolumeDown",
            K::BrightnessUp => "BrightnessUp",
            K::BrightnessDown => "BrightnessDown",
            K::Sleep => "Sleep",
            K::Wake => "Wake",
            K::Power => "Power",
        }
    }

    /// Atempts to construct a `KeyMedia` from a JavaScript `KeyboardEvent` semantic [key].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    pub const fn from_js_key(key: &str) -> Option<Self> {
        use KeyMedia as K;
        match key.as_bytes() {
            b"Play" => Some(K::Play),
            b"Pause" => Some(K::Pause),
            b"MediaPlayPause" => Some(K::PlayPause),
            b"MediaReverse" => Some(K::Reverse),
            b"Stop" => Some(K::Stop),
            b"FastForward" => Some(K::FastForward),
            b"Rewind" => Some(K::Rewind),
            b"NextTrack" => Some(K::Next),
            b"PreviousTrack" => Some(K::Previous),
            b"Record" => Some(K::Record),
            b"VolumeDown" => Some(K::LowerVolume),
            b"VolumeUp" => Some(K::RaiseVolume),
            b"VolumeMute" => Some(K::MuteVolume),
            //
            b"Eject" => Some(K::Eject),
            b"MediaSelect" => Some(K::MediaSelect),
            b"LaunchMedia" => Some(K::LaunchMedia),
            b"Bass Boost" => Some(K::BassBoost),
            b"Bass Up" => Some(K::BassUp),
            b"Bass Down" => Some(K::BassDown),
            b"Treble Up" => Some(K::TrebleUp),
            b"Treble Down" => Some(K::TrebleDown),
            b"MicMute" => Some(K::MicrophoneMute),
            b"MicVolumeUp" => Some(K::MicrophoneVolumeUp),
            b"MicVolumeDown" => Some(K::MicrophoneVolumeDown),
            b"BrightnessUp" => Some(K::BrightnessUp),
            b"BrightnessDown" => Some(K::BrightnessDown),
            b"Sleep" => Some(K::Sleep),
            b"Wake" => Some(K::Wake),
            b"Power" => Some(K::Power),
            _ => None,
        }
    }
    /// Returns a JavaScript `KeyboardEvent` semantic [key].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    pub const fn to_js_key(self) -> &'static str {
        use KeyMedia as K;
        match self {
            K::Play => "Play",
            K::Pause => "Pause",
            K::PlayPause => "MediaPlayPause",
            K::Reverse => "MediaReverse",
            K::Stop => "Stop",
            K::FastForward => "FastForward",
            K::Rewind => "Rewind",
            K::Next => "NextTrack",
            K::Previous => "PreviousTrack",
            K::Record => "Record",
            K::LowerVolume => "VolumeDown",
            K::RaiseVolume => "VolumeUp",
            K::MuteVolume => "VolumeMute",
            //
            K::Eject => "Eject",
            K::MediaSelect => "MediaSelect",
            K::LaunchMedia => "LaunchMedia",
            K::BassBoost => "Bass Boost",
            K::BassUp => "Bass Up",
            K::BassDown => "Bass Down",
            K::TrebleUp => "Treble Up",
            K::TrebleDown => "Treble Down",
            K::MicrophoneMute => "MicMute",
            K::MicrophoneVolumeUp => "MicVolumeUp",
            K::MicrophoneVolumeDown => "MicVolumeDown",
            K::BrightnessUp => "BrightnessUp",
            K::BrightnessDown => "BrightnessDown",
            K::Sleep => "Sleep",
            K::Wake => "Wake",
            K::Power => "Power",
        }
    }
}

#[rustfmt::skip]
#[cfg(feature = "js")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "js")))]
impl KeyMod {
    /// Atempts to construct a `KeyMod` from a JavaScript `KeyboardEvent`
    /// physical [code] and [location].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    /// [location]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
    pub const fn from_js_code(code: &str, location: JsKeyLocation) -> Option<Self> {
        use {KeyMod as K, JsKeyLocation as L};
        match (code.as_bytes(), location) {
            (b"ShiftLeft", L::Left) => Some(K::LeftShift),
            (b"ControlLeft", L::Left) => Some(K::LeftControl),
            (b"AltLeft", L::Left) => Some(K::LeftAlt),
            (b"MetaLeft", L::Left) => Some(K::LeftSuper),
            (b"HyperLeft", L::Left) => Some(K::LeftHyper),
            (b"OSLeft", L::Left) => Some(K::LeftMeta),
            (b"ShiftRight", L::Right) => Some(K::RightShift),
            (b"ControlRight", L::Right) => Some(K::RightControl),
            (b"AltRight", L::Right) => Some(K::RightAlt),
            (b"MetaRight", L::Right) => Some(K::RightSuper),
            (b"HyperRight", L::Right) => Some(K::RightHyper),
            (b"OSRight", L::Right) => Some(K::RightMeta),
            (b"AltGraph", L::Standard) => Some(K::IsoLevel3Shift),
            (b"Level5Shift", L::Standard) => Some(K::IsoLevel5Shift),
            _ => None,
        }
    }
    /// Returns a JavaScript `KeyboardEvent` physical [code] and [location].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    /// [location]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
    pub const fn to_js_code(self) -> (&'static str, JsKeyLocation) {
        use {KeyMod as K, JsKeyLocation as L};
        match self {
            K::LeftShift => ("ShiftLeft", L::Left),
            K::LeftControl => ("ControlLeft", L::Left),
            K::LeftAlt => ("AltLeft", L::Left),
            K::LeftSuper => ("MetaLeft", L::Left),
            K::LeftHyper => ("HyperLeft", L::Left),
            K::LeftMeta => ("OSLeft", L::Left),
            K::RightShift => ("ShiftRight", L::Right),
            K::RightControl => ("ControlRight", L::Right),
            K::RightAlt => ("AltRight", L::Right),
            K::RightSuper => ("MetaRight", L::Right),
            K::RightHyper => ("HyperRight", L::Right),
            K::RightMeta => ("OSRight", L::Right),
            K::IsoLevel3Shift => ("AltGraph", L::Standard),
            K::IsoLevel5Shift => ("Level5Shift", L::Standard),
        }
    }
    /// Atempts to construct a `KeyMod` from a JavaScript `KeyboardEvent`
    /// semantic [key] and [location].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    /// [location]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
    pub const fn from_js_key(key: &str, location: JsKeyLocation) -> Option<Self> {
        use {KeyMod as K, JsKeyLocation as L};
        match (key.as_bytes(), location) {
            (b"Shift", L::Left) => Some(K::LeftShift),
            (b"Control", L::Left) => Some(K::LeftControl),
            (b"Alt", L::Left) => Some(K::LeftAlt),
            (b"Meta", L::Left) => Some(K::LeftSuper),
            (b"Hyper", L::Left) => Some(K::LeftHyper),
            (b"OS", L::Left) => Some(K::LeftMeta),
            (b"Shift", L::Right) => Some(K::RightShift),
            (b"Control", L::Right) => Some(K::RightControl),
            (b"Alt", L::Right) => Some(K::RightAlt),
            (b"Meta", L::Right) => Some(K::RightSuper),
            (b"Hyper", L::Right) => Some(K::RightHyper),
            (b"OS", L::Right) => Some(K::RightMeta),
            (b"AltGraph", L::Standard) => Some(K::IsoLevel3Shift),
            (b"Level5Shift", L::Standard) => Some(K::IsoLevel5Shift),
            _ => None,
        }
    }
    /// Returns a JavaScript `KeyboardEvent` semantic [key] and [location].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    /// [location]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
    pub const fn to_js_key(self) -> (&'static str, JsKeyLocation) {
        use {KeyMod as K, JsKeyLocation as L};
        match self {
            K::LeftShift => ("Shift", L::Left),
            K::LeftControl => ("Control", L::Left),
            K::LeftAlt => ("Alt", L::Left),
            K::LeftSuper => ("Meta", L::Left),
            K::LeftHyper => ("Hyper", L::Left),
            K::LeftMeta => ("OS", L::Left),
            K::RightShift => ("Shift", L::Right),
            K::RightControl => ("Control", L::Right),
            K::RightAlt => ("Alt", L::Right),
            K::RightSuper => ("Meta", L::Right),
            K::RightHyper => ("Hyper", L::Right),
            K::RightMeta => ("OS", L::Right),
            K::IsoLevel3Shift => ("AltGraph", L::Standard),
            K::IsoLevel5Shift => ("Level5Shift", L::Standard),
        }
    }
}
