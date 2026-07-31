// devela/src/media/font/format/mod.rs
//
//! Font storage and interchange formats.
//

// mod bdf; // Bdf
mod dvbf; // Dvbf

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            // bdf::_all::*,
            dvbf::_all::*,
        };
    }
    _crate_internals {
        // pub use super::{
        //     bdf::_crate_internals::*,
        // };
    }
}
