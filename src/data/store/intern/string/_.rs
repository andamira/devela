// devela/src/data/store/intern/string/_.rs
//
//!
//

crate::mods_in! {
    #[cfg(any(test, doctest))]
    mod _test;
    #[cfg(any(test, doctest, feature = "_docs_examples"))]
    mod _example;

    mod define; // intern_string!
    mod _internal; // __intern_string!
    mod_ impls;
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::define::intern_string;
        #[cfg(any(test, doctest, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
    _hidden {
        pub use super::{
            _internal::__intern_string,
            impls::_hidden::*,
        };
    }
}
