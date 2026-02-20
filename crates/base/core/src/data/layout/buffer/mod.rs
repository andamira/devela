// devela_base_core::data::layout::buffer
//
//!
//

mod linear; // buffer_linear!

mod impls; // hidden macros for buffer variants

#[cfg(any(test, feature = "_docs_examples"))]
mod examples; // BufferStaticExample, BufferViewExample

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            linear::*,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::examples::*;
    }
}
