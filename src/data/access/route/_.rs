// devela/src/data/access/route/_.rs
//
#![doc = crate::_DOC_DATA_ACCESS_ROUTE!()] // public
#![doc = crate::_doc!(modules: crate::data::access; route)]
#![doc = crate::_doc!(flat:"data")]
// #![doc = crate::_QUO_DATA_ACCESS_ROUTE!()]
#![doc = crate::_doc!(hr)]
//!
//! Backend-neutral segmented access routes.
//!
//! A route is ordered reachability without resolver, storage,
//! transport, filesystem, URL, or platform semantics.
//!
//! Routes are structural. They can later be interpreted as filesystem paths,
//! URL paths, command paths, UI routes, asset keys, symbolic addresses,
//! or virtual resource locations.
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
