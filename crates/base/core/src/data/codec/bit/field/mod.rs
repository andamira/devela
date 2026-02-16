// devela_base_core::data::codec::bit::field
//
//!
//

mod bitfield;

#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods
    _mods {
        pub use super::bitfield::*;
    }
}
