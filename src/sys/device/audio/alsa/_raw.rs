// devela::sys::device::audio::alsa::_raw
//
//! Raw ALSA bindings.
//
// LINKS
// - https://www.alsa-project.org/alsa-doc/alsa-lib
// - https://www.kernel.org/doc/html/v4.14/sound/kernel-api/index.html
// - https://www.alemauri.eu/alsa/part1.html
// - https://alsa.opensrc.org/Asoundrc
// - https://soundprogramming.net/programming/alsa-tutorial-1-initialization/
// - https://web.archive.org/web/20110817205715/https://www.suse.de/~mana/alsa090_howto.html

#![allow(unused, nonstandard_style)] // WIP

use crate::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

/// The handle for the PCM device.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct snd_pcm_t {
    _unused: [u8; 0],
}

/// PCM hardware configurations container.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct snd_pcm_hw_params_t {
    _unused: [u8; 0],
}
/// PCM software configuration container.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct snd_pcm_sw_params_t {
    _unused: [u8; 0],
}

/// Unsigned frames quantity.
pub(crate) type snd_pcm_uframes_t = c_ulong;
/// Signed frames quantity.
pub(crate) type snd_pcm_sframes_t = c_long;

/* constant */

pub(crate) const SND_PCM_STREAM_PLAYBACK: snd_pcm_stream_t = 0;
pub(crate) const SND_PCM_STREAM_CAPTURE: snd_pcm_stream_t = 1;
pub(crate) const SND_PCM_STREAM_LAST: snd_pcm_stream_t = 1;
/// The direction of the PCM stream.
pub(crate) type snd_pcm_stream_t = c_uint;

pub(crate) const SND_PCM_ACCESS_MMAP_INTERLEAVED: snd_pcm_access_t = 0;
pub(crate) const SND_PCM_ACCESS_MMAP_NONINTERLEAVED: snd_pcm_access_t = 1;
pub(crate) const SND_PCM_ACCESS_MMAP_COMPLEX: snd_pcm_access_t = 2;
pub(crate) const SND_PCM_ACCESS_RW_INTERLEAVED: snd_pcm_access_t = 3;
pub(crate) const SND_PCM_ACCESS_RW_NONINTERLEAVED: snd_pcm_access_t = 4;
pub(crate) const SND_PCM_ACCESS_LAST: snd_pcm_access_t = 4;
pub(crate) type snd_pcm_access_t = c_uint;

