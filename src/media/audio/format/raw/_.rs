// devela/src/media/audio/format/raw/_.rs
//
//! Headerless raw PCM audio.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod buf; // PcmRawBuf
    mod error; // PcmRawError
    mod namespace; // PcmRaw
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            buf::*,
            error::*,
            namespace::*,
        };
    }
}
