// devela_base_alloc::data::layout::buffer
//
//!
//

#[cfg(any(test, feature = "_docs_examples"))]
mod examples; // BufferAllocExample

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::examples::*;
    }
    _reexports {
        pub use devela_base_core::data::layout::{
            buffer_linear,
        };
        #[cfg(feature = "_docs_examples")]
        pub use devela_base_core::data::layout::{
            BufferStaticExample, BufferViewExample,
        };
    }
}
