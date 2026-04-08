// devela::data::layout::buffer
//
//!
//

#[cfg(feature = "alloc")]
#[cfg(any(test, feature = "_docs_examples"))]
mod examples_alloc; // BufferAllocExample

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[cfg(feature = "alloc")]
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::examples_alloc::BufferAllocExample;
    }
    _reexports {
        #[doc(inline)]
        pub use devela_base_core::data::layout::buffer_linear;

        #[cfg(feature = "_docs_examples")]
        pub use devela_base_core::data::layout::{
            BufferStaticExample, BufferViewExample,
        };
    }
}
