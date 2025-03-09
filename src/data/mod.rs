// devela::data
//
//! Data handling and manipulation.
#![doc = crate::doc_!(modules: crate; data: codec, iter, key, list, table, uid, xipher)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: array, collections, hash, iter, vec)]
//
// safety
#![cfg_attr(feature = "safe_data", forbid(unsafe_code))]

mod absence; // NoData
mod collection;
mod sort;

pub mod codec;
pub mod iter;
pub mod key;
pub mod list;
pub mod table;
pub mod uid;
pub mod xipher; // cryptography

#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
#[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
#[cfg(all(not(any(feature = "safe_data", feature = "safe_mem")), feature = "unsafe_layout"))]
pub mod dst;

crate::items! { // structural access: _mods, _pub_mods, _internals, _all, _always
    #[allow(unused)]
    pub use {_mods::*, _internals::*};
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _mods { #![allow(unused)]
        pub use super::{absence::*, collection::*, sort::_all::*};
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{
            codec::_all::*, iter::_all::*, key::_all::*, list::_all::*,
            table::_all::*, uid::_all::*, xipher::_all::*,
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
        // pub use super::node::*;
    }
    pub(super) mod _internals { #![allow(unused)]
        pub(crate) use super::table::_internals::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_pub_mods::*, _mods::*};
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{
            codec::_always::*, collection::*, iter::_always::*, list::_always::*,
        };
    }
}
// WIPZONE
// mod pool;
// mod view;
// #[cfg(_graph··)]
// pub mod graph;
// #[cfg(_node··)]
// pub mod node;
