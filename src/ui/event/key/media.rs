// devela::ui::event::key::media
//
//! Defines [`KeyMedia`].
//

use crate::ConstInit;

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
