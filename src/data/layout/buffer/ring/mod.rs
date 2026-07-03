// devela/src/data/layout/buffer/ring/mod.rs
//
//! Ring buffers.
//

#[cfg(test)]
mod _test;
#[cfg(any(test, feature = "_docs_examples"))]
mod _example; // BufferRingStaticExample

mod define; // buffer_ring!, BufferRingU8
mod impls; // hidden macros for buffer variants

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
}
