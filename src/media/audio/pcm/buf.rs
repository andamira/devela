// devela::media::audio::pcm::buf
//
//! Defines [`PcmBuf`].
//
// TOC
// - struct PcmBuf
// - impl ConstInit
// - impl generic metadata
// - impl shared interleaved
// - impl exclusive interleaved
// - impl shared planar
// - impl exclusive planar

use crate::{AudioChannels, ConstInit, PcmSample, PcmSpec};
use crate::{PhantomData, is, test_size_of, whilst};

test_size_of![test_size_of_PcmBuf_i8: PcmBuf<i8, &[i8]> = 24]; // 192 bits
test_size_of![test_size_of_PcmBuf_f64: PcmBuf<f64, &[f64]> = 24]; // 192 bits
test_size_of![test_size_of_PcmBuf_planar_i32: PcmBuf<i32, &[&[i32]]> = 24]; // 192 bits

#[doc = crate::_tags!(audio data)]
/// Typed PCM sample buffer over caller-chosen storage.
#[doc = crate::_doc_meta!{location("media/audio")}]
///
/// `PcmBuf` pairs sample storage with the [`PcmSpec`] parts needed to interpret
/// it as PCM audio: sample encoding, channel layout, and sample rate.
///
/// The storage type determines both ownership and layout:
/// - `PcmBuf<T, &[T]>` borrows interleaved samples, e.g. `LRLRLR…`.
/// - `PcmBuf<T, &mut [T]>` mutably borrows interleaved samples.
/// - `PcmBuf<T, &[&[T]]>` borrows planar samples, e.g. `[LLLL…], [RRRR…]`.
/// - `PcmBuf<T, &mut [&mut [T]]>` mutably borrows planar samples.
///
/// Interleaved buffers count frames by dividing the sample length by the channel
/// count. Planar buffers count frames by checking that the number of planes
/// matches the channel count and that all planes have the same length.
///
/// # Examples
///
/// Borrowed interleaved stereo samples:
/// ```
/// # use devela::{AudioChannels, PcmBuf, PcmSample};
/// let pcm = PcmBuf::from_parts(
///     &[0i16, 1, 2, 3][..],
///     PcmSample::I16,
///     AudioChannels::Stereo,
///     44_100,
/// );
/// assert_eq!(pcm.samples_len(), 4);
/// assert_eq!(pcm.frames(), Some(2));
/// ```
///
/// Borrowed planar stereo samples:
/// ```
/// # use devela::{AudioChannels, PcmBuf, PcmSample, PcmSpec};
/// let left = [0i16, 2];
/// let right = [1i16, 3];
/// let planes: &[&[i16]] = &[&left, &right];
/// let spec = PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, 44_100);
/// let pcm = PcmBuf::new(planes, spec);
/// assert_eq!(pcm.planes_len(), 2);
/// assert_eq!(pcm.frames(), Some(2));
/// ```
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PcmBuf<T, B> {
    /// Sample storage, whose type determines layout and mutability.
    pub data: B,
    /// PCM metadata.
    pub spec: PcmSpec,
    _sample: PhantomData<fn() -> T>,
}

impl<T> ConstInit for PcmBuf<T, &[T]> {
    const INIT: Self = Self::new(&[], PcmSpec::INIT);
}
impl<T> ConstInit for PcmBuf<T, &mut [T]> {
    const INIT: Self = Self::new(&mut [], PcmSpec::INIT);
}
impl<T> ConstInit for PcmBuf<T, &[&[T]]> {
    const INIT: Self = Self::new(&[], PcmSpec::INIT);
}
impl<T> ConstInit for PcmBuf<T, &mut [&mut [T]]> {
    const INIT: Self = Self::new(&mut [], PcmSpec::INIT);
}

/// # Generic storage
/// Common PCM metadata methods independent of the sample storage shape.
#[rustfmt::skip]
impl<T, B> PcmBuf<T, B> {
    /// Creates a PCM buffer over the given storage and stream specification.
    pub const fn new(data: B, spec: PcmSpec) -> Self { Self { data, spec, _sample: PhantomData } }
    /// Creates a PCM buffer from storage and separate stream metadata parts.
    pub const fn from_parts(data: B, format: PcmSample, channels: AudioChannels, sample_rate: u32)
        -> Self { Self::new(data, PcmSpec::new(format, channels, sample_rate)) }

    /// Returns the sample encoding.
    pub const fn sample(&self) -> PcmSample { self.spec.sample }
    /// Returns the channel layout.
    #[must_use]
    pub const fn channels(&self) -> AudioChannels { self.spec.channels }
    /// Returns the number of channels.
    #[must_use]
    pub const fn channel_count(&self) -> usize { self.spec.channel_count() }
    /// Returns the sample rate in Hertz.
    #[must_use]
    pub const fn sample_rate(&self) -> u32 { self.spec.sample_rate }
}

