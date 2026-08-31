// devela/src/data/value/_.rs
//
#![doc = crate::_DOC_DATA_VALUE!()] // public
#![doc = crate::_doc!(modules: crate::data; value: tuple)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Values describe the semantic forms carried by data independently of
//! storage, identity, and canonical raw representation.
//!
//! [`ValueKind`] classifies broad semantic categories, while [`ValueKind4`]
//! provides its compact universal band. [`NoData`] represents absence and
//! [`Oneof`] represents a fixed choice among alternatives.
//!
//! Exact canonical representation belongs to [`data::word`][mod@crate::data::word],
//! arrangement and retention belong to [`data::layout`][crate::data::layout]
//! and [`data::store`][crate::data::store].
//

crate::mods_in! {
    mod absence; // NoData
    mod_ intro; // Introspect
    mod_ of; // Oneof

    mod_ kind; // ValueKind, ValueKind4, WIP ValueKindSet
    // mod profile; // TODO ValueProfile
    // mod_ schema; // WIP Schemas for encoded data structures.
    #[cfg(feature = "_tuple")]
    pub mod_ tuple; // Tuple, TupleFmt, TupleEnumRef, TupleEnumMut
    mod_ value_; // Value*
}

crate::mods_out! { // _mods, _pub_mods
    _mods {
        pub use super::{
            absence::NoData,
            of::_all::*,
            kind::_all::*,
            intro::_all::*,
            // profile::*,
            // schema::_all::*,
            value_::_all::*,
        };
    }
    _pub_mods {
        #[cfg(feature = "_tuple")]
        pub use super::tuple::_all::*;
    }
}
