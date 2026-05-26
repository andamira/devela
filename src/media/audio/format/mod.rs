// devela::media::audio::format
//
#![doc = crate::_DOC_MEDIA_AUDIO_FORMAT!()] // public
#![doc = crate::_doc!(modules: crate::media::audio; format)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]

// mod aiff; // PcmAiff
// mod flac;
// mod mp3; // Mp3Decoder
// mod mulaw; // MuLaw
// mod ogg;
mod raw; // Headerless raw PCM audio.
mod wav; // Waveform Audio File Format

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // mp3::*,
            // mulaw::*,
            raw::*,
            wav::*,
        };
    }
}
