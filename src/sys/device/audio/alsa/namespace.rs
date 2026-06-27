// devela/src/sys/device/audio/alsa/namespace.rs
//
//! Defines [`Alsa`].
//

use super::_raw;
use crate::{AlsaError, AlsaPcmHandle, AudioDevice, AudioDeviceDir, AudioStreamDir};
#[cfg(ffi_alsa··)]
use crate::{AlsaHintList, AlsaHintValue};
#[cfg(all(ffi_alsa··, feature = "alloc"))]
use crate::{AudioDeviceCow, Cow, ToString, Vec};
use crate::{CStr, Ptr};

#[doc = crate::_tags!(audio linux namespace)]
/// ALSA operations.
#[doc = crate::_doc_meta!{location("sys/device/audio")}]
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
    /// Returns `Ok` if native ALSA support was compiled in.
    pub const fn require_available() -> Result<(), AlsaError> {
        if Self::is_available() { Ok(()) } else { Err(AlsaError::Unavailable) }
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

// Public ALSA surface preserved when the native backend is unavailable.
#[cfg(not(ffi_alsa··))]
crate::items! {
    macro_rules! _unavailable { () => { Err(AlsaError::Unavailable) }; }
    #[allow(missing_docs, unused_variables)]
    impl Alsa {
        pub fn open_pcm(id: &CStr, dir: AudioStreamDir)
            -> Result<AlsaPcmHandle, AlsaError> { _unavailable!() }
        pub fn open_default_playback() -> Result<AlsaPcmHandle, AlsaError> { _unavailable!() }
        pub fn open_default_capture() -> Result<AlsaPcmHandle, AlsaError> { _unavailable!() }
        pub fn for_each_pcm_device(f: impl FnMut(AudioDevice<'_>) -> Result<(), AlsaError>)
            -> Result<(), AlsaError> { _unavailable!() }
    }
}
