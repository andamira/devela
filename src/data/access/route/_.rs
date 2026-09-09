// devela/src/data/access/route/_.rs
//
#![doc = crate::_DOC_DATA_ACCESS_ROUTE!()] // public
#![doc = crate::_doc!(modules: crate::data::access; route)]
#![doc = crate::_doc!(flat:"data")]
// #![doc = crate::_QUO_DATA_ACCESS_ROUTE!()]
#![doc = crate::_doc!(hr)]
//!
//! Routes express ordered reachability as a sequence of uninterpreted segments.
//!
//! A route carries no resolver, storage, transport,
//! filesystem, URL, or platform semantics of its own.
//

crate::mods_in! {
    mod define; // Route, RouteAnchor, RouteName, RouteSeg

    // impls
    // mod parse;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
