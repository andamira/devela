// devela/src/data/topol/graph/csr/_.rs
//
//!
//

crate::mods_in! {
    #[cfg(any(test, doctest))]
    mod _test;
    #[cfg(any(test, doctest, feature = "_docs_examples"))]
    mod _example;

    mod define; // graph_csr!
    mod_ impls;
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::define::graph_csr;
        #[cfg(any(test, doctest, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
    _hidden {
        pub use super::impls::_hidden::*;
    }
}
