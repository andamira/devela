// devela::media::audio::pcm::io
//
//! Defines [`PcmStream`], [`PcmDrain`],
//! [`PcmSink`], [`PcmSinkPlanar`], [`PcmSource`], [`PcmSourcePlanar`].
//

use crate::{PcmBuf, PcmLayout, PcmSampleType, PcmSpec};

#[doc = crate::_tags!(audio)]
/// A configured PCM stream.
#[doc = crate::_doc_location!("media/audio/pcm")]
/// This is the shared base for PCM input and output streams. It exposes the
/// active sample specification and memory layout negotiated with the backend.
pub trait PcmStream {
    /// Error returned by the stream.
    type Error;

    /// Returns the configured PCM spec.
    fn pcm_spec(&self) -> Option<PcmSpec>;
    /// Returns the configured PCM layout.
    fn pcm_layout(&self) -> Option<PcmLayout>;
}

/// A PCM stream that can wait for pending playback frames to finish.
///
/// Draining is mainly meaningful for playback sinks. It lets the backend finish
/// consuming frames that have already been written before the stream is closed,
/// reused, or the program exits.
///
/// This is similar in spirit to flushing buffered output, but stronger: it waits
/// until queued audio has actually been played or consumed by the backend.
pub trait PcmDrain: PcmStream {
    /// Waits until pending playback frames are consumed.
    fn drain(&mut self) -> Result<(), Self::Error>;
}

#[doc = crate::_tags!(audio)]
/// A stream that writes interleaved PCM frames.
#[doc = crate::_doc_location!("media/audio/pcm")]
///
/// A sink consumes audio. In practice, this is the trait used for playback,
/// output, speakers, audio devices, files, or mixers that receive PCM frames.
pub trait PcmSink<T: PcmSampleType>: PcmStream {
    /// Writes interleaved frames. Returns the number of frames written.
    fn write(&mut self, pcm: PcmBuf<T, &[T]>) -> Result<usize, Self::Error>;
    /// Writes all requested interleaved frames.
    fn write_all(&mut self, pcm: PcmBuf<T, &[T]>) -> Result<(), Self::Error>;

    /// Writes all requested interleaved frames and drains pending playback.
    fn write_all_drain(&mut self, pcm: PcmBuf<T, &[T]>) -> Result<(), Self::Error>
    where
        Self: PcmDrain,
    {
        self.write_all(pcm)?;
        self.drain()
    }
}
#[doc = crate::_tags!(audio)]
/// A stream that writes planar PCM frames.
#[doc = crate::_doc_location!("media/audio/pcm")]
///
/// This is the planar counterpart to [`PcmSink`].
/// It consumes one sample plane per channel instead of one interleaved sample slice.
pub trait PcmSinkPlanar<T: PcmSampleType>: PcmStream {
    /// Writes planar frames. Returns the number of frames written.
    fn write_planar(&mut self, pcm: PcmBuf<T, &[&[T]]>) -> Result<usize, Self::Error>;
    /// Writes all requested planar frames.
    fn write_all_planar(&mut self, pcm: PcmBuf<T, &[&[T]]>) -> Result<(), Self::Error>;

    /// Writes all requested planar frames and drains pending playback.
    fn write_all_planar_drain(&mut self, pcm: PcmBuf<T, &[&[T]]>) -> Result<(), Self::Error>
    where
        Self: PcmDrain,
    {
        self.write_all_planar(pcm)?;
        self.drain()
    }
}

#[doc = crate::_tags!(audio)]
/// A stream that reads interleaved PCM frames.
#[doc = crate::_doc_location!("media/audio/pcm")]
///
/// A source produces audio. In practice, this is the trait used for recording,
/// input, microphones, audio devices, files, or generators that provide PCM frames.
pub trait PcmSource<T: PcmSampleType>: PcmStream {
    /// Reads interleaved frames. Returns the number of frames read.
    fn read(&mut self, pcm: PcmBuf<T, &mut [T]>) -> Result<usize, Self::Error>;
    /// Reads all requested interleaved frames.
    fn read_all(&mut self, pcm: PcmBuf<T, &mut [T]>) -> Result<(), Self::Error>;
}
#[doc = crate::_tags!(audio)]
/// A stream that reads planar PCM frames.
#[doc = crate::_doc_location!("media/audio/pcm")]
///
/// This is the planar counterpart to [`PcmSource`].
/// It fills one sample plane per channel instead of one interleaved sample slice.
pub trait PcmSourcePlanar<T: PcmSampleType>: PcmStream {
    /// Reads planar frames. Returns the number of frames read.
    fn read_planar(&mut self, pcm: PcmBuf<T, &mut [&mut [T]]>) -> Result<usize, Self::Error>;
    /// Reads all requested planar frames.
    fn read_all_planar(&mut self, pcm: PcmBuf<T, &mut [&mut [T]]>) -> Result<(), Self::Error>;
}
