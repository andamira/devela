// devela::media::audio::format::wav
//
//! Minimal RIFF/WAVE parsing for PCM-family audio.
//

#[cfg(test)]
mod tests;

mod error; // PcmWavError
mod namespace; // PcmWav, PcmWavFmt
mod refalloc; // PcmWavRef, PcmWavAlloc

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            error::*,
            refalloc::*,
            namespace::*,
        };
    }
}
