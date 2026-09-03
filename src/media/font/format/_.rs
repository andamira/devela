// devela/src/media/font/format/_.rs
//
//! Font storage and interchange formats.
//

crate::mods_in! {
    mod_ bdf; // Bdf
    mod_ dvbf; // Dvbf
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            bdf::_all::*,
            dvbf::_all::*,
        };
    }
}
