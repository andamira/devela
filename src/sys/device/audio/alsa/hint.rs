// devela/src/sys/device/audio/alsa/hint.rs

use super::_raw;
use crate::{CStr, Libc, c_char};

pub(crate) struct AlsaHintList {
    pub(crate) raw: *mut *mut c_char,
}
impl Drop for AlsaHintList {
    fn drop(&mut self) {
        unsafe {
            let _ = _raw::snd_device_name_free_hint(self.raw);
        }
    }
}

pub(crate) struct AlsaHintValue {
    pub(crate) raw: *mut c_char,
}
impl AlsaHintValue {
    pub(crate) unsafe fn get(hint: *mut c_char, key: &CStr) -> Self {
        Self {
            raw: unsafe { _raw::snd_device_name_get_hint(hint, key.as_ptr()) },
        }
    }
    pub(crate) fn as_cstr(&self) -> Option<&CStr> {
        if self.raw.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(self.raw) })
        }
    }
    pub(crate) fn as_str(&self) -> Option<&str> {
        self.as_cstr()?.to_str().ok()
    }
}
impl Drop for AlsaHintValue {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                Libc::free(self.raw.cast());
            }
        }
    }
}
