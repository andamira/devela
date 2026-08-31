// devela/src/data/codec/pack/wrap/riff/_.rs
//
//! Resource Interchange File Format.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod chunk; // RiffChunk, RiffChunkIter
    mod error; // RiffError
    mod namespace; // Riff
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            chunk::*,
            error::*,
            namespace::*,
        };
    }
}
