// devela/src/data/topol/graph/adj/mod.rs
//
//!
//

#[cfg(test)]
mod _test;
#[cfg(any(test, feature = "_docs_examples"))]
mod _example;

mod define; // graph_adj!
mod impls;

crate::structural_mods! { // _mods
    _mods {
        pub use super::define::graph_adj;
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
}
