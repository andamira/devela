// devela/src/data/access/iter/strided/mod.rs
//
//!
//

#[cfg(test)]
mod _test;

mod canonical; // StridedIter, StridedIterMut
mod define; // strided_iter!

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            canonical::*,
            define::*,
        };
    }
}
