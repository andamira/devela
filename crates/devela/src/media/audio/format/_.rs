//
#![doc = crate::_DOC_MEDIA_AUDIO_FORMAT!()] // public
#![doc = crate::_doc!(modules: crate::media::audio; format)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]

crate::mods_in! {
    // mod_ aiff; // PcmAiff
    // mod_ flac;
    // mod_ mp3; // Mp3Decoder
    // mod_ mulaw; // MuLaw
    // mod_ ogg;
    #[cfg(feature = "pcm")]
    mod_ raw; // Headerless raw PCM audio.
    #[cfg(feature = "wav")]
    mod_ wav; // Waveform Audio File Format
}
crate::mods_out! { // _mods
    _mods {
        // pub use super::mp3::_all::*;
        // pub use super::mulaw::_all::*;
        #[cfg(feature = "pcm")]
        pub use super::raw::_all::*;
        #[cfg(feature = "wav")]
        pub use super::wav::_all::*;
    }
}
