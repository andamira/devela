// devela/src/data/layout/buffer/ring/mod.rs
//
//! Ring buffers.
//

#[cfg(test)]
mod tests;
#[cfg(any(test, feature = "_docs_examples"))]
mod examples; // BufferRingStaticExample

mod definition; // buffer_ring!
mod impls; // hidden macros for buffer variants

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            definition::*,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::examples::*;
    }
}
