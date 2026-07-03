// devela/src/data/codec/pack/wrap/riff/mod.rs
//
//! Resource Interchange File Format.
//

#[cfg(test)]
mod _test;

mod chunk; // RiffChunk, RiffChunkIter
mod error; // RiffError
mod namespace; // Riff

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            chunk::*,
            error::*,
            namespace::*,
        };
    }
}
