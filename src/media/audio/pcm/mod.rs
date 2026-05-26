// devela::media::audio::pcm
//
#![doc = crate::_DOC_MEDIA_AUDIO_PCM!()] // public
#![doc = crate::_doc!(modules: crate::media::audio; pcm)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod tests;

mod buf; // PcmBuf
mod layout; // PcmLayout
mod sample; // PcmSample, PcmSampleType
mod spec; // PcmSpec

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            buf::*,
            layout::*,
            sample::*,
            spec::*,
        };
    }
}
