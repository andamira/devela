// devela::media::audio::format::wav
//
//! Minimal RIFF/WAVE parsing for PCM-family audio.
//

#[cfg(test)]
mod tests;

mod buf; // PcmWavBuf
mod error; // PcmWavError
mod fmt; // PcmWavFmt
mod namespace; // PcmWav, PcmWavFmt

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            buf::*,
            error::*,
            fmt::*,
            namespace::*,
        };
    }
}
