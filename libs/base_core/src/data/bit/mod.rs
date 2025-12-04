// devela_base_core::data::codec::bit
//
//! Bit-focused items.
//

// internals
mod _docs;

mod field; // bitfield
mod ops; // BitOps
// mod view; // BitView // WIP
mod wise; // Bitwise

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            field::_all::*,
            ops::*,
            // view::*,
            wise::*,
        };
    }
}
