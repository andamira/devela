// devela/src/data/access/iter/lending/_.rs
//
//!
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // IteratorLending[DoubleEnded|ExactSize|Peek]
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            define::*,
        };
    }
}
