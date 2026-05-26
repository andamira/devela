// devela::sys::device::audio::alsa::error
//
//! Defines [`AlsaError`].
//

#[cfg(feature = "alloc")]
use crate::CStr;
use crate::{_alsa_raw as _raw, c_int, impl_trait};
use crate::{PcmLayout, PcmSample, PcmSpec};

crate::test_size_of![AlsaError = 24]; // 192 bits

#[doc = crate::_tags!(audio linux error)]
/// ALSA PCM error.
#[doc = crate::_doc_location!("sys/device/audio")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum AlsaError {
    /// ALSA returned a negative error code.
    Code(c_int),
    /// The PCM stream has not been configured.
    Unconfigured,
    /// The PCM stream layout has not been configured.
    LayoutUnconfigured,
    /// The requested layout does not match the configured stream layout.
    LayoutMismatch { active: PcmLayout, expected: PcmLayout },
    /// The Rust sample type does not match the PCM sample encoding.
    SampleMismatch { pcm: PcmSample, ty: PcmSample },
    /// The buffer spec does not match the configured stream spec.
    SpecMismatch { buffer: PcmSpec, stream: PcmSpec },
    /// The requested PCM spec is invalid.
    InvalidSpec(PcmSpec),
    /// The PCM buffer does not contain complete frames.
    IncompleteFrames,
    /// A read/write operation made no progress.
    NoProgress,
    /// ALSA reported more frames than were requested.
    FrameCountExceeded { read_or_written: usize, requested: usize },
    /// Too many channels for the fixed channel-pointer buffer.
    TooManyChannels { count: usize, max: usize },
}
impl AlsaError {
    /// Creates an ALSA error from a raw negative error code.
    #[must_use]
    pub const fn code(code: c_int) -> Self {
        Self::Code(code)
    }
    /// Returns `Ok(())` for non-negative ALSA codes.
    pub const fn result(code: c_int) -> Result<(), Self> {
        if code < 0 { Err(Self::Code(code)) } else { Ok(()) }
    }
    /// The stream is not configured.
    #[must_use]
    pub const fn unconfigured() -> Self {
        Self::Unconfigured
    }
    /// The stream layout is not configured.
    #[must_use]
    pub const fn layout_unconfigured() -> Self {
        Self::LayoutUnconfigured
    }
    /// The requested layout does not match the configured layout.
    #[must_use]
    pub const fn layout_mismatch(active: PcmLayout, expected: PcmLayout) -> Self {
        Self::LayoutMismatch { active, expected }
    }
    /// The Rust sample type does not match the PCM sample encoding.
    #[must_use]
    pub const fn sample_mismatch(pcm: PcmSample, ty: PcmSample) -> Self {
        Self::SampleMismatch { pcm, ty }
    }
    /// The buffer spec does not match the configured stream spec.
    #[must_use]
    pub const fn spec_mismatch(buffer: PcmSpec, stream: PcmSpec) -> Self {
        Self::SpecMismatch { buffer, stream }
    }
    /// The requested PCM spec is invalid.
    #[must_use]
    pub const fn invalid_spec(spec: PcmSpec) -> Self {
        Self::InvalidSpec(spec)
    }
    /// The PCM buffer does not contain complete frames.
    #[must_use]
    pub const fn incomplete_frames() -> Self {
        Self::IncompleteFrames
    }
    /// A read/write operation made no progress.
    #[must_use]
    pub const fn no_progress() -> Self {
        Self::NoProgress
    }
    /// ALSA reported more frames than were requested.
    #[must_use]
    pub const fn frame_count_exceeded(read_or_written: usize, requested: usize) -> Self {
        Self::FrameCountExceeded { read_or_written, requested }
    }
    /// Too many channels for the fixed channel-pointer buffer.
    #[must_use]
    pub const fn too_many_channels(count: usize, max: usize) -> Self {
        Self::TooManyChannels { count, max }
    }
}
impl_trait![fmt::Display+Error for AlsaError |self, f| match *self {
    Self::Code(code) => cfg_select! {
        ffi_alsa·· => { unsafe {
            let msg = _raw::snd_strerror(code);
            if msg.is_null() {
                write![f, "ALSA error {code}"]
            } else if let Ok(msg) = CStr::from_ptr(msg).to_str() {
                write![f, "ALSA error {code}: {msg}"]
            } else {
                write![f, "ALSA error {code}"]
            }
        }}
        _ => write![f, "ALSA error {code}"],
    },
    Self::Unconfigured =>
        f.write_str("ALSA PCM stream is not configured"),
    Self::LayoutUnconfigured =>
        f.write_str("ALSA PCM stream layout is not configured"),
    Self::LayoutMismatch { active, expected } =>
        write![f, "ALSA PCM layout mismatch: active {active}, expected {expected}"],
    Self::SampleMismatch { pcm, ty } =>
        write![f, "PCM sample mismatch: buffer uses {pcm}, Rust type implies {ty}"],
    Self::SpecMismatch { buffer, stream } =>
        write![f, "PCM spec mismatch: buffer {buffer}, stream {stream}"],
    Self::InvalidSpec(spec) =>
        write![f, "Invalid PCM spec: {spec}"],
    Self::IncompleteFrames =>
        f.write_str("PCM buffer does not contain complete frames"),
    Self::NoProgress =>
        f.write_str("ALSA PCM operation made no progress"),
    Self::FrameCountExceeded { read_or_written, requested } =>
        write![f, "ALSA PCM operation exceeded requested frames: got {read_or_written}, requested {requested}"],
    Self::TooManyChannels { count, max } =>
        write![f, "Too many PCM channels: got {count}, maximum supported here is {max}"],
}];
