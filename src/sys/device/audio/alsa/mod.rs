// devela::sys::device::audio::alsa
//
// #![doc = crate::_DOC_SYS_DEVICE_AUDIO!()]
//! Advanced Linux Sound Architecture (ALSA).
#![doc = crate::_doc!(modules: crate::sys::device; audio)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//
#![cfg_attr(not(ffi_alsa··), allow(dead_code, unused_imports))]

#[cfg(test)]
mod tests;

pub(crate) mod _raw; //

#[cfg(ffi_alsa··)]
mod hint;

mod namespace; // Alsa, AlsaError
mod pcm; // AlsaPcmHandle

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            namespace::*,
            pcm::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_raw as _alsa_raw;
        #[cfg(ffi_alsa··)]
        pub(crate) use super::hint::*;
    }
}
