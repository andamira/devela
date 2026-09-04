// devela/src/data/layout/buffer/ring/_.rs
//
//! Ring buffers.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;
    #[cfg(any(test, feature = "_docs_examples"))]
    mod _example; // BufferRingStaticExample

    mod define; // buffer_ring!, BufferRingU8
    mod_ impls; // hidden macros for buffer variants
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            define::{BufferRingU8, buffer_ring},
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
