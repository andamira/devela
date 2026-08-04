// devela/src/data/id/handle/mod.rs
//
//!
//

mod generation; // handle_gen!
mod span; // handle_span!

#[cfg(any(test, feature = "_docs_examples"))]
mod _example; // HandleSpanExample

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            generation::handle_gen,
            span::handle_span,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
}
