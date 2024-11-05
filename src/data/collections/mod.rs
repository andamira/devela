// devela::data::collections
//
//! Data collections.
#![doc = crate::doc_!(extends: array, collections, vec)]
// #![doc = crate::doc_!(modules: crate::data; collections)]
#![doc = crate::doc_!(newline)]
//!
//

#[allow(unused_imports)]
use crate::code::items;

mod array;
mod list;
mod reexports;
mod traits;
#[allow(unused_imports)]
pub use {array::all::*, list::all::*, reexports::*, traits::*};

#[cfg(_some_destaque)]
items! { mod destaque; pub use destaque::*; }

// #[cfg(_some_graph)]
// items! { mod graph; pub use graph::*; }

// #[cfg(_some_node)]
// items! { mod node; pub use node::*; }

#[cfg(_some_stack)]
items! { mod stack; pub use stack::*; }

#[cfg(feature = "_tuple")]
items! { mod tuple; pub use tuple::*; } // Tuple, TupleFmt

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
items! {
    mod vec;
    #[allow(unused_imports)]
    pub use vec::*;
}

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{array::all::*, list::all::*, reexports::*, traits::*};

    #[cfg(_some_destaque)]
    pub use super::destaque::all::*;

    // #[cfg(_some_graph)]
    // pub use super::graph::*;

    // #[cfg(_some_node)]
    // pub use super::node::*;

    #[cfg(_some_stack)]
    pub use super::stack::all::*;

    #[cfg(feature = "_tuple")]
    pub use super::tuple::*;

    #[allow(unused_imports)]
    #[cfg(feature = "alloc")]
    pub use super::vec::*;
}
