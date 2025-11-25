// devela::media::audio
//
//! Audio functionality.
//
// safety
#![cfg_attr(feature = "safe_audio", forbid(unsafe_code))]

// mod drum_machine;

crate::structural_mods! { // _mods
    _mods {
        // pub use super::drum_machine::*;

        // re-exports
        pub use devela_base_core::media::audio::{
            AudioChannel, AudioChannels,
        };
    }
}
