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
crate::mods_out! { // _mods, _internals
    _mods {
        #[cfg(feature = "bitwise")]
        pub use super::namespace::Bitwise;
        #[doc(inline)]
        pub use super::r#macro::bitwise;
    }
    _hidden {
        #[doc(hidden)]
        pub use super::r#macro::{
            __bitwise, __bitwise_backend, __bitwise_inline, __bitwise_methods,
        };
    }
}
