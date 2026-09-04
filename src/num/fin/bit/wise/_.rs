// devela/src/num/fin/bit/wise/_.rs
//
//! Defines the [`Bitwise`] namespace.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // Bitwise
    mod impls;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::Bitwise,
        };
    }
}
