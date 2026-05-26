// devela::media::audio::pcm
//
//! Pulse-code modulation audio buffers and stream metadata.
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
