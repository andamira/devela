// devela::data::collections
//
//! Data collections, extends
//! `std::{`[`array`], [`collections`], [`vec`]`}`.
//!
//! [`array`]: mod@std::array
//! [`collections`]: std::collections
//! [`vec`]: std::vec
//

/* always compiled */

mod array;
mod destaque;
mod reexports;
mod traits;
mod tuple; // Tuple, TupleFmt
#[allow(unused_imports)]
pub use {array::all::*, destaque::all::*, reexports::*, traits::*, tuple::*};

/* feature-gated */

#[cfg(feature = "_-stack_any-_")]
mod stack;
#[cfg(feature = "_-stack_any-_")]
pub use stack::*;

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
mod vec;
#[allow(unused_imports)]
#[cfg(feature = "alloc")]
pub use vec::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{array::all::*, destaque::all::*, reexports::*, traits::*, tuple::*};

    // feature-gated
    #[cfg(feature = "_-stack_any-_")]
    pub use super::stack::*;
    #[allow(unused_imports)]
    #[cfg(feature = "alloc")]
    pub use super::vec::*;
}
