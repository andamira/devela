// devela/src/num/grain/lim/mod.rs
//
//! Bounded numeric carriers and range-preserving arithmetic.
//

#[cfg(test)]
mod _test;

#[cfg(any(test, feature = "_docs_examples"))]
crate::__doc_auto_hide_features! { (("_docs_examples"))
    mod _example;
}

mod define; // bound_int!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::bound_int,
        };

        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
}
