// devela::sys::device::audio::alsa::pcm
//
//! Defines [`AlsaPcmHandle`].
//
// TOC
// - struct AlsaPcmHandle
// - public impls
// - trait impls
// - internal helpers
//   - internal impls
//   - struct HwParams

use super::_raw;
use crate::{AlsaError, AudioChannels, PcmBuf, PcmLayout, PcmSampleType, PcmSpec};
use crate::{Ptr, c_int, c_uint, is, whilst};

#[cfg(target_pointer_width = "64")]
crate::test_size_of![AlsaPcmHandle = 24]; // 192 bits

#[doc = crate::_tags!(audio linux guard)]
/// Owned ALSA PCM stream handle.
#[doc = crate::_doc_meta!{location("sys/device/audio")}]
#[derive(Debug)]
pub struct AlsaPcmHandle {
    raw: *mut _raw::snd_pcm_t,
    layout: Option<PcmLayout>,
    spec: Option<PcmSpec>,
}

unsafe impl Send for AlsaPcmHandle {}
#[cfg(ffi_alsa··)]
impl Drop for AlsaPcmHandle {
    fn drop(&mut self) {
        unsafe {
            let _ = _raw::snd_pcm_close(self.raw);
        }
    }
}

#[cfg(ffi_alsa··)]
#[rustfmt::skip]
mod impl_traits {
    use super::*;
    use crate::{PcmDrain, PcmSink, PcmSinkPlanar, PcmSource, PcmSourcePlanar, PcmStream};

    impl PcmStream for AlsaPcmHandle {
        type Error = AlsaError;
        fn pcm_spec(&self) -> Option<PcmSpec> { self.spec }
        fn pcm_layout(&self) -> Option<PcmLayout> { self.layout }
    }
    impl PcmDrain for AlsaPcmHandle {
        fn drain(&mut self) -> Result<(), AlsaError> { self.drain() }
    }
    impl<T: PcmSampleType> PcmSink<T> for AlsaPcmHandle {
        fn write(&mut self, pcm: PcmBuf<T, &[T]>) -> Result<usize, AlsaError> { self.write(pcm) }
        fn write_all(&mut self, pcm: PcmBuf<T, &[T]>) -> Result<(), AlsaError> {
            self.write_all(pcm)
        }
    }
    impl<T: PcmSampleType> PcmSource<T> for AlsaPcmHandle {
        fn read(&mut self, pcm: PcmBuf<T, &mut [T]>) -> Result<usize, AlsaError> { self.read(pcm) }
        fn read_all(&mut self, pcm: PcmBuf<T, &mut [T]>) -> Result<(), AlsaError> {
            self.read_all(pcm)
        }
    }
    impl<T: PcmSampleType> PcmSinkPlanar<T> for AlsaPcmHandle {
        fn write_planar(&mut self, pcm: PcmBuf<T, &[&[T]]>) -> Result<usize, AlsaError> {
            self.write_planar(pcm)
        }
        fn write_all_planar(&mut self, pcm: PcmBuf<T, &[&[T]]>) -> Result<(), Self::Error> {
            self.write_all_planar(pcm)
        }
    }
    impl<T: PcmSampleType> PcmSourcePlanar<T> for AlsaPcmHandle {
        fn read_planar(&mut self, pcm: PcmBuf<T, &mut [&mut [T]]>) -> Result<usize, AlsaError> {
            self.read_planar(pcm)
        }
        fn read_all_planar(&mut self, pcm: PcmBuf<T, &mut [&mut [T]]>) -> Result<(), Self::Error> {
            self.read_all_planar(pcm)
        }
    }
}

#[cfg(ffi_alsa··)]
impl AlsaPcmHandle {
    /* configuration */

