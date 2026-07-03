// devela/src/media/visual/image/format/netpbm/mod.rs
//
//! The Netpbm family of simple bitmap formats.
//

#[cfg(test)]
mod _test;

mod _helper; // (PnmCursor, PnmFormat, PnmHeader)
mod _impls; // (impl Pnm)

mod namespace; // Pnm

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            namespace::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_helper::*;
    }
}
