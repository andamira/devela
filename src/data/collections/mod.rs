// devela::data::collections
//
//! Data collections, extends
//! `std::{`[`array`], [`collections`], [`vec`]`}`.
//!
//! [`array`]: mod@std::array
//! [`collections`]: std::collections
//! [`vec`]: mod@std::vec
//

mod array;
mod reexports;
mod traits;
#[allow(unused_imports)]
pub use {array::all::*, reexports::*, traits::*};

#[cfg(_some_destaque)]
mod destaque;
#[cfg(_some_destaque)]
pub use destaque::*;

#[cfg(_some_node)]
mod node;
#[cfg(_some_node)]
pub use node::*;

#[cfg(_some_stack)]
mod stack;
#[cfg(_some_stack)]
pub use stack::*;

#[cfg(feature = "_tuple")]
mod tuple; // Tuple, TupleFmt
#[cfg(feature = "_tuple")]
pub use tuple::*;

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
mod vec;
#[allow(unused_imports)]
#[cfg(feature = "alloc")]
pub use vec::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{array::all::*, reexports::*, traits::*};

    #[cfg(_some_destaque)]
    pub use super::destaque::all::*;

    #[cfg(_some_node)]
    pub use super::node::*;

    #[cfg(_some_stack)]
    pub use super::stack::all::*;

    #[cfg(feature = "_tuple")]
    pub use super::tuple::*;

    #[allow(unused_imports)]
    #[cfg(feature = "alloc")]
    pub use super::vec::*;
}
