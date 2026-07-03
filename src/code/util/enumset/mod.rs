// devela/src/code/util/enumset/mod.rs
//
//! An enum with an associated bit set.
//

#[cfg(test)]
mod _test;
#[cfg(any(test, feature = "_docs_examples"))]
mod _example; // EnumExample, EnumSetExample

mod define; // enumset!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::enumset,
        };
        #[cfg(feature = "_docs_examples")]
        pub use super::_example::*;
    }
}