    /// Configures this PCM stream for interleaved read/write.
    ///
    /// Returns the actual spec accepted by ALSA. The sample rate may be adjusted.
    pub fn configure_interleaved(&mut self, spec: PcmSpec) -> Result<PcmSpec, AlsaError> {
        self.configure_access(spec, PcmLayout::Interleaved)
    }
    /// Configures this PCM stream for planar read/write.
    ///
    /// Returns the actual spec accepted by ALSA. The sample rate may be adjusted.
    pub fn configure_planar(&mut self, spec: PcmSpec) -> Result<PcmSpec, AlsaError> {
        self.configure_access(spec, PcmLayout::Planar)
    }
    /// Prepares this stream for I/O.
    pub fn prepare(&mut self) -> Result<(), AlsaError> {
        unsafe { AlsaError::result(_raw::snd_pcm_prepare(self.raw)) }
    }
    /// Waits until queued playback frames have been played.
    pub fn drain(&mut self) -> Result<(), AlsaError> {
        unsafe { AlsaError::result(_raw::snd_pcm_drain(self.raw)) }
    }
    /// Stops the stream and discards queued frames.
    pub fn discard(&mut self) -> Result<(), AlsaError> {
        unsafe { AlsaError::result(_raw::snd_pcm_drop(self.raw)) }
    }

    /* interleaved playback */

    /// Writes interleaved frames. Returns the number of frames written.
    pub fn write<T: PcmSampleType>(&mut self, pcm: PcmBuf<T, &[T]>) -> Result<usize, AlsaError> {
        self.validate_pcm(&pcm, PcmLayout::Interleaved)?;
        self.write_pcm(pcm)
    }
    /// Writes interleaved samples from a slice using the configured stream spec.
    /// Returns the number of frames written.
    pub fn write_from<T: PcmSampleType>(&mut self, data: &[T]) -> Result<usize, AlsaError> {
        let spec = self.spec_for_sample::<T>(PcmLayout::Interleaved)?;
        self.write_pcm(PcmBuf::from_interleaved(data, spec))
    }
    /// Writes all requested interleaved samples.
    pub fn write_all<T: PcmSampleType>(&mut self, pcm: PcmBuf<T, &[T]>) -> Result<(), AlsaError> {
        self.validate_pcm(&pcm, PcmLayout::Interleaved)?;
        self.write_all_pcm(pcm)
    }
    /// Writes all complete interleaved frames from the given sample slice.
    pub fn write_all_from<T: PcmSampleType>(&mut self, data: &[T]) -> Result<(), AlsaError> {
        let spec = self.spec_for_sample::<T>(PcmLayout::Interleaved)?;
        self.write_all_pcm(PcmBuf::from_interleaved(data, spec))
    }

    /* planar playback */

    /// Writes planar frames. Returns the number of frames written.
    pub fn write_planar<T: PcmSampleType>(
        &mut self,
        pcm: PcmBuf<T, &[&[T]]>,
    ) -> Result<usize, AlsaError> {
        self.validate_pcm(&pcm, PcmLayout::Planar)?;
        self.write_planar_pcm(pcm)
    }
    /// Writes all requested planar frames.
    pub fn write_all_planar<T: PcmSampleType>(
        &mut self,
        pcm: PcmBuf<T, &[&[T]]>,
    ) -> Result<(), AlsaError> {
        self.validate_pcm(&pcm, PcmLayout::Planar)?;
        self.write_all_planar_pcm(pcm)
    }

    /* interleaved capture */

    /// Reads interleaved frames. Returns the number of frames read.
    pub fn read<T: PcmSampleType>(&mut self, pcm: PcmBuf<T, &mut [T]>) -> Result<usize, AlsaError> {
        self.validate_pcm(&pcm, PcmLayout::Interleaved)?;
        self.read_pcm(pcm)
    }
    /// Reads interleaved samples into a slice using the configured stream spec.
    /// Returns the number of frames read.
    pub fn read_into<T: PcmSampleType>(&mut self, data: &mut [T]) -> Result<usize, AlsaError> {
        let spec = self.spec_for_sample::<T>(PcmLayout::Interleaved)?;
        self.read_pcm(PcmBuf::from_interleaved_mut(data, spec))
    }
    /// Reads all requested interleaved frames.
    pub fn read_all<T: PcmSampleType>(
        &mut self,
        pcm: PcmBuf<T, &mut [T]>,
    ) -> Result<(), AlsaError> {
        self.validate_pcm(&pcm, PcmLayout::Interleaved)?;
        self.read_all_pcm(pcm)
    }
    /// Reads all requested complete interleaved frames into the given sample slice.
    pub fn read_all_into<T: PcmSampleType>(&mut self, data: &mut [T]) -> Result<(), AlsaError> {
        let spec = self.spec_for_sample::<T>(PcmLayout::Interleaved)?;
        self.read_all_pcm(PcmBuf::from_interleaved_mut(data, spec))
    }

