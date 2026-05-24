// devela::sys::device::audio::common
//
//! Defines [`AudioDevice`], [`AudioDeviceCow`], [`AudioDeviceDir`], [`AudioStreamDir`].
//

#[cfg(feature = "alloc")]
use crate::Cow;
#[cfg(feature = "alsa")]
use crate::{_alsa_raw, CStr};
use crate::{_impl_init, ConstInit, impl_trait};

#[doc = crate::_tags!(audio io)]
/// Borrowed description of an audio endpoint.
#[doc = crate::_doc_location!("media/audio")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AudioDevice<'a> {
    /// Backend device identifier.
    ///
    /// Examples include `default`, `hw:0,0`, PulseAudio sink names,
    /// CPAL device names, or SDL device names.
    pub id: &'a str,
    /// Optional human-readable display name.
    pub name: Option<&'a str>,
    /// Optional human-readable description.
    pub desc: Option<&'a str>,
    /// Endpoint direction capability.
    pub dir: AudioDeviceDir,
}
impl ConstInit for AudioDevice<'_> {
    const INIT: Self = Self {
        id: "",
        name: None,
        desc: None,
        dir: AudioDeviceDir::INIT,
    };
}
impl<'a> AudioDevice<'a> {
    /// Returns the preferred human-facing label for this device.
    pub const fn label(self) -> &'a str {
        match self.name {
            Some(name) => name,
            None => self.id,
        }
    }
    /// Returns the description when available, otherwise the identifier.
    pub const fn desc_or_id(self) -> &'a str {
        match self.desc {
            Some(desc) => desc,
            None => self.id,
        }
    }
}

#[cfg(feature = "alloc")]
#[doc = crate::_tags!(audio io)]
/// Owned or borrowed description of an audio endpoint.
#[doc = crate::_doc_location!("sys/device/audio")]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AudioDeviceCow<'a> {
    /// Backend-specific device identifier.
    pub id: Cow<'a, str>,
    /// Optional human-readable display name.
    pub name: Option<Cow<'a, str>>,
    /// Optional human-readable description.
    pub desc: Option<Cow<'a, str>>,
    /// Supported device direction.
    pub dir: AudioDeviceDir,
}

#[doc = crate::_tags!(audio dir)]
/// Direction capability of an audio endpoint.
#[doc = crate::_doc_location!("media/audio")]
///
/// An endpoint may support output as [`Playback`](Self::Playback),
/// input as [`Capture`](Self::Capture), both directions as
/// [`Duplex`](Self::Duplex), or have an unknown capability.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AudioDeviceDir {
    /// Output-capable endpoint.
    Playback,
    /// Input-capable endpoint.
    Capture,
    /// Input/output-capable endpoint.
    #[default]
    Duplex,
    /// Unknown or backend-specific endpoint direction.
    Unknown,
}
_impl_init![Self::Duplex => AudioDeviceDir];
impl_trait![fmt::Display for AudioDeviceDir |self, f| f.write_str(self.as_code())];
impl AudioDeviceDir {
    /// Returns the short lowercase code.
    pub const fn as_code(self) -> &'static str {
        match self {
            Self::Playback => "playback",
            Self::Capture => "capture",
            Self::Duplex => "duplex",
            Self::Unknown => "unknown",
        }
    }
    /// Returns whether this endpoint can be used for playback.
    pub const fn has_playback(self) -> bool {
        matches!(self, Self::Playback | Self::Duplex)
    }
    /// Returns whether this endpoint can be used for capture.
    pub const fn has_capture(self) -> bool {
        matches!(self, Self::Capture | Self::Duplex)
    }
    /// Returns the unique stream direction, if this endpoint is single-direction.
    pub const fn single_stream_dir(self) -> Option<AudioStreamDir> {
        match self {
            Self::Playback => Some(AudioStreamDir::Playback),
            Self::Capture => Some(AudioStreamDir::Capture),
            Self::Duplex | Self::Unknown => None,
        }
    }
    // Returns the direction from ALSA format.
    #[cfg(feature = "alsa")]
    #[cfg_attr(not(ffi_alsa··), allow(dead_code))]
    pub(crate) const fn from_alsa_ioid(ioid: Option<&CStr>) -> Self {
        match ioid {
            Some(cstr) => match cstr.to_bytes() {
                b"Output" => Self::Playback,
                b"Input" => Self::Capture,
                _ => Self::Unknown,
            },
            None => Self::Duplex,
        }
    }
}

#[doc = crate::_tags!(audio dir)]
/// Direction of an audio stream.
#[doc = crate::_doc_location!("media/audio")]
///
/// A stream can move audio toward an output endpoint as [`Playback`](Self::Playback),
/// or from an input endpoint as [`Capture`](Self::Capture).
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AudioStreamDir {
    /// Output stream direction.
    ///
    /// Audio flows from the program to an output device.
    #[default]
    Playback,

    /// Input stream direction.
    ///
    /// Audio flows from an input device to the program.
    Capture,
}
_impl_init![Self::Playback => AudioStreamDir];
impl_trait![fmt::Display for AudioStreamDir |self, f| f.write_str(self.as_code())];

impl AudioStreamDir {
    /// Returns the short lowercase code.
    pub const fn as_code(self) -> &'static str {
        match self {
            Self::Playback => "playback",
            Self::Capture => "capture",
        }
    }
    /// Returns a human-readable name.
    pub const fn as_name(self) -> &'static str {
        match self {
            Self::Playback => "Playback",
            Self::Capture => "Capture",
        }
    }
    /// Returns whether this is [`Playback`](Self::Playback).
    pub const fn is_playback(self) -> bool {
        matches!(self, Self::Playback)
    }
    /// Returns whether this is [`Capture`](Self::Capture).
    pub const fn is_capture(self) -> bool {
        matches!(self, Self::Capture)
    }
    // Converts the direction to ALSA format.
    #[cfg(feature = "alsa")]
    #[cfg_attr(not(ffi_alsa··), allow(dead_code))]
    pub(crate) const fn to_alsa(self) -> _alsa_raw::snd_pcm_stream_t {
        match self {
            Self::Playback => _alsa_raw::SND_PCM_STREAM_PLAYBACK,
            Self::Capture => _alsa_raw::SND_PCM_STREAM_CAPTURE,
        }
    }
}
