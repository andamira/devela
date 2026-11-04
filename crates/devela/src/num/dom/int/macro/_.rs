//
//! Integer type generation: public grammar, private emitters, and examples.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;
    #[cfg(any(test, feature = "_docs_examples"))]
    mod _example; // IntExample

    mod define; // int!
    mod _internal; // __int!
    mod base_alg; // __int_base_alg!
    mod_ impls; // operation-family helpers
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::define::int;
        #[cfg(feature = "_docs_examples")]
        pub use super::_example::IntExample;
    }
    _hidden {
        #[doc(hidden)]
        pub use super::{
            _internal::__int,
            base_alg::__int_base_alg,
            impls::_hidden::__int_impl_base,
        };
    }
}
