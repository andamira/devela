// devela/src/text/ascii/digits/mod.rs
//
//! Defines [`Digits`].
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod _docs; // DOC_*

    mod define; // Digits
    mod u8;
    mod u16;
    mod u32;
    mod u64;
    mod u128;
    mod usize;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::Digits,
        };
    }
    // TODO: self_internals {}
}
use _docs::*; // IMPROVE
