// devela/src/data/codec/pack/wrap/_.rs
//
//! Structured wrappers and chunked containers.
//

crate::mods_in! {
    mod_ riff; // Resource Interchange File Format
    // mod png_chunk; // MAYBE
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            riff::_all::*,
            // png_chunk::*,
        };
    }
}