    /* planar capture */

    /// Reads planar frames. Returns the number of frames read.
    pub fn read_planar<T: PcmSampleType>(
        &mut self,
        pcm: PcmBuf<T, &mut [&mut [T]]>,
    ) -> Result<usize, AlsaError> {
        self.validate_pcm(&pcm, PcmLayout::Planar)?;
        self.read_planar_pcm(pcm)
    }
    /// Reads all requested planar frames.
    pub fn read_all_planar<T: PcmSampleType>(
        &mut self,
        pcm: PcmBuf<T, &mut [&mut [T]]>,
    ) -> Result<(), AlsaError> {
        self.validate_pcm(&pcm, PcmLayout::Planar)?;
        self.read_all_planar_pcm(pcm)
    }
}

/* private helpers */

#[cfg(ffi_alsa··)]
impl AlsaPcmHandle {
    pub(super) const unsafe fn from_raw(raw: *mut _raw::snd_pcm_t) -> Self {
        Self { raw, layout: None, spec: None }
    }

    /* configuration and validation*/

    /// Configures this PCM stream for the given ALSA access mode.
    fn configure_access(
        &mut self,
        mut spec: PcmSpec,
        layout: PcmLayout,
    ) -> Result<PcmSpec, AlsaError> {
        if !spec.has_valid_rate() || spec.channel_count() == 0 {
            return Err(AlsaError::invalid_spec(spec));
        }
        unsafe {
            let hw = HwParams::new(self.raw)?;
            AlsaError::result(_raw::snd_pcm_hw_params_set_access(
                self.raw,
                hw.raw,
                layout.to_alsa(),
            ))?;
            AlsaError::result(_raw::snd_pcm_hw_params_set_format(
                self.raw,
                hw.raw,
                spec.sample.to_alsa(),
            ))?;
            let mut rate = spec.sample_rate as c_uint;
            AlsaError::result(_raw::snd_pcm_hw_params_set_rate_near(
                self.raw,
                hw.raw,
                &mut rate,
                Ptr::null_mut(),
            ))?;
            spec.sample_rate = rate;
            AlsaError::result(_raw::snd_pcm_hw_params_set_channels(
                self.raw,
                hw.raw,
                spec.channel_count() as c_uint,
            ))?;
            AlsaError::result(_raw::snd_pcm_hw_params(self.raw, hw.raw))?;
            self.prepare()?;
            self.spec = Some(spec);
            self.layout = Some(layout);
            Ok(spec)
        }
    }
    /// Validates that `pcm` matches the Rust sample type and configured stream.
    fn validate_pcm<T, B>(
        &self,
        pcm: &PcmBuf<T, B>,
        expected_layout: PcmLayout,
    ) -> Result<(), AlsaError>
    where
        T: PcmSampleType,
    {
        let spec = self.spec.ok_or(AlsaError::Unconfigured)?;
        let active_layout = self.layout.ok_or(AlsaError::LayoutUnconfigured)?;
        if active_layout != expected_layout {
            return Err(AlsaError::LayoutMismatch {
                active: active_layout,
                expected: expected_layout,
            });
        }
        if pcm.spec.sample != T::SAMPLE {
            return Err(AlsaError::SampleMismatch { pcm: pcm.spec.sample, ty: T::SAMPLE });
        }
        if pcm.spec != spec {
            return Err(AlsaError::SpecMismatch { buffer: pcm.spec, stream: spec });
        }
        Ok(())
    }
    /// Returns the configured spec if it matches the Rust sample type and layout.
    fn spec_for_sample<T>(&self, expected_layout: PcmLayout) -> Result<PcmSpec, AlsaError>
    where
        T: PcmSampleType,
    {
        let spec = self.spec.ok_or(AlsaError::Unconfigured)?;
        let active_layout = self.layout.ok_or(AlsaError::LayoutUnconfigured)?;
        if active_layout != expected_layout {
            return Err(AlsaError::layout_mismatch(active_layout, expected_layout));
        }
        if spec.sample != T::SAMPLE {
            return Err(AlsaError::sample_mismatch(spec.sample, T::SAMPLE));
        }
        Ok(spec)
    }
    /// Validates progress from a partial read/write operation.
    fn check_progress(n: usize, remaining: usize) -> Result<(), AlsaError> {
        is![n == 0, return Err(AlsaError::NoProgress)];
        is![n > remaining, return Err(AlsaError::frame_count_exceeded(n, remaining))];
        Ok(())
    }

