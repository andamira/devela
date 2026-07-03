// devela/src/data/codec/bin/bit/field/mod.rs

#[cfg(any(test, doctest))]
mod _test;

#[cfg(any(test, feature = "_docs_examples"))]
mod _example; // BitfieldExample

mod define; // bitfield!

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            define::bitfield,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
}
