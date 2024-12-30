// devela::data::collections
//
//! Data collections.
//!
#![doc = crate::doc_!(extends: array, collections, vec)]
//

mod reexports;

mod array;
mod list;

#[cfg(_destaque_·)]
mod destaque;
// #[cfg(_graph_·)]
// mod graph;
// #[cfg(_node_·)]
// mod node;
#[cfg(_stack_·)]
mod stack;
mod traits;
#[cfg(feature = "_tuple")]
mod tuple; // Tuple, TupleFmt, TupleEnumRef, TupleEnumMut
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
mod vec;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{array::_all::*, list::_all::*, reexports::*};

        #[cfg(_destaque_·)]
        pub use super::destaque::_all::*;
        // #[cfg(_graph_·)]
        // pub use super::graph::*;
        // #[cfg(_node_·)]
        // pub use super::node::*;
        #[cfg(_stack_·)]
        pub use super::stack::_all::*;
        pub use super::traits::_all::*;
        #[cfg(feature = "_tuple")]
        pub use super::tuple::_all::*;
        #[cfg(feature = "alloc")]
        pub use super::vec::_all::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
