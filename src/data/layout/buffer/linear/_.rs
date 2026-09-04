// devela/src/data/layout/buffer/linear/_.rs
//
//! Defines linear buffers.
//!
//! > A semantic machine that overlays occupancy semantics over contiguous storage.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;
    #[cfg(any(test, feature = "_docs_examples"))]
    mod _example; // BufferLinearStaticExample, BufferLinearViewExample, BufferLinearAllocExample

    mod define; // buffer_linear!
    mod_ impls; // hidden macros for buffer variants
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            define::buffer_linear,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
    _hidden {
        pub use super::{
            impls::_hidden::*,
        };
    }
}
