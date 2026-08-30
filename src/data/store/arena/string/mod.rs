// devela/src/data/store/arena/string/mod.rs
//
//!
//

#[cfg(test)]
mod _test;
#[cfg(any(test, feature = "_docs_examples"))]
mod _example;

mod define; // arena_string!
mod impls; // hidden macros for arena_string variants

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::arena_string,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
}
