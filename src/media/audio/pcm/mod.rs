// devela::media::audio::pcm
//
//! Pulse-code modulation audio buffers and stream metadata.
//

#[cfg(test)]
mod tests;

mod buffer; // PcmBuffer
mod sample; // PcmSample
mod spec; // PcmSpec

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            buffer::*,
            sample::*,
            spec::*,
        };
    }
}
