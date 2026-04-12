// devela::data::codec::bit::field
//
//!
//

mod bitfield; // bitfield!

#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods
    _mods {
        pub use super::bitfield::bitfield;
    }
}
