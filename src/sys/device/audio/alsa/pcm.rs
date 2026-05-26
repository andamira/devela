// devela::sys::device::audio::alsa::pcm
//
//! Defines [`AlsaPcmHandle`].
//

use super::_raw;
use crate::{AlsaError, PcmBuf, PcmSampleType, PcmSpec};
use crate::{Ptr, c_int, c_uint, is};

#[cfg(target_pointer_width = "64")]
crate::test_size_of![AlsaPcmHandle = 16]; // 128 bits

#[doc = crate::_tags!(audio linux guard)]
/// Owned ALSA PCM stream handle.
#[doc = crate::_doc_location!("sys/device/audio")]
#[derive(Debug)]
pub struct AlsaPcmHandle {
    raw: *mut _raw::snd_pcm_t,
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
impl AlsaPcmHandle {
    pub(super) const unsafe fn from_raw(raw: *mut _raw::snd_pcm_t) -> Self {
        Self { raw, spec: None }
    }

    /* configuration */

    /// Configures this PCM stream for interleaved read/write.
    ///
    /// Returns the actual spec accepted by ALSA. The sample rate may be adjusted.
    pub fn configure_interleaved(&mut self, spec: PcmSpec) -> Result<PcmSpec, AlsaError> {
        self.configure_access(spec, _raw::SND_PCM_ACCESS_RW_INTERLEAVED)
    }
    /// Returns the PCM metadata, if configured.
    pub const fn spec(&self) -> Option<PcmSpec> {
        self.spec
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
        self.validate_pcm(&pcm)?;
        self.write_pcm(pcm)
    }
    /// Writes interleaved samples from a slice using the configured stream spec.
    /// Returns the number of frames written.
    pub fn write_from<T: PcmSampleType>(&mut self, data: &[T]) -> Result<usize, AlsaError> {
        let spec = self.spec_for_sample::<T>()?;
        self.write_pcm(PcmBuf::from_interleaved(data, spec))
    }
    /// Writes all requested interleaved samples.
    pub fn write_all<T: PcmSampleType>(&mut self, pcm: PcmBuf<T, &[T]>) -> Result<(), AlsaError> {
        self.validate_pcm(&pcm)?;
        self.write_all_pcm(pcm)
    }
    /// Writes all complete interleaved frames from the given sample slice.
    pub fn write_all_from<T: PcmSampleType>(&mut self, data: &[T]) -> Result<(), AlsaError> {
        let spec = self.spec_for_sample::<T>()?;
        self.write_all_pcm(PcmBuf::from_interleaved(data, spec))
    }

    /* interleaved capture */

