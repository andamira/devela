// devela/src/media/visual/image/format/netpbm/_.rs
//
//! The Netpbm family of simple bitmap formats.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod _helper; // (PnmCursor, PnmFormat, PnmHeader)
    mod _impls; // (impl Pnm)

    mod namespace; // Pnm
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::{
            namespace::Pnm,
        };
    }
    _crate_internals {
        pub(crate) use super::_helper::*;
    }
}