    /* interleaved playback */

    /// Writes interleaved frames. Returns the number of frames written.
    fn write_pcm<T>(&mut self, pcm: PcmBuf<T, &[T]>) -> Result<usize, AlsaError> {
        let frames = pcm.frames().ok_or_else(AlsaError::incomplete_frames)?;
        let frames = frames as _raw::snd_pcm_uframes_t;
        unsafe {
            let written = _raw::snd_pcm_writei(self.raw, pcm.data.as_ptr().cast(), frames);
            self.io_result(written)
        }
    }
    /// Writes all requested interleaved frames.
    fn write_all_pcm<T>(&mut self, pcm: PcmBuf<T, &[T]>) -> Result<(), AlsaError> {
        let channels = pcm.channel_count();
        let frames = pcm.frames().ok_or_else(AlsaError::incomplete_frames)?;
        let mut done = 0;
        while done < frames {
            let offset = done * channels;
            let remaining = frames - done;
            let sub = PcmBuf::new(&pcm.data[offset..], pcm.spec);
            let n = self.write_pcm(sub)?;
            Self::check_progress(n, remaining)?;
            done += n;
        }
        Ok(())
    }

    /* planar playback */

    /// Writes planar frames. Returns the number of frames written.
    fn write_planar_pcm<T: PcmSampleType>(
        &mut self,
        pcm: PcmBuf<T, &[&[T]]>,
    ) -> Result<usize, AlsaError> {
        let frames = pcm.frames().ok_or_else(AlsaError::incomplete_frames)?;
        let frames = frames as _raw::snd_pcm_uframes_t;
        let planes = pcm.planes_len();
        let mut ptrs = [Ptr::null_mut(); AudioChannels::MAX_COUNT];
        is![planes > ptrs.len(), return Err(AlsaError::too_many_channels(planes, ptrs.len()))];
        whilst! { i in 0..planes; { ptrs[i] = pcm.data[i].as_ptr().cast_mut().cast(); }}
        unsafe {
            let written = _raw::snd_pcm_writen(self.raw, ptrs.as_mut_ptr(), frames);
            self.io_result(written)
        }
    }
    /// Writes all requested planar frames.
    fn write_all_planar_pcm<T: PcmSampleType>(
        &mut self,
        pcm: PcmBuf<T, &[&[T]]>,
    ) -> Result<(), AlsaError> {
        let frames = pcm.frames().ok_or_else(AlsaError::incomplete_frames)?;
        let planes_len = pcm.planes_len();
        let mut done = 0;
        while done < frames {
            let remaining = frames - done;
            let mut planes = [<&[T]>::default(); AudioChannels::MAX_COUNT];
            if planes_len > planes.len() {
                return Err(AlsaError::too_many_channels(planes_len, planes.len()));
            }
            whilst! { i in 0..planes_len; { planes[i] = &pcm.data[i][done..]; }}
            let sub = PcmBuf::from_planar(&planes[..planes_len], pcm.spec);
            let n = self.write_planar_pcm(sub)?;
            Self::check_progress(n, remaining)?;
            done += n;
        }
        Ok(())
    }

    /* interleaved capture */

