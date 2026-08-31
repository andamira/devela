// devela/src/data/codec/bin/bit/set/_.rs

crate::mods_in! {
    #[cfg(any(test, doctest))]
    mod _test;

    mod define; // set!
}
crate::mods_out! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            define::set,
        };
    }
}
