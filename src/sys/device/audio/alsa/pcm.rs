// devela::sys::device::audio::alsa::pcm
//
//! Defines [`AlsaPcmHandle`].
//

use super::_raw;
use crate::{AlsaError, AudioChannels, PcmBuffer, PcmSample, PcmSpec};
use crate::{Ptr, c_int, c_uint, is};

#[doc = crate::_tags!(audio linux guard)]
/// Owned ALSA PCM stream handle.
#[doc = crate::_doc_location!("sys/device/audio")]
#[derive(Debug)]
pub struct AlsaPcmHandle {
    raw: *mut _raw::snd_pcm_t,
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
        Self { raw }
    }
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

    /// Configures this PCM stream for interleaved read/write.
    ///
    /// Returns the actual spec accepted by ALSA. The sample rate may be adjusted.
    pub fn configure_interleaved(&mut self, mut spec: PcmSpec) -> Result<PcmSpec, AlsaError> {
        if !spec.has_valid_rate() || spec.channel_count() == 0 {
            return Err(AlsaError::invalid_argument());
        }
        unsafe {
            let hw = HwParams::new(self.raw)?;
            AlsaError::result(_raw::snd_pcm_hw_params_set_access(
                self.raw,
                hw.raw,
                _raw::SND_PCM_ACCESS_RW_INTERLEAVED,
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
            Ok(spec)
        }
    }
    /// Prepares this stream for I/O.
    pub fn prepare(&mut self) -> Result<(), AlsaError> {
        unsafe { AlsaError::result(_raw::snd_pcm_prepare(self.raw)) }
    }

    /* write */

    /// Writes interleaved `u8` frames. Returns the number of frames written.
    pub fn write_u8(&mut self, pcm: PcmBuffer<'_, u8>) -> Result<usize, AlsaError> {
        self.write_pcm(pcm, PcmSample::U8)
    }
    /// Writes all requested interleaved `u8` frames.
    pub fn write_all_u8(&mut self, pcm: PcmBuffer<'_, u8>) -> Result<(), AlsaError> {
        self.write_all_pcm(pcm, Self::write_u8)
    }
    /// Writes interleaved `i16` frames. Returns the number of frames written.
    pub fn write_i16(&mut self, pcm: PcmBuffer<'_, i16>) -> Result<usize, AlsaError> {
        self.write_pcm(pcm, PcmSample::I16)
    }
    /// Writes all requested interleaved `i16` frames.
    pub fn write_all_i16(&mut self, pcm: PcmBuffer<'_, i16>) -> Result<(), AlsaError> {
        self.write_all_pcm(pcm, Self::write_i16)
    }
    /// Writes interleaved `i32` frames. Returns the number of frames written.
    pub fn write_i32(&mut self, pcm: PcmBuffer<'_, i32>) -> Result<usize, AlsaError> {
        self.write_pcm(pcm, PcmSample::I32)
    }
    /// Writes all requested interleaved `i32` frames.
    pub fn write_all_i32(&mut self, pcm: PcmBuffer<'_, i32>) -> Result<(), AlsaError> {
        self.write_all_pcm(pcm, Self::write_i32)
    }
    /// Writes interleaved `f32` frames. Returns the number of frames written.
    pub fn write_f32(&mut self, pcm: PcmBuffer<'_, f32>) -> Result<usize, AlsaError> {
        self.write_pcm(pcm, PcmSample::F32)
    }
    /// Writes all requested interleaved `f32` frames.
    pub fn write_all_f32(&mut self, pcm: PcmBuffer<'_, f32>) -> Result<(), AlsaError> {
        self.write_all_pcm(pcm, Self::write_f32)
    }

    /* read */

    /// Reads interleaved `u8` frames. Returns the number of frames read.
    pub fn read_u8(
        &mut self,
        data: &mut [u8],
        channels: AudioChannels,
    ) -> Result<usize, AlsaError> {
        self.read_pcm(data, channels)
    }
    /// Reads all requested interleaved `u8` frames.
    pub fn read_all_u8(
        &mut self,
        data: &mut [u8],
        channels: AudioChannels,
    ) -> Result<(), AlsaError> {
        self.read_all_pcm(data, channels, Self::read_u8)
    }
    /// Reads interleaved `i16` frames. Returns the number of frames read.
    pub fn read_i16(
        &mut self,
        data: &mut [i16],
        channels: AudioChannels,
    ) -> Result<usize, AlsaError> {
        self.read_pcm(data, channels)
    }
    /// Reads all requested interleaved `i16` frames.
    pub fn read_all_i16(
        &mut self,
        data: &mut [i16],
        channels: AudioChannels,
    ) -> Result<(), AlsaError> {
        self.read_all_pcm(data, channels, Self::read_i16)
    }
    /// Reads interleaved `i32` frames. Returns the number of frames read.
    pub fn read_i32(
        &mut self,
        data: &mut [i32],
        channels: AudioChannels,
    ) -> Result<usize, AlsaError> {
        self.read_pcm(data, channels)
    }
    /// Reads all requested interleaved `i32` frames.
    pub fn read_all_i32(
        &mut self,
        data: &mut [i32],
        channels: AudioChannels,
    ) -> Result<(), AlsaError> {
        self.read_all_pcm(data, channels, Self::read_i32)
    }
    /// Reads interleaved `f32` frames. Returns the number of frames read.
    pub fn read_f32(
        &mut self,
        data: &mut [f32],
        channels: AudioChannels,
    ) -> Result<usize, AlsaError> {
        self.read_pcm(data, channels)
    }
    /// Reads all requested interleaved `f32` frames.
    pub fn read_all_f32(
        &mut self,
        data: &mut [f32],
        channels: AudioChannels,
    ) -> Result<(), AlsaError> {
        self.read_all_pcm(data, channels, Self::read_f32)
    }

    /// Waits until queued playback frames have been played.
    pub fn drain(&mut self) -> Result<(), AlsaError> {
        unsafe { AlsaError::result(_raw::snd_pcm_drain(self.raw)) }
    }
    /// Stops the stream and discards queued frames.
    pub fn discard(&mut self) -> Result<(), AlsaError> {
        unsafe { AlsaError::result(_raw::snd_pcm_drop(self.raw)) }
    }
}
// Private helpers
#[cfg(ffi_alsa··)]
impl AlsaPcmHandle {
    // Writes interleaved frames. Returns the number of frames written.
    fn write_pcm<T>(
        &mut self,
        pcm: PcmBuffer<'_, T>,
        expected: PcmSample,
    ) -> Result<usize, AlsaError> {
        is![pcm.format() != expected, return Err(AlsaError::invalid_argument())];
        let frames = pcm.frames().ok_or_else(AlsaError::invalid_argument)?;
        let frames = frames as _raw::snd_pcm_uframes_t;
        unsafe {
            let written = _raw::snd_pcm_writei(self.raw, pcm.data.as_ptr().cast(), frames);
            self.io_result(written)
        }
    }
    // Writes all requested interleaved frames.
    fn write_all_pcm<T>(
        &mut self,
        pcm: PcmBuffer<'_, T>,
        mut write: impl FnMut(&mut Self, PcmBuffer<'_, T>) -> Result<usize, AlsaError>,
    ) -> Result<(), AlsaError> {
        let channels = pcm.channel_count();
        let frames = pcm.frames().ok_or_else(AlsaError::invalid_argument)?;
        let mut done = 0;
        while done < frames {
            let offset = done * channels;
            let sub =
                PcmBuffer::new(&pcm.data[offset..], pcm.format, pcm.channels, pcm.sample_rate);
            let n = write(self, sub)?;
            is! { n == 0, return Err(AlsaError::invalid_argument()) }
            done += n;
        }
        Ok(())
    }
    // Reads interleaved frames. Returns the number of frames read.
    fn read_pcm<T>(&mut self, data: &mut [T], channels: AudioChannels) -> Result<usize, AlsaError> {
        let channels = channels.channels() as usize;
        if channels == 0 || !data.len().is_multiple_of(channels) {
            return Err(AlsaError::invalid_argument());
        }
        let frames = (data.len() / channels) as _raw::snd_pcm_uframes_t;
        unsafe {
            let read = _raw::snd_pcm_readi(self.raw, data.as_mut_ptr().cast(), frames);
            self.io_result(read)
        }
    }
    // Reads all requested interleaved frames.
    fn read_all_pcm<T>(
        &mut self,
        data: &mut [T],
        channels: AudioChannels,
        mut read: impl FnMut(&mut Self, &mut [T], AudioChannels) -> Result<usize, AlsaError>,
    ) -> Result<(), AlsaError> {
        let channel_count = channels.channels() as usize;
        if channel_count == 0 || !data.len().is_multiple_of(channel_count) {
            return Err(AlsaError::invalid_argument());
        }
        let frames = data.len() / channel_count;
        let mut done = 0;
        while done < frames {
            let offset = done * channel_count;
            let remaining = frames - done;
            let n = read(self, &mut data[offset..], channels)?;
            is! { n == 0 || n > remaining, return Err(AlsaError::invalid_argument()) }
            done += n;
        }
        Ok(())
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