    /// Reads interleaved frames. Returns the number of frames read.
    fn read_pcm<T>(&mut self, mut pcm: PcmBuf<T, &mut [T]>) -> Result<usize, AlsaError> {
        let frames = pcm.frames().ok_or_else(AlsaError::incomplete_frames)?;
        let frames = frames as _raw::snd_pcm_uframes_t;
        unsafe {
            let read = _raw::snd_pcm_readi(self.raw, pcm.data_mut().as_mut_ptr().cast(), frames);
            self.io_result(read)
        }
    }
    /// Reads all requested interleaved frames.
    fn read_all_pcm<T>(&mut self, pcm: PcmBuf<T, &mut [T]>) -> Result<(), AlsaError> {
        let channel_count = pcm.channel_count();
        let frames = pcm.frames().ok_or_else(AlsaError::incomplete_frames)?;
        let mut done = 0;
        while done < frames {
            let offset = done * channel_count;
            let remaining = frames - done;
            let sub = PcmBuf::new(&mut pcm.data[offset..], pcm.spec);
            let n = self.read_pcm(sub)?;
            Self::check_progress(n, remaining)?;
            done += n;
        }
        Ok(())
    }

    /* planar capture */

    /// Reads planar frames. Returns the number of frames read.
    fn read_planar_pcm<T: PcmSampleType>(
        &mut self,
        mut pcm: PcmBuf<T, &mut [&mut [T]]>,
    ) -> Result<usize, AlsaError> {
        let frames = pcm.frames().ok_or_else(AlsaError::incomplete_frames)?;
        let frames = frames as _raw::snd_pcm_uframes_t;
        let planes = pcm.planes_len();
        let mut ptrs = [Ptr::null_mut(); AudioChannels::MAX_COUNT];
        is![planes > ptrs.len(), return Err(AlsaError::too_many_channels(planes, ptrs.len()))];
        let data = pcm.planes_mut();
        whilst! { i in 0..planes; { ptrs[i] = data[i].as_mut_ptr().cast(); }}
        unsafe {
            let read = _raw::snd_pcm_readn(self.raw, ptrs.as_mut_ptr(), frames);
            self.io_result(read)
        }
    }
    /// Reads all requested planar frames.
    fn read_all_planar_pcm<T: PcmSampleType>(
        &mut self,
        mut pcm: PcmBuf<T, &mut [&mut [T]]>,
    ) -> Result<(), AlsaError> {
        let frames = pcm.frames().ok_or_else(AlsaError::incomplete_frames)?;
        let planes = pcm.planes_len();
        let data = pcm.planes_mut();
        let mut done = 0;
        while done < frames {
            let remaining = frames - done;
            let mut ptrs = [Ptr::null_mut(); AudioChannels::MAX_COUNT];
            is![planes > ptrs.len(), return Err(AlsaError::too_many_channels(planes, ptrs.len()))];
            whilst! { i in 0..planes; { ptrs[i] = data[i][done..].as_mut_ptr().cast(); }}
            unsafe {
                let raw_remaining = remaining as _raw::snd_pcm_uframes_t;
                let read = _raw::snd_pcm_readn(self.raw, ptrs.as_mut_ptr(), raw_remaining);
                let n = self.io_result(read)?;
                Self::check_progress(n, remaining)?;
                done += n;
            }
        }
        Ok(())
    }

    /* low-level result handling */
    fn io_result(&mut self, frames: _raw::snd_pcm_sframes_t) -> Result<usize, AlsaError> {
        if frames >= 0 {
            Ok(frames as usize)
        } else {
            let recovered = unsafe { _raw::snd_pcm_recover(self.raw, frames as c_int, 1) };
            if recovered < 0 { Err(AlsaError::Code(recovered)) } else { Ok(0) }
        }
    }
}

#[cfg(ffi_alsa··)]
struct HwParams {
    raw: *mut _raw::snd_pcm_hw_params_t,
}
#[cfg(ffi_alsa··)]
impl HwParams {
    fn new(pcm: *mut _raw::snd_pcm_t) -> Result<Self, AlsaError> {
        unsafe {
            let mut raw = Ptr::null_mut();
            AlsaError::result(_raw::snd_pcm_hw_params_malloc(&mut raw))?;
            AlsaError::result(_raw::snd_pcm_hw_params_any(pcm, raw))?;
            Ok(Self { raw })
        }
    }
}
#[cfg(ffi_alsa··)]
impl Drop for HwParams {
    fn drop(&mut self) {
        unsafe { _raw::snd_pcm_hw_params_free(self.raw) }
    }
}
