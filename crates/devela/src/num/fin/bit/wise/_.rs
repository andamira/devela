//
//! Defines the [`Bitwise`] namespace.
//

crate::mods_in! {
    #[cfg(test)]
    #[cfg(feature = "bitwise")]
    mod _test;

    #[cfg(feature = "bitwise")]
    mod namespace; // Bitwise
    #[cfg(feature = "bitwise")]
    mod impls;

    mod r#macro; // bitwise!
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "bitwise")]
        pub use super::namespace::Bitwise;
        #[doc(inline)]
        pub use super::r#macro::bitwise;
    }
}
