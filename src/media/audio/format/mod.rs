// devela::media::audio::codec
//
#![doc = crate::_tags!(audio codec)]
#![doc = crate::_DOC_MEDIA_AUDIO_CODEC!()]
#![doc = crate::_doc!(modules: crate::media::audio; codec)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]

// mod aiff; // PcmAiff
// mod flac;
// mod mp3; // Mp3Decoder
// mod mulaw; // MuLaw
// mod ogg;
// mod raw; // PcmRaw
mod wav; // PcmWav

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