/// # Shared interleaved slices
/// Methods for borrowed interleaved PCM samples.
impl<'a, T> PcmBuf<T, &'a [T]> {
    /* constructors*/
    /// Creates a PCM buffer from borrowed interleaved samples.
    pub const fn from_interleaved(data: &'a [T], spec: PcmSpec) -> Self {
        Self::new(data, spec)
    }
    /* direct accessors*/
    /// Returns the sample data.
    pub const fn data(&self) -> &'a [T] {
        self.data
    }
    /* shape queries */
    /// Returns whether there are no samples.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Returns the number of sample values.
    #[must_use]
    pub const fn samples_len(&self) -> usize {
        self.data.len()
    }
    /// Returns the frame count, or `None` if the sample count is incomplete.
    #[must_use]
    pub const fn frames(&self) -> Option<usize> {
        let len = self.samples_len();
        let channels = self.channel_count();
        if channels == 0 || len % channels != 0 { None } else { Some(len / channels) }
    }
    /// Returns whether the sample data contains complete frames.
    #[must_use]
    pub const fn has_complete_frames(&self) -> bool {
        self.frames().is_some()
    }
    /* visitors */
    /// Visits the interleaved sample slice.
    pub fn visit_samples<F, R>(&self, f: F) -> R
    where
        for<'v> F: FnOnce(&'v [T]) -> R,
    {
        f(self.data)
    }
    /// Visits each sample, in storage order.
    pub fn visit_each<F>(&self, f: F)
    where
        for<'v> F: Fn(&'v T),
    {
        for sample in self.data.iter() {
            f(sample);
        }
    }
    /// Visits each complete interleaved frame.
    pub fn visit_each_frame<F>(&self, f: F)
    where
        for<'v> F: Fn(usize, &'v [T]),
    {
        let channels = self.channel_count();
        is! { channels == 0, return }
        for (frame, samples) in self.data.chunks_exact(channels).enumerate() {
            f(frame, samples);
        }
    }
}

/// # Exclusive interleaved slices
/// Methods for mutably borrowed interleaved PCM samples.
impl<'a, T> PcmBuf<T, &'a mut [T]> {
    /// Creates a PCM buffer from mutably borrowed interleaved samples.
    pub const fn from_interleaved_mut(data: &'a mut [T], spec: PcmSpec) -> Self {
        Self::new(data, spec)
    }
    /* direct accessors*/
    /// Returns the sample data.
    pub const fn data(&self) -> &[T] {
        &*self.data
    }
    /// Returns the mutable sample data.
    pub const fn data_mut(&mut self) -> &mut [T] {
        self.data
    }
    /* shape queries */
    /// Returns whether there are no samples.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Returns the number of sample values.
    #[must_use]
    pub const fn samples_len(&self) -> usize {
        self.data.len()
    }
    /// Returns the frame count, or `None` if the sample count is incomplete.
    #[must_use]
    pub const fn frames(&self) -> Option<usize> {
        let len = self.samples_len();
        let channels = self.channel_count();
        if channels == 0 || len % channels != 0 { None } else { Some(len / channels) }
    }
    /// Returns whether the sample data contains complete frames.
    #[must_use]
    pub const fn has_complete_frames(&self) -> bool {
        self.frames().is_some()
    }
    /* visitors */
    /// Visits the interleaved sample slice mutably.
    pub fn visit_samples_mut<F, R>(&mut self, f: F) -> R
    where
        for<'v> F: FnOnce(&'v mut [T]) -> R,
    {
        f(self.data)
    }
    /// Visits each sample mutably, in storage order.
    pub fn visit_each_mut<F>(&mut self, f: F)
    where
        for<'v> F: Fn(&'v mut T),
    {
        for sample in self.data.iter_mut() {
            f(sample);
        }
    }
    /// Visits each complete interleaved frame mutably.
    pub fn visit_each_frame_mut<F>(&mut self, f: F)
    where
        for<'v> F: Fn(usize, &'v mut [T]),
    {
        let channels = self.channel_count();
        is! { channels == 0, return }
        for (frame, samples) in self.data.chunks_exact_mut(channels).enumerate() {
            f(frame, samples);
        }
    }
}

