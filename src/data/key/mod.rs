// devela::data::key
//
#![doc = crate::_DOC_DATA_KEY!()]
//!
//! Provides tools for mapping, indexing, and efficiently organizing data
//! through unique keys, including maps, sets, and other key-value structures.
//!
#![doc = crate::_doc!(extends: collections)]
//

mod reexports;
mod static_map; // define_static_map!

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods { #![allow(unused)]
        pub use super::reexports::*;
        pub use super::static_map::*;
        // WIPZONE
        // pub use set::*;
        // pub use trie::*;
    }
    mod _pub_mods { #![allow(unused)]
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
// WIPZONE
// mod set;
// mod trie;
