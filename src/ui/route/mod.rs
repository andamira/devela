// devela/src/ui/route/mod.rs
//
#![doc = crate::_DOC_UI_ROUTE!()] // public
#![doc = crate::_doc!(modules: crate::ui; route)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod _test;

mod hit; // HitRegion
mod state; // RouteActive, RouteCapture, RouteFocus, RouteHot

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            hit::*,
            state::*,
        };
    }
}
