// devela::media::audio::pcm::format
//
//!
//

// mod aiff; // PcmAiff
// mod raw; // PcmRaw
mod wav; // PcmWav

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // aiff::*,
            // raw::*,
            wav::*,
        };
    }
}
