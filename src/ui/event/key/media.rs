// devela::ui::event::key::media
//
//! Defines [`KeyMedia`].
//
// TOC
// - KeyMedia
// - impls

use crate::ConstInit;

/* definitions */

#[doc = crate::_tags!(interaction)]
/// Media key codes.
#[doc = crate::_doc_location!("ui/event")]
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
impl ConstInit for KeyMedia {
    const INIT: Self = Self::Play;
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
