// devela::media::audio
//
#![doc = crate::_DOC_MEDIA_AUDIO!()] // public
#![doc = crate::_doc!(modules: crate::media; audio)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_audio", forbid(unsafe_code))]

// mod codec; // …
// mod drum_machine;
mod layout; // AudioChannel, AudioChannels
// mod music; //
// mod pcm; // WIP: PcmBuffer, PcmPlanar, PcmRaw, PcmWav, PcmSample, PcmSpec, …
// mod spatial; // …
// mod synth; // …

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // codec::*,
            // drum_machine::*,
            layout::*,
            // music::*,
            // pcm::*,
            // spatial::*,
            // synth::*,
        };
    }
}
