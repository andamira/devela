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
        // #[cfg(feature = "bitwise")]
        pub use super::define::Bitwise;
    }
}
