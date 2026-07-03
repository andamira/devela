// devela/src/data/access/iter/lending/mod.rs
//
//!
//

#[cfg(test)]
mod _test;

mod define; // IteratorLending[DoubleEnded|ExactSize|Peek]

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            define::*,
        };
    }
}
