// devela/src/sys/mem/size/bit/_.rs
//
//! Functionality related to memory bit size.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // BitSized
    mod impls;
}
crate::mods_out! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            define::*,
        };
    }
}
