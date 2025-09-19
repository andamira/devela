// devela::data
//
#![doc = crate::_DOC_DATA!()]
#![doc = crate::_doc!(modules: crate; data: codec, iter, key, list, table, uid)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: array, collections, hash, iter, vec)]
//
// safety
#![cfg_attr(feature = "safe_data", forbid(unsafe_code))]

mod absence; // NoData
mod bit; // bitfield handling and binary transformations.

mod collection;
pub mod codec;
pub mod errors {
    //! Data-related errors.
    #[doc(inline)]
    pub use devela_base_core::data::errors::*;
}
pub mod iter;
pub mod key;
pub mod list;
pub mod table;
pub mod uid;

#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
#[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
#[cfg(all(not(any(feature = "safe_data", feature = "safe_mem")), feature = "unsafe_layout"))]
pub mod dst;

// WIPZONE
// mod pool;
// mod view;
// #[cfg(_graph··)]
// pub mod graph;
// #[cfg(_node··)]
// pub mod node;

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{absence::*, bit::_all::*, collection::*};

        #[doc(inline)]
        pub use devela_base_core::Sort;
        // #[doc(inline)]
        // #[cfg(feature = "alloc")]
        // pub use devela_base_alloc::SortAlloc;
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            codec::_all::*, errors::*, iter::_all::*, key::_all::*,
            list::_all::*, table::_all::*, uid::_all::*,
        };

        #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
        #[cfg(all(
            not(any(feature = "safe_data", feature = "safe_mem")),
            feature = "unsafe_layout"
        ))]
        pub use super::dst::_all::*;

        // WIPZONE
        // pub use super::pool::*;
        // pub use super::view::*;
        // #[cfg(_graph··)]
        // pub use super::graph::*;
        // #[cfg(_node··)]
        // pub use super::node::_all::*;
    }
    _crate_internals {
        pub(crate) use super::table::_crate_internals::*;
        #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
        #[cfg(all(
            not(any(feature = "safe_data", feature = "safe_mem")),
            feature = "unsafe_layout"
        ))]
        pub(crate) use super::dst::_crate_internals::*;
    }
}