pub(crate) const SND_PCM_FORMAT_UNKNOWN: snd_pcm_format_t = -1;
pub(crate) const SND_PCM_FORMAT_S8: snd_pcm_format_t = 0;
pub(crate) const SND_PCM_FORMAT_U8: snd_pcm_format_t = 1;
pub(crate) const SND_PCM_FORMAT_S16_LE: snd_pcm_format_t = 2;
pub(crate) const SND_PCM_FORMAT_S16_BE: snd_pcm_format_t = 3;
pub(crate) const SND_PCM_FORMAT_U16_LE: snd_pcm_format_t = 4;
pub(crate) const SND_PCM_FORMAT_U16_BE: snd_pcm_format_t = 5;
pub(crate) const SND_PCM_FORMAT_S24_LE: snd_pcm_format_t = 6;
pub(crate) const SND_PCM_FORMAT_S24_BE: snd_pcm_format_t = 7;
pub(crate) const SND_PCM_FORMAT_U24_LE: snd_pcm_format_t = 8;
pub(crate) const SND_PCM_FORMAT_U24_BE: snd_pcm_format_t = 9;
pub(crate) const SND_PCM_FORMAT_S32_LE: snd_pcm_format_t = 10;
pub(crate) const SND_PCM_FORMAT_S32_BE: snd_pcm_format_t = 11;
pub(crate) const SND_PCM_FORMAT_U32_LE: snd_pcm_format_t = 12;
pub(crate) const SND_PCM_FORMAT_U32_BE: snd_pcm_format_t = 13;
pub(crate) const SND_PCM_FORMAT_FLOAT_LE: snd_pcm_format_t = 14;
pub(crate) const SND_PCM_FORMAT_FLOAT_BE: snd_pcm_format_t = 15;
pub(crate) const SND_PCM_FORMAT_FLOAT64_LE: snd_pcm_format_t = 16;
pub(crate) const SND_PCM_FORMAT_FLOAT64_BE: snd_pcm_format_t = 17;
pub(crate) const SND_PCM_FORMAT_IEC958_SUBFRAME_LE: snd_pcm_format_t = 18;
pub(crate) const SND_PCM_FORMAT_IEC958_SUBFRAME_BE: snd_pcm_format_t = 19;
pub(crate) const SND_PCM_FORMAT_MU_LAW: snd_pcm_format_t = 20;
pub(crate) const SND_PCM_FORMAT_A_LAW: snd_pcm_format_t = 21;
pub(crate) const SND_PCM_FORMAT_IMA_ADPCM: snd_pcm_format_t = 22;
pub(crate) const SND_PCM_FORMAT_MPEG: snd_pcm_format_t = 23;
pub(crate) const SND_PCM_FORMAT_GSM: snd_pcm_format_t = 24;
pub(crate) const SND_PCM_FORMAT_S20_LE: snd_pcm_format_t = 25;
pub(crate) const SND_PCM_FORMAT_S20_BE: snd_pcm_format_t = 26;
pub(crate) const SND_PCM_FORMAT_U20_LE: snd_pcm_format_t = 27;
pub(crate) const SND_PCM_FORMAT_U20_BE: snd_pcm_format_t = 28;
pub(crate) const SND_PCM_FORMAT_SPECIAL: snd_pcm_format_t = 31;
pub(crate) const SND_PCM_FORMAT_S24_3LE: snd_pcm_format_t = 32;
pub(crate) const SND_PCM_FORMAT_S24_3BE: snd_pcm_format_t = 33;
pub(crate) const SND_PCM_FORMAT_U24_3LE: snd_pcm_format_t = 34;
pub(crate) const SND_PCM_FORMAT_U24_3BE: snd_pcm_format_t = 35;
pub(crate) const SND_PCM_FORMAT_S20_3LE: snd_pcm_format_t = 36;
pub(crate) const SND_PCM_FORMAT_S20_3BE: snd_pcm_format_t = 37;
pub(crate) const SND_PCM_FORMAT_U20_3LE: snd_pcm_format_t = 38;
pub(crate) const SND_PCM_FORMAT_U20_3BE: snd_pcm_format_t = 39;
pub(crate) const SND_PCM_FORMAT_S18_3LE: snd_pcm_format_t = 40;
pub(crate) const SND_PCM_FORMAT_S18_3BE: snd_pcm_format_t = 41;
pub(crate) const SND_PCM_FORMAT_U18_3LE: snd_pcm_format_t = 42;
pub(crate) const SND_PCM_FORMAT_U18_3BE: snd_pcm_format_t = 43;
pub(crate) const SND_PCM_FORMAT_G723_24: snd_pcm_format_t = 44;
pub(crate) const SND_PCM_FORMAT_G723_24_1B: snd_pcm_format_t = 45;
pub(crate) const SND_PCM_FORMAT_G723_40: snd_pcm_format_t = 46;
pub(crate) const SND_PCM_FORMAT_G723_40_1B: snd_pcm_format_t = 47;
pub(crate) const SND_PCM_FORMAT_DSD_U8: snd_pcm_format_t = 48;
pub(crate) const SND_PCM_FORMAT_DSD_U16_LE: snd_pcm_format_t = 49;
pub(crate) const SND_PCM_FORMAT_DSD_U32_LE: snd_pcm_format_t = 50;
pub(crate) const SND_PCM_FORMAT_DSD_U16_BE: snd_pcm_format_t = 51;
pub(crate) const SND_PCM_FORMAT_DSD_U32_BE: snd_pcm_format_t = 52;
pub(crate) const SND_PCM_FORMAT_LAST: snd_pcm_format_t = 52;
pub(crate) const SND_PCM_FORMAT_S16: snd_pcm_format_t = 2;
pub(crate) const SND_PCM_FORMAT_U16: snd_pcm_format_t = 4;
pub(crate) const SND_PCM_FORMAT_S24: snd_pcm_format_t = 6;
pub(crate) const SND_PCM_FORMAT_U24: snd_pcm_format_t = 8;
pub(crate) const SND_PCM_FORMAT_S32: snd_pcm_format_t = 10;
pub(crate) const SND_PCM_FORMAT_U32: snd_pcm_format_t = 12;
pub(crate) const SND_PCM_FORMAT_FLOAT: snd_pcm_format_t = 14;
pub(crate) const SND_PCM_FORMAT_FLOAT64: snd_pcm_format_t = 16;
pub(crate) const SND_PCM_FORMAT_IEC958_SUBFRAME: snd_pcm_format_t = 18;
pub(crate) const SND_PCM_FORMAT_S20: snd_pcm_format_t = 25;
pub(crate) const SND_PCM_FORMAT_U20: snd_pcm_format_t = 27;
pub(crate) type snd_pcm_format_t = c_int;

