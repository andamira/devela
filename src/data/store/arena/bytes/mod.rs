// devela/src/data/store/arena/bytes/mod.rs
//
//!
//

#[cfg(test)]
mod _test;
#[cfg(any(test, feature = "_docs_examples"))]
mod _example;

mod define; // arena_bytes!
mod _internal; // __ArenaBytesArray
mod impls; // hidden macros for arena variants
// mod field; // WIP ArenaField
// mod primitive; // WIP ArenaPrimitive
// mod recipe; // WIP ArenaRecipe

crate::structural_mods! { // _mods, _hidden
    _mods {
        pub use super::{
            define::*,
            // field::*,
            // primitive::*,
            // recipe::*,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::{
            ArenaBytesExample, ArenaBytesHandleExample, ArenaBytesMarkExample,
        };
    }
    _hidden {
        pub use super::_internal::*;
    }
}
