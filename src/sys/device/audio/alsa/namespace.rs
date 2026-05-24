// devela::sys::device::audio::alsa::namespace
//
//! Defines [`Alsa`], [`AlsaError`].
//

use super::_raw;
#[cfg(ffi_alsa··)]
use crate::{AlsaHintList, AlsaHintValue};
use crate::{AlsaPcmHandle, AudioDevice, AudioDeviceDir, AudioStreamDir};
#[cfg(all(ffi_alsa··, feature = "alloc"))]
use crate::{AudioDeviceCow, Cow, ToString, Vec};
use crate::{CStr, Ptr, c_int};

#[doc = crate::_tags!(audio linux namespace)]
/// ALSA operations.
#[doc = crate::_doc_location!("sys/device/audio")]
///
/// # Availability
/// Methods that call ALSA are available
/// when the native `asound` library is discoverable at build time.
#[derive(Debug)]
pub struct Alsa;
impl Alsa {
    /// Returns whether the native ALSA library was found at build time.
    pub const fn is_available() -> bool {
        cfg!(ffi_alsa··)
    }
}
#[cfg(ffi_alsa··)]
impl Alsa {
    /// Opens an ALSA PCM device.
    pub fn open_pcm(id: &CStr, dir: AudioStreamDir) -> Result<AlsaPcmHandle, AlsaError> {
        unsafe {
            let mut handle = Ptr::null_mut();
            AlsaError::result(_raw::snd_pcm_open(&mut handle, id.as_ptr(), dir.to_alsa() as _, 0))?;
            Ok(AlsaPcmHandle::from_raw(handle))
        }
    }
    /// Opens the default playback device.
    pub fn open_default_playback() -> Result<AlsaPcmHandle, AlsaError> {
        Self::open_pcm(c"default", AudioStreamDir::Playback)
    }
    /// Opens the default capture device.
    pub fn open_default_capture() -> Result<AlsaPcmHandle, AlsaError> {
        Self::open_pcm(c"default", AudioStreamDir::Capture)
    }

    /// Returns the ALSA PCM device hints as owned device descriptions.
    ///
    /// This is the allocating counterpart to [`for_each_pcm_device`](Self::for_each_pcm_device).
    /// It copies ALSA's temporary hint strings into Rust-owned storage before
    /// returning.
    ///
    /// The returned device `id` can be passed back to [`open_pcm`](Self::open_pcm).
    ///
    /// # Notes
    /// ALSA usually does not expose a separate human-readable display name through
    /// this hint API, so `name` is commonly `None`. Use [`AudioDevice::label`] or
    /// the device description when presenting devices to users.
    ///
    /// # Errors
    /// Returns [`AlsaError`] if ALSA fails to enumerate device hints.
    #[cfg(feature = "alloc")]
    pub fn pcm_devices() -> Result<Vec<AudioDeviceCow<'static>>, AlsaError> {
        let mut out = Vec::new();
        Self::for_each_pcm_device(|dev| {
            out.push(AudioDeviceCow {
                id: Cow::Owned(dev.id.to_string()),
                name: dev.name.map(|s| Cow::Owned(s.to_string())),
                desc: dev.desc.map(|s| Cow::Owned(s.to_string())),
                dir: dev.dir,
            });
            Ok(())
        })?;
        Ok(out)
    }

    /// Calls `f` for each ALSA PCM device hint.
    ///
    /// The callback receives a backend-neutral [`AudioDevice`] whose `id`
    /// can be passed back to [`open_pcm`](Self::open_pcm).
    ///
    /// This method does not allocate Rust-owned storage.
    /// ALSA still allocates temporary hint strings internally,
    /// which are released before the method returns.
    ///
    /// # Notes
    /// [`AudioDevice::name`] is commonly `None` for ALSA devices.
    ///
    /// # Errors
    /// Returns [`AlsaError`] if ALSA fails to enumerate device hints
    /// or if the callback returns an error.
    pub fn for_each_pcm_device(
        mut f: impl FnMut(AudioDevice<'_>) -> Result<(), AlsaError>,
    ) -> Result<(), AlsaError> {
        unsafe {
            let mut hints = Ptr::null_mut();
            AlsaError::result(_raw::snd_device_name_hint(-1, c"pcm".as_ptr(), &mut hints))?;
            let hints = AlsaHintList { raw: hints };
            let mut p = hints.raw;
            while !p.is_null() && !(*p).is_null() {
                let hint = *p;
                let name = AlsaHintValue::get(hint, c"NAME");
                let desc = AlsaHintValue::get(hint, c"DESC");
                let ioid = AlsaHintValue::get(hint, c"IOID");
                if let Some(id) = name.as_str() {
                    f(AudioDevice {
                        id,
                        name: None,
                        desc: desc.as_str(),
                        dir: AudioDeviceDir::from_alsa_ioid(ioid.as_cstr()),
                    })?;
                }
                p = p.add(1);
            }
            Ok(())
        }
    }
}

#[doc = crate::_tags!(audio linux error)]
/// ALSA operation error.
#[doc = crate::_doc_location!("sys/device/audio")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AlsaError {
    /// Negative ALSA/libc-style error code.
    pub code: c_int,
}
#[rustfmt::skip]
impl AlsaError {
    /// Invalid argument.
    ///
    /// Matches the usual negative `EINVAL` value used by ALSA/libc-style APIs.
    pub const INVALID_ARGUMENT: c_int = -22;

    /// Creates a new ALSA error from a negative error code.
    pub const fn new(code: c_int) -> Self {
        Self { code }
    }

    /// Creates an invalid-argument error.
    pub const fn invalid_argument() -> Self {
        Self::new(Self::INVALID_ARGUMENT)
    }

    #[inline(always)]
    pub(crate) const fn result(code: c_int) -> Result<(), AlsaError> {
        if code < 0 { Err(AlsaError::new(code)) } else { Ok(()) }
    }
}
