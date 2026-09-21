//
//!
//

crate::mods_in! {
    #[cfg(any(test, feature = "_docs_examples"))]
    mod _test_example; // DivisorExample

    mod define; // divisor!, DivisorExample
    mod _internal; // __Divisor, __divisor!
}
crate::mods_out! { // _mods, _crate_internals, _hidden
    _mods {
        pub use super::define::divisor;
        #[cfg(feature = "_docs_examples")]
        pub use super::_test_example::DivisorExample;
    }
    _hidden {
        pub use super::_internal::{__Divisor, __divisor};
    }
}
