// devela/src/media/audio/format/wav/mod.rs
//
//! RIFF/WAVE support for PCM-family audio.
//!
//! This module parses and writes simple WAVE containers around raw PCM-family
//! payloads. It supports classic PCM, IEEE float, and WAVE_FORMAT_EXTENSIBLE
//! metadata for PCM/float subformats.
//!
//! The audio payload remains raw interleaved bytes.
//! Use [`PcmRaw`] or [`PcmRawBuf`] to materialize typed samples explicitly.
//

#[cfg(test)]
mod _test;

mod buf; // PcmWavBuf
mod error; // PcmWavError
mod fmt; // PcmWavFmt
mod namespace; // PcmWav

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
