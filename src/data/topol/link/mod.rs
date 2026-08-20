// devela/src/data/topol/link/mod.rs
//
#![doc = crate::_DOC_DATA_TOPOL_LINK!()] // public
#![doc = crate::_doc!(modules: crate::data::topol; link)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//!
//! Links represent fixed-arity named relations to externally interpreted targets.
//!
//! Each link is independently optional and addresses at most one target.
//! Links do not own values, establish target lifetime,
//! or impose relationships between different fields.
//!
//! [`link!`] generates compact link records with arbitrary named dimensions.
//! Higher-level topologies may compose these links into linear,
//! hierarchical, or other constrained relations.
//

#[cfg(test)]
mod _test;
#[cfg(any(test, feature = "_docs_examples"))]
mod _example;

mod define; // link!

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::define::link;

        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
}