    /// Reads interleaved frames. Returns the number of frames read.
    pub fn read<T: PcmSampleType>(&mut self, pcm: PcmBuf<T, &mut [T]>) -> Result<usize, AlsaError> {
        self.validate_pcm(&pcm)?;
        self.read_pcm(pcm)
    }
    /// Reads interleaved samples into a slice using the configured stream spec.
    /// Returns the number of frames read.
    pub fn read_into<T: PcmSampleType>(&mut self, data: &mut [T]) -> Result<usize, AlsaError> {
        let spec = self.spec_for_sample::<T>()?;
        self.read_pcm(PcmBuf::from_interleaved_mut(data, spec))
    }
    /// Reads all requested interleaved frames.
    pub fn read_all<T: PcmSampleType>(
        &mut self,
        pcm: PcmBuf<T, &mut [T]>,
    ) -> Result<(), AlsaError> {
        self.validate_pcm(&pcm)?;
        self.read_all_pcm(pcm)
    }
    /// Reads all requested complete interleaved frames into the given sample slice.
    pub fn read_all_into<T: PcmSampleType>(&mut self, data: &mut [T]) -> Result<(), AlsaError> {
        let spec = self.spec_for_sample::<T>()?;
        self.read_all_pcm(PcmBuf::from_interleaved_mut(data, spec))
    }
}
// Private helpers
#[cfg(ffi_alsa··)]
impl AlsaPcmHandle {
    /* configuration and validation*/
    // Configures this PCM stream for the given ALSA access mode.
    fn configure_access(
        &mut self,
        mut spec: PcmSpec,
        access: _raw::snd_pcm_access_t,
    ) -> Result<PcmSpec, AlsaError> {
        if !spec.has_valid_rate() || spec.channel_count() == 0 {
            return Err(AlsaError::invalid_argument());
        }
        unsafe {
            let hw = HwParams::new(self.raw)?;
            AlsaError::result(_raw::snd_pcm_hw_params_set_access(self.raw, hw.raw, access))?;
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
            Ok(spec)
        }
    }
    // Validates that `pcm` matches both its Rust sample type and this configured stream.
    fn validate_pcm<T: PcmSampleType, B>(&self, pcm: &PcmBuf<T, B>) -> Result<(), AlsaError> {
        let spec = self.spec.ok_or_else(AlsaError::invalid_argument)?;
        is![pcm.spec.sample != T::SAMPLE, return Err(AlsaError::invalid_argument())];
        is![pcm.spec != spec, return Err(AlsaError::invalid_argument())];
        Ok(())
    }
    // Returns the configured spec if it matches the Rust sample type.
    fn spec_for_sample<T: PcmSampleType>(&self) -> Result<PcmSpec, AlsaError> {
        let spec = self.spec.ok_or_else(AlsaError::invalid_argument)?;
        is![spec.sample != T::SAMPLE, return Err(AlsaError::invalid_argument())];
        Ok(spec)
    }
    /* interleaved playback */
    // Writes interleaved frames. Returns the number of frames written.
    fn write_pcm<T>(&mut self, pcm: PcmBuf<T, &[T]>) -> Result<usize, AlsaError> {
        let frames = pcm.frames().ok_or_else(AlsaError::invalid_argument)?;
        let frames = frames as _raw::snd_pcm_uframes_t;
        unsafe {
            let written = _raw::snd_pcm_writei(self.raw, pcm.data.as_ptr().cast(), frames);
            self.io_result(written)
        }
    }
    // Writes all requested interleaved frames.
    fn write_all_pcm<T>(&mut self, pcm: PcmBuf<T, &[T]>) -> Result<(), AlsaError> {
        let channels = pcm.channel_count();
        let frames = pcm.frames().ok_or_else(AlsaError::invalid_argument)?;
        let mut done = 0;
        while done < frames {
            let offset = done * channels;
            let sub = PcmBuf::new(&pcm.data[offset..], pcm.spec);
            let n = self.write_pcm(sub)?;
            is! { n == 0, return Err(AlsaError::invalid_argument()) }
            done += n;
        }
        Ok(())
    }
    /* interleaved capture */
    // Reads interleaved frames. Returns the number of frames read.
    fn read_pcm<T>(&mut self, mut pcm: PcmBuf<T, &mut [T]>) -> Result<usize, AlsaError> {
        let frames = pcm.frames().ok_or_else(AlsaError::invalid_argument)?;
        let frames = frames as _raw::snd_pcm_uframes_t;
        unsafe {
            let read = _raw::snd_pcm_readi(self.raw, pcm.data_mut().as_mut_ptr().cast(), frames);
            self.io_result(read)
        }
    }
    // Reads all requested interleaved frames.
    fn read_all_pcm<T>(&mut self, pcm: PcmBuf<T, &mut [T]>) -> Result<(), AlsaError> {
        let channel_count = pcm.channel_count();
        let frames = pcm.frames().ok_or_else(AlsaError::invalid_argument)?;
        let mut done = 0;
        while done < frames {
            let offset = done * channel_count;
            let remaining = frames - done;
            let sub = PcmBuf::new(&mut pcm.data[offset..], pcm.spec);
            let n = self.read_pcm(sub)?;
            is! { n == 0 || n > remaining, return Err(AlsaError::invalid_argument()) }
            done += n;
        }
        Ok(())
    }
    /* low-level result handling */
    fn io_result(&mut self, frames: _raw::snd_pcm_sframes_t) -> Result<usize, AlsaError> {
        if frames >= 0 {
            Ok(frames as usize)
        } else {
            let err = frames as c_int;
            unsafe {
                AlsaError::result(_raw::snd_pcm_recover(self.raw, err, 1))?;
            }
            Err(AlsaError::new(err))
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
