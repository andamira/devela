// devela_base_core::data::access::iter::lending
//
//!
//

#[cfg(test)]
mod tests;

mod definitions; // IteratorLending[DoubleEnded|ExactSize|Peek]

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            definitions::*,
        };
    }
}
