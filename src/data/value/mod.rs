// devela/src/data/value/mod.rs
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

mod absence; // NoData
mod intro; // Introspect
mod of; // Oneof

mod kind; // ValueKind, ValueKind4, WIP ValueKindSet
// mod profile; // TODO ValueProfile
// mod schema; // WIP Schemas for encoded data structures.
mod value; // Value*
#[cfg(feature = "_tuple")]
pub mod tuple; // Tuple, TupleFmt, TupleEnumRef, TupleEnumMut

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            absence::NoData,
            of::_all::*,
            kind::*,
            intro::_all::*,
            // profile::*,
            // schema::_all::*,
            value::_all::*,
        };
    }
    _pub_mods {
        #[cfg(feature = "_tuple")]
        pub use super::tuple::_all::*;
    }
}
