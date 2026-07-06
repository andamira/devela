// devela/src/sys/device/audio/mod.rs
//
#![doc = crate::_DOC_SYS_DEVICE_AUDIO!()] // public
#![doc = crate::_doc!(modules: crate::sys::device; audio)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//
// Access to hardware endpoints for input and output,
// including backend bindings for ALSA, JACK, Pulse,
// and platform-specific audio drivers.
//

mod common; // AudioDevice, AudioDeviceDir

#[cfg(all(feature = "alsa", not(feature = "safe_sys")))]
mod alsa;
// mod pulse;

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::common::*;
        #[cfg(all(feature = "alsa", not(feature = "safe_sys")))]
        pub use super::alsa::_all::*;
        // pub use super::pulse::*;
    }
    _crate_internals {
        #[cfg(all(feature = "alsa", not(feature = "safe_sys")))]
        pub(crate) use super::alsa::_crate_internals::*;
    }
}
