// devela/src/media/audio/_.rs
//
#![doc = crate::_DOC_MEDIA_AUDIO!()] // public
#![doc = crate::_doc!(modules: crate::media; audio: format, pcm)] // acoustic, music, synth
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_audio", forbid(unsafe_code))]

crate::mods_in! {
        // mod_acoustic; // Acoustic spaces, sources, listeners, propagation, effects
        // mod_effect; // Signal filters, dynamics, delays, and transformations
    pub mod_ format; // Encoded audio representations and containers
        // mod_ instrument; // Instruments, sample maps, presets, and sound banks
        mod layout; // Channel roles, arrangements, and sample layouts
    // mod_ music; // Theory, tuning, notation, harmony, performance data
    pub mod_ pcm; // PCM samples, buffers, specs, and stream metadata
    // mod_ synth; // Oscillators, envelopes, voices, modulation
    // mod time; // Audio time scales, modes, perceptual anchors
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // acoustic::_all::*,
            // effect::_all::*,
            layout::{AudioChannel, AudioChannels},
            // music::_all::*,
            // synth::_all::*,
            // time::*,
        };
    }
    _pub_mods {
        pub use super::{
            format::_all::*,
            pcm::_all::*,
        };
    }
}
