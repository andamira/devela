// devela/src/data/layout/buffer/linear/mod.rs
//
//! Defines linear buffers.
//!
//! > A semantic machine that overlays occupancy semantics over contiguous storage.
//

#[cfg(test)]
mod _test;
#[cfg(any(test, feature = "_docs_examples"))]
mod _example; // BufferLinearStaticExample, BufferLinearViewExample, BufferLinearAllocExample

mod define; // buffer_linear!
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
