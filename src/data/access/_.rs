// devela/src/data/access/_.rs
//
#![doc = crate::_DOC_DATA_ACCESS!()] // public
#![doc = crate::_doc!(modules: crate::data; access: iter, route)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Access concerns how existing data is reached and traversed.
//!
//! An access mechanism may retain a current position, expose successive values,
//! describe a route, or transfer through caller-provided storage.
//!
//! Identity, physical arrangement, and retention belong respectively to
//! [`data::id`][crate::data::id], [`data::layout`][crate::data::layout],
//! and [`data::store`][crate::data::store].
//

crate::mods_in! {
    // mod address; // Symbolic and contextual references interpreted through resolution TODO
    mod_ cursor; // Retained positional access over ordered data
    pub mod_ iter; // Composable external and lending traversal
    mod_ offset; // Explicit positional access: read_at!, write_at!
    pub mod_ route; // Segmented routes before domain-specific interpretation
    mod transfer; // Caller-buffered gather/scatter transfers
}
crate::mods_out! { // _mods, _pub_mods, _reexports
    _mods {
        pub use super::{
            // address::_all::*,
            cursor::_all::*,
            offset::_all::*,
            transfer::StridedBlocks,
        };
    }
    _pub_mods {
        pub use super::{
            iter::_all::*,
            route::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            iter::IteratorLending,
            route::Route,
        };
    }
}
