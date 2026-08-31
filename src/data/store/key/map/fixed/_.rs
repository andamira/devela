// devela/src/data/store/key/map/fixed/_.rs
//
//! Fixed-capacity open-addressed hash maps with mutable entries.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;
    #[cfg(any(test, feature = "_docs_examples"))]
    mod _example;

    mod define; // map!
    mod_ impls; // hidden macros for map! variants
    mod entry; // MapFixedEntry
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            define::map,
            entry::MapFixedEntry,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
    _hidden {
        pub use super::impls::_hidden::*;
    }
}
