// devela/src/data/codec/bin/bit/set/mod.rs

#[cfg(any(test, doctest))]
mod _test;

mod define; // set!

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            define::set,
        };
    }
}
