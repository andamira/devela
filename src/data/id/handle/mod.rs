// devela::data::id::handle
//
//!
//

// mod define; // WIP TEMP handle2!
mod legacy; // OLD handle! (Span)

#[cfg(any(test, feature = "_docs_examples"))]
mod examples;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            // define::*,
            legacy::*,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::examples::*;
    }
}
