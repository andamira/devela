// devela/src/data/codec/pack/wrap/mod.rs
//
//! Structured wrappers and chunked containers.
//

mod riff; // Resource Interchange File Format
// mod png_chunk;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            riff::_all::*,
            // png_chunk::*,
        };
    }
}
