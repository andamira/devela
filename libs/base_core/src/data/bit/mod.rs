// devela_base_core::data::codec::bit
//
//! Bit-focused items.
//

mod field; // bitfield
mod wrapper; // Bitwise

crate::structural_mods! { // _mods
    _mods {
        pub use super::{field::_all::*, wrapper::*};
    }
}
