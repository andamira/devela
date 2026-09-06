// devela/src/data/store/arena/string/_.rs
//
//!
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;
    #[cfg(any(test, feature = "_docs_examples"))]
    mod _example;

    mod define; // arena_string!
    mod _internal; // __arena_string!
    mod_ impls; // hidden macros for arena_string variants
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            define::arena_string,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
    _hidden {
        pub use super::{
            _internal::__arena_string,
            impls::_hidden::*,
        };
    }
}
