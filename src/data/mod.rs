// devela::data
//
#![doc = crate::_DOC_DATA!()] // public, root
#![doc = crate::_DOC_DATA_MODULES!()]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: array, collections, hash, iter, vec)]
//
// safety
#![cfg_attr(feature = "safe_data", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_DATA_MODULES =
    crate::_doc!(modules: crate; data: codec, error, iter, key, list, uid, value); // table
    // address
}

mod absence; // NoData
mod bit;
mod collection; // DataCollection

pub mod codec;
pub mod error {
    #![doc = crate::_DOC_DATA_ERROR!()] // public
    #![doc = crate::_doc!(modules: crate::data; error)]
    #![doc = crate::_doc!(flat:"data")]
    #![doc = crate::_doc!(hr)]
    #[doc(inline)]
    pub use devela_base_core::data::error::*;
}
pub mod iter;
pub mod key;
pub mod list;
// pub mod table; // TODO
pub mod uid;
pub mod value; // TODO

#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
#[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
#[cfg(all(not(any(feature = "safe_data", feature = "safe_mem")), feature = "unsafe_layout"))]
pub mod dst;

// WIPZONE
// mod _wip_pool;
// mod _wip_view;
// #[cfg(_graph··)]
// pub mod graph;
// #[cfg(_node··)]
// pub mod node;

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            absence::*,
            bit::_all::*,
            collection::*,
        };

        #[doc(inline)]
        pub use devela_base_core::data::{Sort, define_handle};
        #[cfg(feature = "_docs_examples")]
        pub use devela_base_core::data::HandleExample;
        // #[doc(inline)]
        // #[cfg(feature = "alloc")]
        // pub use devela_base_alloc::SortAlloc;
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            codec::_all::*,
            error::*,
            iter::_all::*,
            key::_all::*,
            list::_all::*,
            // table::_all::*, // TODO
            uid::_all::*,
            value::_all::*, // TODO
        };

        #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
        #[cfg(all(
            not(any(feature = "safe_data", feature = "safe_mem")),
            feature = "unsafe_layout"
        ))]
        pub use super::dst::_all::*;

        // WIPZONE
        // pub use super::_wip_pool::*;
        // pub use super::_wip_view::*;
        // #[cfg(_graph··)]
        // pub use super::graph::*;
        // #[cfg(_node··)]
        // pub use super::node::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_DATA_MODULES;
        pub(crate) use super::value::_crate_internals::*;
        #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
        #[cfg(all(
            not(any(feature = "safe_data", feature = "safe_mem")),
            feature = "unsafe_layout"
        ))]
        pub(crate) use super::dst::_crate_internals::*;
    }
}
