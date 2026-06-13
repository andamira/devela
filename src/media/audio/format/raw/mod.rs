// devela/src/media/audio/format/raw/mod.rs
//
//! Headerless raw PCM audio.
//

#[cfg(test)]
mod tests;

mod buf; // PcmRawBuf
mod error; // PcmRawError
mod namespace; // PcmRaw

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            buf::*,
            error::*,
            namespace::*,
        };
    }
}
