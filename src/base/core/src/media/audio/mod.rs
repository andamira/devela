// devela_base_core::media::audio
//
//! Audio functionality.
//
// safety
// #![cfg_attr(base_safe_audio, forbid(unsafe_code))] // no feature

// mod codec; // …
mod layout; // AudioChannel, AudioChannels
// mod pcm; // PcmBuffer, PcmPlanar, PcmRaw, PcmWav, PcmSample, PcmSpec, …
// mod spatial; // …
// mod synth; // …

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // codec::*,
            layout::*,
            // pcm::*,
            // spatial::*,
            // synth::*,
        };
    }
}
