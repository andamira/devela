// devela/src/media/font/format/mod.rs
//
//! Font storage and interchange formats.
//

mod bdf; // Bdf
mod dvbf; // Dvbf

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            bdf::_all::*,
            dvbf::_all::*,
        };
    }
}
