// devela::media::audio
//
//! Audio functionality.
#![doc = crate::_doc!(lf)]
#![doc = crate::_doc!(modules: crate::media; audio)]
#![doc = crate::_doc!(flat:"media")]
//
// safety
#![cfg_attr(feature = "safe_audio", forbid(unsafe_code))]

// mod drum_machine;

crate::structural_mods! { // _mods, _reexports
    _mods {
        // pub use super::drum_machine::*;
    }
    _reexports {
        pub use devela_base_core::media::audio::{
            AudioChannel, AudioChannels,
            // PcmSample, PcmSpec, PcmRaw, PcmWav, PcmBuffer, PcmPlanar, WIP
        };
    }
}
