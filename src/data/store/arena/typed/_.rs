// devela/src/data/store/arena/typed/mod.rs
//
//! Typed monotonic arenas.
//

crate::mods_in! {
    #[cfg(any(test, doctest))]
    mod _test;
    #[cfg(any(test, doctest, feature = "_docs_examples"))]
    mod _example;

    mod define; // arena!
    mod_ impls; // hidden macros for arena variants
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::define::arena;
        #[cfg(any(test, doctest, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
    _hidden {
        pub use super::impls::_hidden::*;
    }
}
