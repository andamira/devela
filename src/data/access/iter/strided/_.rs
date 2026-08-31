// devela/src/data/access/iter/strided/_.rs
//
//!
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod canonical; // StridedIter, StridedIterMut
    mod define; // iter_strided!
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::{
            canonical::{StridedIter, StridedIterMut},
            define::iter_strided,
        };
    }
}
