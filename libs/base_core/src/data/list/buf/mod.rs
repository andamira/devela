// devela_base_core::data::list::buf
//
//!
//

mod line; // BufLine
// mod ring; // BufRing

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            line::*,
            // ring::*,
        };
    }
}
