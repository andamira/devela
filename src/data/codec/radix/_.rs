// devela/src/data/codec/radix/_.rs
//
//! Radix-based encodings.
//

crate::mods_in! {
    #[cfg(test)]
    mod_ _test;

    mod define; // Radix
    mod_ impls;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::Radix,
        };
    }
}