#[rustfmt::skip]
#[cfg(ffi_alsa··)]
#[link(name = "asound")]
unsafe extern "C" {
    /// Open PCM.
    pub(crate) fn snd_pcm_open(pcm: *mut *mut snd_pcm_t, name: *const c_char, stream: c_int,
        mode: c_int) -> c_int;
    pub(crate) fn snd_pcm_close(pcm: *mut snd_pcm_t) -> c_int;
    pub(crate) fn snd_device_name_hint(card: c_int, iface: *const c_char,
        hints: *mut *mut *mut c_char) -> c_int;
    pub(crate) fn snd_device_name_free_hint(hints: *mut *mut c_char) -> c_int;
    pub(crate) fn snd_device_name_get_hint(hint: *mut c_char, id: *const c_char) -> *mut c_char;
    pub(crate) fn snd_pcm_drain(pcm: *mut snd_pcm_t) -> c_int;
    pub(crate) fn snd_pcm_drop(pcm: *mut snd_pcm_t) -> c_int;
    /// returns number of frames written.
    pub(crate) fn snd_pcm_writei(pcm: *mut snd_pcm_t, buffer: *const c_void,
        size: snd_pcm_uframes_t) -> snd_pcm_sframes_t;
    /// returns number of frames read.
    pub(crate) fn snd_pcm_readi(pcm: *mut snd_pcm_t, buffer: *mut c_void,
        size: snd_pcm_uframes_t) -> snd_pcm_sframes_t;
    pub(crate) fn snd_pcm_prepare(pcm: *mut snd_pcm_t) -> c_int;
    pub(crate) fn snd_pcm_recover(pcm: *mut snd_pcm_t, err: c_int, silent: c_int) -> c_int;

    pub(crate) fn snd_pcm_hw_params_malloc(ptr: *mut *mut snd_pcm_hw_params_t) -> c_int;
    pub(crate) fn snd_pcm_hw_params_any( pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t)
        -> c_int;
    pub(crate) fn snd_pcm_hw_params_set_access(pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t, _access: snd_pcm_access_t) -> c_int;
    pub(crate) fn snd_pcm_hw_params_set_format(pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t, val: snd_pcm_format_t) -> c_int;
    pub(crate) fn snd_pcm_hw_params_set_rate_near(pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t, val: *mut c_uint, dir: *mut c_int) -> c_int;
    pub(crate) fn snd_pcm_hw_params_set_channels(pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t, val: c_uint) -> c_int;
    pub(crate) fn snd_pcm_hw_params_set_buffer_size_near(pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t, val: *mut snd_pcm_uframes_t) -> c_int;
    pub(crate) fn snd_pcm_hw_params(pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t)
        -> c_int;
    pub(crate) fn snd_pcm_hw_params_free(obj: *mut snd_pcm_hw_params_t);
}