/// # Shared planar slices
/// Methods for borrowed per-channel PCM sample planes.
impl<'a, T> PcmBuf<T, &'a [&'a [T]]> {
    /// Creates a PCM buffer from borrowed per-channel sample planes.
    pub const fn from_planar(data: &'a [&'a [T]], spec: PcmSpec) -> Self {
        Self::new(data, spec)
    }
    /* direct accessors*/
    /// Returns the per-channel sample planes.
    pub const fn planes(&self) -> &'a [&'a [T]] {
        self.data
    }
    /* shape queries*/
    /// Returns the number of planes.
    #[must_use]
    pub const fn planes_len(&self) -> usize {
        self.data.len()
    }
    /// Returns whether there are no planes.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Returns the frame count, or `None` if planes are incomplete or uneven.
    #[must_use]
    pub const fn frames(&self) -> Option<usize> {
        let planes = self.data.len();
        is! { planes != self.channel_count(), return None }
        is! { planes == 0, return Some(0) }
        let frames = self.data[0].len();
        whilst! { i in 1..planes; {
            is! { self.data[i].len() != frames, return None }
        }}
        Some(frames)
    }
    /// Returns whether the plane data contains complete frames.
    #[must_use]
    pub const fn has_complete_frames(&self) -> bool {
        self.frames().is_some()
    }
    /* visitors */
    /// Visits the per-channel sample planes as shared slice.
    pub fn visit_planes<F, R>(&self, f: F) -> R
    where
        for<'v> F: FnOnce(&'v [&'a [T]]) -> R,
    {
        f(self.data)
    }
    /// Visits each per-channel sample plane.
    pub fn visit_each_plane<F>(&self, f: F)
    where
        for<'v> F: Fn(usize, &'v [T]),
    {
        for (i, plane) in self.data.iter().enumerate() {
            f(i, plane);
        }
    }
}

/// # Exclusive planar slices
/// Methods for mutably borrowed per-channel PCM sample planes.
impl<'a, 'p, T> PcmBuf<T, &'a mut [&'p mut [T]]> {
    /// Creates a PCM buffer from mutably borrowed per-channel sample planes.
    pub const fn from_planar_mut(data: &'a mut [&'p mut [T]], spec: PcmSpec) -> Self {
        Self::new(data, spec)
    }
    /* direct accessors*/
    /// Returns the per-channel sample planes.
    pub const fn planes(&self) -> &[&'p mut [T]] {
        &*self.data
    }
    /// Returns the mutable per-channel sample planes.
    pub const fn planes_mut(&mut self) -> &mut [&'p mut [T]] {
        &mut *self.data
    }
    /// Returns one plane as a shared sample slice.
    pub fn plane(&self, index: usize) -> Option<&[T]> {
        self.data.get(index).map(|plane| &**plane)
    }
    /// Returns one plane as a mutable sample slice.
    pub fn plane_mut(&mut self, index: usize) -> Option<&mut [T]> {
        self.data.get_mut(index).map(|plane| &mut **plane)
    }
    /* shape queries*/
    /// Returns whether there are no sample planes.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Returns the number of sample planes.
    #[must_use]
    pub const fn planes_len(&self) -> usize {
        self.data.len()
    }
    /// Returns the frame count, or `None` if the planes are incomplete.
    #[must_use]
    pub const fn frames(&self) -> Option<usize> {
        let planes = self.data.len();
        is! { planes != self.channel_count(), return None }
        is! { planes == 0, return Some(0) }
        let frames = self.data[0].len();
        whilst! { i in 1..planes; {
            is! { self.data[i].len() != frames, return None }
        }}
        Some(frames)
    }
    /// Returns whether the planes contain complete frames.
    #[must_use]
    pub const fn has_complete_frames(&self) -> bool {
        self.frames().is_some()
    }
    /* visitors */
    /// Visits the per-channel sample planes as an exclusive slice.
    pub fn visit_planes_mut<F, R>(&mut self, f: F) -> R
    where
        for<'v> F: FnOnce(&'v mut [&'p mut [T]]) -> R,
    {
        f(self.data)
    }
    /// Visits each per-channel sample plane mutably.
    pub fn visit_each_plane_mut<F>(&mut self, f: F)
    where
        for<'v> F: Fn(usize, &'v mut [T]),
    {
        for (i, plane) in self.data.iter_mut().enumerate() {
            f(i, plane);
        }
    }
    /// Visits each sample mutably, in plane-major order.
    pub fn visit_each_mut<F>(&mut self, f: F)
    where
        for<'v> F: Fn(usize, usize, &'v mut T),
    {
        for (channel, plane) in self.data.iter_mut().enumerate() {
            for (frame, sample) in plane.iter_mut().enumerate() {
                f(channel, frame, sample);
            }
        }
    }
}
