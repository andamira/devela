// devela::data::codec::pack::wrap::riff
//
//! Resource Interchange File Format.
//

#[cfg(test)]
mod tests;

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
