// devela/src/data/access/iter/lending/mod.rs
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
