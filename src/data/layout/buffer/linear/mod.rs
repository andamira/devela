// devela::data::layout::buffer::linear
//
//! Defines linear buffers.
//!
//! > A semantic machine that overlays occupancy semantics over contiguous storage.
//

#[cfg(test)]
mod tests;
#[cfg(any(test, feature = "_docs_examples"))]
mod examples; // BufferLinearStaticExample, BufferLinearViewExample, BufferLinearAllocExample

mod definition; // buffer_linear!
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
