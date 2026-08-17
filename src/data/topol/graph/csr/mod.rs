// devela/src/data/topol/graph/csr/mod.rs
//
//!
//

#[cfg(any(test, doctest))]
mod _test;
#[cfg(any(test, doctest, feature = "_docs_examples"))]
mod _example;

mod define; // graph_csr!
mod impls;

crate::structural_mods! { // _mods
    _mods {
        pub use super::define::graph_csr;
        #[cfg(any(test, doctest, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
}
