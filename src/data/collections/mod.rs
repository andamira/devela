// devela::data::collections
//
//! Data collections, extends
//! `std::{`[`array`], [`collections`], [`vec`]`}`.
//!
//! [`array`]: mod@std::array
//! [`collections`]: std::collections
//! [`vec`]: std::vec
//

#![allow(unused_imports)]

/* always compiled, non-public modules */

mod array;
mod destaque;
mod reexports;
mod stack;
mod traits;
mod tuple; // ExtTuple, TupleFmt

pub use {array::all::*, destaque::all::*, reexports::*, stack::all::*, traits::*, tuple::*};

/* feature-gated, non-public modules */

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
mod vec;

#[cfg(feature = "alloc")]
pub use vec::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        array::all::*, destaque::all::*, reexports::*, stack::all::*, traits::*, tuple::*,
    };

    // feature-gated
    #[cfg(feature = "alloc")]
    pub use super::vec::*;
}
