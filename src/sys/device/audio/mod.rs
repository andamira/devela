// devela::sys::device::audio
//
#![doc = crate::_DOC_DEVICE_AUDIO!()] // public
#![doc = crate::_doc!(modules: crate::sys::device; audio)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//
// Access to hardware endpoints for input and output,
// including backend bindings for ALSA, JACK, Pulse,
// and platform-specific audio drivers.
//

// TEMP std (println), alloc (String)
// #[cfg(all(feature = "audio", feature = "unsafe_syscall", feature = "std"))]
// mod alsa;
// mod pulse;

crate::structural_mods! { // _mods
    _mods {
        // #[cfg(all(feature = "audio", feature = "unsafe_syscall", feature = "std"))]
        // pub use super::alsa::*;
        // pub use super::pulse::*;
    }
}
