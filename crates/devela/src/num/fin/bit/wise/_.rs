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
}
crate::mods_out! { // _mods, _hidden
    _mods {
        #[cfg(feature = "bitwise")]
        pub use super::namespace::Bitwise;
        #[doc(inline)]
        pub use crate::boot::base::num::bitwise;
    }
    _hidden {
        #[doc(hidden)]
        pub use crate::boot::base::num::{
            __bitwise, __bitwise_backend, __bitwise_inline, __bitwise_methods,
        };
    }
}
